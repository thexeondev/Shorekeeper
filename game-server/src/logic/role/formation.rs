use shorekeeper_protocol::RoleFormationData;

pub struct RoleFormation {
    pub id: i32,
    pub cur_role: i32,
    pub role_ids: Vec<i32>,
    pub is_current: bool,
}

impl RoleFormation {
    pub fn load_from_save(data: RoleFormationData) -> Self {
        Self {
            id: data.formation_id,
            cur_role: data.cur_role,
            role_ids: data.role_id_list,
            is_current: data.is_current,
        }
    }

    pub fn build_save_data(&self) -> RoleFormationData {
        RoleFormationData {
            formation_id: self.id,
            cur_role: self.cur_role,
            role_id_list: self.role_ids.iter().map(|&role_id| role_id).collect(),
            is_current: self.is_current,
        }
    }
}
