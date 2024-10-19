use std::{cell::RefCell, collections::HashSet, rc::Rc, sync::Arc};

use basic_info::PlayerBasicInfo;
use common::time_util;
use explore_tools::ExploreTools;
use location::PlayerLocation;
use player_func::PlayerFunc;
use shorekeeper_protocol::{
    message::Message, ItemPkgOpenNotify, PbGetRoleListNotify, PlayerBasicData, PlayerRoleData,
    PlayerSaveData, ProtocolUnit,
};

use crate::session::Session;

use super::{
    ecs::world::World,
    role::{Role, RoleFormation},
};

mod basic_info;
mod explore_tools;
mod in_world_player;
mod location;
mod player_func;

pub use in_world_player::InWorldPlayer;

pub struct Player {
    session: Option<Arc<Session>>,
    // Persistent
    pub basic_info: PlayerBasicInfo,
    pub role_list: Vec<Role>,
    pub formation_list: Vec<RoleFormation>,
    pub location: PlayerLocation,
    pub func: PlayerFunc,
    pub explore_tools: ExploreTools,
    // Runtime
    pub world: Rc<RefCell<World>>,
    pub last_save_time: u64,
}

impl Player {
    pub fn init(&mut self) {
        if self.role_list.is_empty() {
            self.on_first_enter();
        }

        // we need shorekeeper
        // TODO: remove this part after implementing team switch
        if !self.role_list.iter().any(|r| r.role_id == 1603) {
            let mut camellya = Role::new(1603);
            camellya.equip_weapon = 21020026;
            self.role_list.push(camellya);
        }

        self.formation_list.clear();
        self.formation_list.push(RoleFormation {
            id: 1,
            cur_role: 1603,
            role_id_set: HashSet::from([1603]),
            is_current: true,
        });
        // End shorekeeper hardcode part

        self.ensure_current_formation();
        self.ensure_basic_unlock_func();
    }

    pub fn notify_general_data(&self) {
        self.notify(self.basic_info.build_notify());
        self.notify(self.func.build_func_open_notify());
        self.notify(self.build_role_list_notify());
        self.notify(self.explore_tools.build_explore_tool_all_notify());
        self.notify(self.explore_tools.build_roulette_update_notify());

        self.notify(ItemPkgOpenNotify {
            open_pkg: (0..8).collect(),
        });
    }

    fn on_first_enter(&mut self) {
        self.role_list.push(Self::create_main_character_role(
            self.basic_info.name.clone(),
            self.basic_info.sex,
        ));

        let role = &self.role_list[0];

        self.formation_list.push(RoleFormation {
            id: 1,
            cur_role: role.role_id,
            role_id_set: HashSet::from([role.role_id]),
            is_current: true,
        });

        self.location = PlayerLocation::default();
    }

    // Ensure basic functionality is unlocked
    // Should be handled by quest progression,
    // but as of right now, just unlock what we need
    fn ensure_basic_unlock_func(&mut self) {
        self.func.unlock(10026); // explore tools
    }

    fn ensure_current_formation(&mut self) {
        if self.formation_list.is_empty() {
            let role = &self.role_list[0];

            self.formation_list.push(RoleFormation {
                id: 1,
                cur_role: role.role_id,
                role_id_set: HashSet::from([role.role_id]),
                is_current: true,
            });
        }

        if !self.formation_list.iter().any(|rf| rf.is_current) {
            self.formation_list[0].is_current = true;
        }

        if let Some(rf) = self.formation_list.iter_mut().find(|rf| rf.is_current) {
            if rf.role_id_set.is_empty() {
                rf.role_id_set.insert(self.role_list[0].role_id);
            }

            if !rf.role_id_set.contains(&rf.cur_role) {
                rf.cur_role = *rf.role_id_set.iter().nth(0).unwrap();
            }
        }
    }

    pub fn build_in_world_player(&self) -> InWorldPlayer {
        InWorldPlayer {
            player_id: self.basic_info.id,
            player_name: self.basic_info.name.clone(),
            player_icon: 0,
            level: self.basic_info.level,
            group_type: 1,
        }
    }

    pub fn get_current_formation_role_list(&self) -> Vec<&Role> {
        self.formation_list
            .iter()
            .find(|rf| rf.is_current)
            .unwrap()
            .role_id_set
            .iter()
            .flat_map(|id| self.role_list.iter().find(|r| r.role_id == *id))
            .collect()
    }

    pub fn get_cur_role_id(&self) -> i32 {
        self.formation_list
            .iter()
            .find(|rf| rf.is_current)
            .unwrap()
            .cur_role
    }

    pub fn load_from_save(save_data: PlayerSaveData) -> Self {
        let role_data = save_data.role_data.unwrap_or_default();

        Self {
            session: None,
            basic_info: PlayerBasicInfo::load_from_save(save_data.basic_data.unwrap_or_default()),
            role_list: role_data
                .role_list
                .into_iter()
                .map(Role::load_from_save)
                .collect(),
            formation_list: role_data
                .role_formation_list
                .into_iter()
                .map(RoleFormation::load_from_save)
                .collect(),
            location: save_data
                .location_data
                .map(PlayerLocation::load_from_save)
                .unwrap_or_default(),
            func: save_data
                .func_data
                .map(PlayerFunc::load_from_save)
                .unwrap_or_default(),
            explore_tools: save_data
                .explore_tools_data
                .map(ExploreTools::load_from_save)
                .unwrap_or_default(),
            world: Rc::new(RefCell::new(World::new())),
            last_save_time: time_util::unix_timestamp(),
        }
    }

    pub fn build_save_data(&self) -> PlayerSaveData {
        PlayerSaveData {
            basic_data: Some(self.basic_info.build_save_data()),
            role_data: Some(PlayerRoleData {
                role_list: self.role_list.iter().map(|r| r.build_save_data()).collect(),
                role_formation_list: self
                    .formation_list
                    .iter()
                    .map(|rf| rf.build_save_data())
                    .collect(),
            }),
            location_data: Some(self.location.build_save_data()),
            func_data: Some(self.func.build_save_data()),
            explore_tools_data: Some(self.explore_tools.build_save_data()),
        }
    }

    pub fn set_session(&mut self, session: Arc<Session>) {
        self.session = Some(session);
    }

    pub fn build_role_list_notify(&self) -> PbGetRoleListNotify {
        PbGetRoleListNotify {
            role_list: self.role_list.iter().map(|r| r.to_protobuf()).collect(),
        }
    }

    pub fn notify(&self, content: impl ProtocolUnit) {
        if let Some(session) = self.session.as_ref() {
            session.forward_to_gateway(Message::Push {
                sequence_number: 0,
                message_id: content.get_message_id(),
                payload: Some(content.encode_to_vec().into_boxed_slice()),
            });
        }
    }

    pub fn respond(&self, content: impl ProtocolUnit, rpc_id: u16) {
        if let Some(session) = self.session.as_ref() {
            session.forward_to_gateway(Message::Response {
                sequence_number: 0,
                message_id: content.get_message_id(),
                rpc_id,
                payload: Some(content.encode_to_vec().into_boxed_slice()),
            });
        }
    }

    fn create_main_character_role(name: String, sex: i32) -> Role {
        let mut role = match sex {
            0 => Role::new(Role::MAIN_CHARACTER_FEMALE_ID),
            1 => Role::new(Role::MAIN_CHARACTER_MALE_ID),
            _ => unreachable!(),
        };

        role.name = name;
        role
    }

    pub fn create_default_save_data(id: i32, name: String, sex: i32) -> PlayerSaveData {
        PlayerSaveData {
            basic_data: Some(PlayerBasicData {
                id,
                name,
                sex,
                level: 1,
                head_photo: 1603,
                head_frame: 80060009,
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
