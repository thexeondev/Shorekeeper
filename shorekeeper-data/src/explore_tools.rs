use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExploreToolsData {
    pub phantom_skill_id: i32,
    pub skill_type: i32,
    pub sort_id: i32,
    pub auto_fill: bool,
    pub show_unlock: bool,
    pub skill_group_id: i32,
    pub is_use_in_phantom_team: bool,
    pub summon_config_id: i32,
    pub authorization: HashMap<i32, i32>,
}
