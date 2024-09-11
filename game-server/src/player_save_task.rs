use std::sync::{Arc, OnceLock};
use tokio::sync::mpsc;

use shorekeeper_database::{query, PgPool};
use shorekeeper_protocol::{PlayerSaveData, Protobuf};

static SENDER: OnceLock<mpsc::Sender<PlayerSaveQuery>> = OnceLock::new();

#[derive(Debug)]
pub enum PlayerSaveReason {
    PeriodicalSave,
    PlayerLogicStopped,
}

pub fn start(db: Arc<PgPool>) {
    let _ = SENDER.get_or_init(|| {
        let (tx, rx) = mpsc::channel(32);
        tokio::spawn(async move { task_loop(rx, db).await });

        tx
    });
}

pub fn push(player_id: i32, save_data: PlayerSaveData, reason: PlayerSaveReason) {
    tracing::debug!(
        "player_save_task: requesting save for player with id {player_id}, reason: {reason:?}"
    );

    let _ = SENDER.get().unwrap().blocking_send(PlayerSaveQuery {
        player_id,
        save_data,
    });
}

struct PlayerSaveQuery {
    pub player_id: i32,
    pub save_data: PlayerSaveData,
}

async fn task_loop(mut receiver: mpsc::Receiver<PlayerSaveQuery>, db: Arc<PgPool>) {
    loop {
        let Some(save_query) = receiver.recv().await else {
            tracing::warn!("player_save_task: channel was closed, exitting");
            return;
        };

        let bin_data = save_query.save_data.encode_to_vec();

        let _ = query("UPDATE t_player_data SET bin_data = ($1) WHERE player_id = ($2)")
            .bind(bin_data)
            .bind(save_query.player_id)
            .execute(db.as_ref())
            .await
            .inspect_err(|err| {
                tracing::error!(
                    "player_save_task: failed to save data for player_id: {}, err: {err}",
                    save_query.player_id
                )
            });
    }
}
