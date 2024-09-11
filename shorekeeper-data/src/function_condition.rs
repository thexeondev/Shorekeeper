use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FunctionConditionData {
    pub function_id: i32,
    pub name: String,
    pub is_on: bool,
    pub open_condition_id: i32,
}
