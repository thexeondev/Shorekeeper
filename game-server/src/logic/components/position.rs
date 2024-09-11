use crate::logic::{ecs::component::Component, math::Transform};

pub struct Position(pub Transform);

impl Component for Position {
    fn set_pb_data(&self, pb: &mut shorekeeper_protocol::EntityPb) {
        pb.pos = Some(self.0.get_position_protobuf());
        pb.rot = Some(self.0.get_rotation_protobuf());
        pb.init_pos = Some(self.0.get_position_protobuf());
    }
}
