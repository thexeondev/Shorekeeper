use shorekeeper_data::instance_dungeon_data;
use shorekeeper_protocol::PlayerLocationData;

use crate::logic::math::{Transform, Vector3f};

pub struct PlayerLocation {
    pub instance_id: i32,
    pub position: Transform,
}

impl PlayerLocation {
    const DEFAULT_INSTANCE_ID: i32 = 8;

    pub fn load_from_save(data: PlayerLocationData) -> Self {
        Self {
            instance_id: data.instance_id,
            position: Transform::load_from_save(data.position.unwrap_or_default()),
        }
    }

    pub fn build_save_data(&self) -> PlayerLocationData {
        PlayerLocationData {
            instance_id: self.instance_id,
            position: Some(self.position.build_save_data()),
        }
    }
}

impl Default for PlayerLocation {
    fn default() -> Self {
        let inst_data = instance_dungeon_data::iter()
            .find(|d| d.id == Self::DEFAULT_INSTANCE_ID)
            .unwrap();

        Self {
            instance_id: inst_data.id,
            position: Transform {
                position: Vector3f::from_data(&inst_data.born_position),
                rotation: Vector3f::from_data(&inst_data.born_rotation),
            },
        }
    }
}
