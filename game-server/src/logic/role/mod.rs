use std::collections::HashMap;

use common::time_util;
pub use formation::RoleFormation;
use shorekeeper_data::role_info_data;
use shorekeeper_protocol::{ArrayIntInt, RoleData, RoleInfo};

mod formation;
pub struct Role {
    pub role_id: i32,
    pub name: String,
    pub level: i32,
    pub exp: i32,
    pub breakthrough: i32,
    pub skill_map: HashMap<i32, i32>,
    pub star: i32,
    pub favor: i32,
    pub create_time: u32,
    pub equip_weapon: i32,
}

impl Role {
    pub const MAIN_CHARACTER_MALE_ID: i32 = 1501;
    pub const MAIN_CHARACTER_FEMALE_ID: i32 = 1502;

    pub fn new(role_id: i32, weapon_id: Option<i32>) -> Self {
        let data = role_info_data::iter().find(|d| d.id == role_id).unwrap();
        let equip_weapon = match weapon_id {
            None => data.init_weapon_item_id,
            Some(x) => x,
        };

        Self {
            role_id,
            name: String::with_capacity(0),
            level: data.max_level,
            exp: 0,
            breakthrough: 0,
            skill_map: HashMap::new(), // TODO!
            star: 0,
            favor: 0,
            create_time: time_util::unix_timestamp() as u32,
            equip_weapon,
        }
    }

    pub fn to_protobuf(&self) -> RoleInfo {
        RoleInfo {
            role_id: self.role_id,
            name: self.name.clone(),
            level: self.level,
            exp: self.exp,
            breakthrough: self.breakthrough,
            create_time: self.create_time,
            skills: self
                .skill_map
                .iter()
                .map(|(k, v)| ArrayIntInt { key: *k, value: *v })
                .collect(),
            star: self.star,
            favor: self.favor,
            ..Default::default()
        }
    }

    pub fn load_from_save(data: RoleData) -> Self {
        Self {
            role_id: data.role_id,
            name: data.name,
            level: data.level,
            exp: data.exp,
            breakthrough: data.breakthrough,
            skill_map: data.skill_map,
            star: data.star,
            favor: data.favor,
            create_time: data.create_time,
            equip_weapon: data.equip_weapon,
        }
    }

    pub fn build_save_data(&self) -> RoleData {
        RoleData {
            role_id: self.role_id,
            name: self.name.clone(),
            level: self.level,
            exp: self.exp,
            breakthrough: self.breakthrough,
            skill_map: self.skill_map.clone(),
            star: self.star,
            favor: self.favor,
            create_time: self.create_time,
            equip_weapon: self.equip_weapon,
            ..Default::default()
        }
    }
}
