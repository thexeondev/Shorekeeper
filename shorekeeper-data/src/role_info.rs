use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RoleInfoData {
    pub id: i32,
    pub quality_id: i32,
    pub role_type: i32,
    pub is_trial: bool,
    pub name: String,
    pub nick_name: String,
    pub introduction: String,
    pub tag: Vec<i32>,
    pub parent_id: i32,
    pub priority: i32,
    pub show_property: Vec<i32>,
    pub element_id: i32,
    pub spillover_item: HashMap<i32, i32>,
    pub breach_model: i32,
    pub special_energy_bar_id: i32,
    pub entity_property: i32,
    pub max_level: i32,
    pub level_consume_id: i32,
    pub breach_id: i32,
    pub skill_id: i32,
    pub skill_tree_group_id: i32,
    pub resonance_id: i32,
    pub resonant_chain_group_id: i32,
    pub is_show: bool,
    pub exchange_consume: HashMap<i32, i32>,
    pub init_weapon_item_id: i32,
    pub weapon_type: i32,
    pub party_id: i32,
    pub item_quality_id: i32,
    pub num_limit: i32,
    pub trial_role: i32,
    pub is_aim: bool,
    pub role_guide: i32,
}
