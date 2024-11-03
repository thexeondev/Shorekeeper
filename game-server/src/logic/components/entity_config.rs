use shorekeeper_protocol::{EEntityType, EntityConfigType, EntityState};

use crate::logic::ecs::component::Component;

pub struct EntityConfig {
    pub config_id: i32,
    pub config_type: EntityConfigType,
    pub entity_type: EEntityType,
    pub entity_state: EntityState
}

impl Component for EntityConfig {
    fn set_pb_data(&self, pb: &mut shorekeeper_protocol::EntityPb) {
        pb.config_id = self.config_id;
        pb.config_type = self.config_type.into();
        pb.entity_type = self.entity_type.into();
        pb.entity_state = self.entity_state.into();
    }
}
