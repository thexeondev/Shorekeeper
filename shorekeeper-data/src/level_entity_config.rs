use serde::Deserialize;
use crate::RawVectorData;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LevelEntityConfigData {
    pub id: i32,
    pub map_id: i32,
    pub entity_id: i64,
    pub blueprint_type: String,
    pub name: String,
    pub in_sleep: bool,
    pub is_hidden: bool,
    pub area_id: i32,
    pub transform: Vec<RawVectorData>,
    // Schemaless property, any suggestions @xeondev??
    pub components_data: serde_json::Value,
}
