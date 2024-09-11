use shorekeeper_protocol::EntityConfigType;

use crate::logic::ecs::component::Component;

pub struct EntityConfig {
    pub config_id: i32,
    pub config_type: EntityConfigType,
}

impl Component for EntityConfig {
    fn set_pb_data(&self, pb: &mut shorekeeper_protocol::EntityPb) {
        pb.config_id = self.config_id;
        pb.config_type = self.config_type.into();
    }
}
