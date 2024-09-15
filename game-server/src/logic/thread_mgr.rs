use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc, Arc, OnceLock,
    },
    thread,
    time::Duration,
};

use common::time_util;
use shorekeeper_protocol::{message::Message, JoinSceneNotify, TransitionOptionPb,
                           AfterJoinSceneNotify, EnterGameResponse, JsPatchNotify};
use shorekeeper_protocol::{PlayerSaveData};

use crate::{
    player_save_task::{self, PlayerSaveReason},
    session::Session,
};

use super::{ecs::world::World, player::Player, utils::world_util};

const WATER_MASK: &str = include_str!("../../watermask.js");
const UID_FIX: &str = include_str!("../../uidfix.js");
const CENSORSHIP_FIX: &str = include_str!("../../censorshipfix.js");

pub enum LogicInput {
    AddPlayer {
        player_id: i32,
        enter_rpc_id: u16,
        session: Arc<Session>,
        player_save_data: PlayerSaveData,
    },
    RemovePlayer {
        player_id: i32,
    },
    ProcessMessage {
        player_id: i32,
        message: Message,
    },
}

#[derive(Clone)]
pub struct LogicThreadHandle {
    sender: mpsc::Sender<LogicInput>,
    load: Arc<AtomicUsize>,
}

static THREAD_HANDLES: OnceLock<Box<[LogicThreadHandle]>> = OnceLock::new();

pub fn start_logic_threads(num_threads: usize) {
    if THREAD_HANDLES.get().is_some() {
        tracing::error!("start_logic_threads: logic threads are already running!");
        return;
    }

    let _ = THREAD_HANDLES.set(
        (0..num_threads)
            .map(|_| {
                let (tx, rx) = mpsc::channel();
                let load = Arc::new(AtomicUsize::new(0));

                let handle = LogicThreadHandle {
                    sender: tx,
                    load: load.clone(),
                };

                thread::spawn(move || logic_thread_func(rx, load));
                handle
            })
            .collect(),
    );
}

// Thread-local logic state
struct LogicState {
    thread_load: Arc<AtomicUsize>, // shared parameter for load-balancing
    worlds: HashMap<i32, Rc<RefCell<World>>>, // owner_id - world
    players: HashMap<i32, RefCell<Player>>, // id - player
}

fn logic_thread_func(receiver: mpsc::Receiver<LogicInput>, load: Arc<AtomicUsize>) {
    const RECV_TIMEOUT: Duration = Duration::from_millis(50);
    const PLAYER_SAVE_PERIOD: u64 = 30;

    let mut state = LogicState {
        thread_load: load,
        worlds: HashMap::new(),
        players: HashMap::new(),
    };

    let mut input_queue = VecDeque::with_capacity(32);

    loop {
        if let Ok(input) = receiver.recv_timeout(RECV_TIMEOUT) {
            input_queue.push_back(input);

            while let Ok(input) = receiver.try_recv() {
                input_queue.push_back(input);
            }
        }

        while let Some(input) = input_queue.pop_front() {
            handle_logic_input(&mut state, input);
        }

        state.worlds.values().for_each(|world| {
            let mut world = world.borrow_mut();
            let mut players = world
                .player_ids()
                .flat_map(|id| state.players.get(id).map(|pl| pl.borrow_mut()))
                .collect::<Box<_>>();

            super::systems::tick_systems(&mut world, &mut players);
        });

        state.players.values().for_each(|player| {
            let mut player = player.borrow_mut();
            if time_util::unix_timestamp() - player.last_save_time > PLAYER_SAVE_PERIOD {
                player_save_task::push(
                    player.basic_info.id,
                    player.build_save_data(),
                    PlayerSaveReason::PeriodicalSave,
                );

                player.last_save_time = time_util::unix_timestamp();
            }
        })
    }
}

fn handle_logic_input(state: &mut LogicState, input: LogicInput) {
    match input {
        LogicInput::AddPlayer {
            player_id,
            enter_rpc_id,
            session,
            player_save_data,
        } => {
            let player = state
                .players
                .entry(player_id)
                .or_insert(RefCell::new(Player::load_from_save(player_save_data)));

            let mut player = player.borrow_mut();
            state.worlds.insert(player_id, player.world.clone());

            player.init();
            player.set_session(session);
            player.notify_general_data();

            player
                .world
                .borrow_mut()
                .set_in_world_player_data(player.build_in_world_player());

            world_util::add_player_entities(&mut player.world.borrow_mut(), &player);
            let scene_info = world_util::build_scene_information(
                &player.world.borrow(),
                player.location.instance_id,
                player.basic_info.id,
            );

            player.notify(JoinSceneNotify {
                max_entity_id: i64::MAX,
                scene_info: Some(scene_info),
                transition_option: Some(TransitionOptionPb::default()),
            });
            player.notify(JsPatchNotify {
                content: WATER_MASK.to_string(),
            });
            player.notify(JsPatchNotify {
                content: UID_FIX
                    .replace("{PLAYER_USERNAME}", &player.basic_info.name)
                    .replace("{SELECTED_COLOR}", "50FC71"),
            });
            player.notify(JsPatchNotify {
                content: CENSORSHIP_FIX.to_string()
            });

            player.respond(EnterGameResponse::default(), enter_rpc_id);
            player.notify(AfterJoinSceneNotify::default());
            drop(player);

            state
                .thread_load
                .store(state.players.len(), Ordering::Relaxed);
        }
        LogicInput::ProcessMessage { player_id, message } => {
            let Some(player) = state.players.get_mut(&player_id) else {
                tracing::warn!("logic_thread: process message requested, but player with id {player_id} doesn't exist");
                return;
            };

            super::handler::handle_logic_message(&mut player.borrow_mut(), message);
        }
        LogicInput::RemovePlayer { player_id } => {
            let Some(player) = state.players.remove(&player_id) else {
                tracing::warn!(
                    "logic_thread: player remove requested, but it doesn't exist (id: {player_id})"
                );
                return;
            };

            let _ = state.worlds.remove(&player_id);
            // TODO: kick co-op players from removed world

            player_save_task::push(
                player_id,
                player.borrow().build_save_data(),
                PlayerSaveReason::PlayerLogicStopped,
            );
        }
    }
}

impl LogicThreadHandle {
    pub fn input(&self, input: LogicInput) {
        let _ = self.sender.send(input);
    }
}

pub fn get_least_loaded_thread() -> LogicThreadHandle {
    let handles = THREAD_HANDLES.get().unwrap();
    handles
        .iter()
        .min_by_key(|h| h.load.load(Ordering::Relaxed))
        .unwrap()
        .clone()
}
