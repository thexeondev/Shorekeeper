use paste::paste;

pub use misc_data::*;

mod misc_data;
#[derive(thiserror::Error, Debug)]
pub enum LoadDataError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse json: {0}")]
    Json(#[from] serde_json::Error),
}

macro_rules! json_data {
    ($($table_type:ident;)*) => {
        $(paste! {
            mod [<$table_type:snake>];
            pub use [<$table_type:snake>]::[<$table_type Data>];
        })*

        $(paste! {
            pub mod [<$table_type:snake _data>] {
                use std::sync::OnceLock;
                type Data = super::[<$table_type Data>];
                pub(crate) static TABLE: OnceLock<Vec<Data>> = OnceLock::new();

                pub fn iter() -> std::slice::Iter<'static, Data> {
                    TABLE.get().unwrap().iter()
                }
            }
        })*

        fn load_json_data(base_path: &str) -> Result<(), LoadDataError> {
            $(paste! {
                let json_content = std::fs::read_to_string(&format!("{}/{}.json", base_path, stringify!($table_type)))?;
                let _ = [<$table_type:snake _data>]::TABLE.set(serde_json::from_str(&json_content)?);
            })*

            Ok(())
        }
    };
}

macro_rules! json_hash_table_data {
    ($($table_type:ident, $key_param:expr;)*) => {
        $(paste! {
            mod [<$table_type:snake>];
            pub use [<$table_type:snake>]::[<$table_type Data>];
        })*

        $(paste! {
            pub mod [<$table_type:snake _data>] {
                use std::collections::HashMap;
                use std::sync::OnceLock;

                pub(crate) type Data = super::[<$table_type Data>];
                pub(crate) static TABLE: OnceLock<HashMap<i64, Data>> = OnceLock::new();

                pub fn iter() -> std::collections::hash_map::Iter<'static, i64, Data> {
                    TABLE.get().unwrap().iter()
                }
            }
        })*

        fn load_json_hash_table_data(base_path: &str) -> Result<(), LoadDataError> {
            $(paste! {
                let json_content = std::fs::read_to_string(&format!("{}/{}.json", base_path, stringify!($table_type)))?;
                let _ = [<$table_type:snake _data>]::TABLE.set(
                    serde_json::from_str::<Vec<[<$table_type:snake _data>]::Data>>(&json_content)?
                        .iter()
                        .cloned()
                        .map(|element| (element.$key_param, element))
                        .collect::<std::collections::HashMap<_, _>>()
                );
            })*

            Ok(())
        }
    };
}

pub fn load_all_json_data(base_path: &str) -> Result<(), LoadDataError> {
    load_json_data(base_path)?;
    load_json_hash_table_data(base_path)?;
    Ok(())
}

json_data! {
    RoleInfo;
    WeaponConf;
    BaseProperty;
    InstanceDungeon;
    FunctionCondition;
    ExploreTools;
}

json_hash_table_data! {
    LevelEntityConfig, entity_id;
}