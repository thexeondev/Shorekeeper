use std::collections::HashSet;

use shorekeeper_protocol::RoleFormationData;

pub struct RoleFormation {
    pub id: i32,
    pub cur_role: i32,
    pub role_id_set: HashSet<i32>,
    pub is_current: bool,
}

impl RoleFormation {
    pub fn load_from_save(data: RoleFormationData) -> Self {
        Self {
            id: data.formation_id,
            cur_role: data.cur_role,
            role_id_set: data.role_id_list.into_iter().collect(),
            is_current: data.is_current,
        }
    }

    pub fn build_save_data(&self) -> RoleFormationData {
        RoleFormationData {
            formation_id: self.id,
            cur_role: self.cur_role,
            role_id_list: self.role_id_set.iter().cloned().collect(),
            is_current: self.is_current,
        }
    }
}
