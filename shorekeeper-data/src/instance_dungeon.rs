use std::collections::HashMap;

use serde::Deserialize;

use crate::{EntranceEntityData, VectorData};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InstanceDungeonData {
    pub id: i32,
    pub map_config_id: i32,
    pub map_name: String,
    pub inst_type: i32,
    pub inst_sub_type: i32,
    pub custom_types: Vec<i32>,
    pub mini_map_id: i32,
    pub sub_levels: Vec<String>,
    pub fight_formation_id: i32,
    pub trial_role_info: Vec<i32>,
    pub revive_id: i32,
    pub born_position: VectorData,
    pub born_rotation: VectorData,
    pub recover_world_location: Vec<i32>,
    pub entrance_entities: Vec<EntranceEntityData>,
    pub exit_entities: Vec<i32>,
    pub first_reward_id: i32,
    pub reward_id: i32,
    pub repeat_reward_id: i32,
    pub enter_control_id: i32,
    pub enter_condition: Vec<i32>,
    pub entity_level: i32,
    pub recommend_level: HashMap<i32, i32>,
    pub recommend_role: Vec<i32>,
    pub recommend_element: Vec<i32>,
    pub share_attri: i32,
    pub can_use_item: i32,
    pub guide_type: i32,
    pub guide_value: i32,
    pub settle_button_type: i32,
    pub auto_leave_time: i32,
    pub limit_time: i32,
    pub leave_wait_time: i32,
    pub verify_creature_gen: bool,
    pub enter_count: i32,
    pub enter_condition_group: i32,
    pub drop_vision_limit: i32,
}
