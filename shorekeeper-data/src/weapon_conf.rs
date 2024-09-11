use serde::Deserialize;

use crate::PropValueData;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WeaponConfData {
    pub item_id: i32,
    pub weapon_name: String,
    pub quality_id: i32,
    pub model_id: i32,
    pub transform_id: i32,
    pub models: Vec<i32>,
    pub reson_level_limit: i32,
    pub first_prop_id: PropValueData,
    pub first_curve: i32,
    pub second_prop_id: PropValueData,
    pub second_curve: i32,
    pub reson_id: i32,
    pub level_id: i32,
    pub breach_id: i32,
    #[serde(rename = "MaxCapcity")] // kuro!
    pub max_capacity: i32,
    pub destructible: bool,
}
