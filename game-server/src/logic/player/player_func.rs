use std::collections::HashMap;

use shorekeeper_data::function_condition_data;
use shorekeeper_protocol::{FuncOpenNotify, Function, PlayerFuncData};

pub struct PlayerFunc {
    pub func_map: HashMap<i32, i32>,
}

impl PlayerFunc {
    pub fn load_from_save(data: PlayerFuncData) -> Self {
        PlayerFunc {
            func_map: data.func_map,
        }
    }

    pub fn build_save_data(&self) -> PlayerFuncData {
        PlayerFuncData {
            func_map: self.func_map.clone(),
        }
    }

    pub fn build_func_open_notify(&self) -> FuncOpenNotify {
        FuncOpenNotify {
            func: self
                .func_map
                .iter()
                .map(|(id, flag)| Function {
                    id: *id,
                    flag: *flag,
                })
                .collect(),
        }
    }
}

impl Default for PlayerFunc {
    fn default() -> Self {
        Self {
            func_map: function_condition_data::iter()
                .filter(|fc| fc.open_condition_id == 0 && fc.is_on)
                .map(|fc| (fc.function_id, 2))
                .collect(),
        }
    }
}
