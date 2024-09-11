use crate::logic::ecs::component::Component;

pub struct OwnerPlayer(pub i32);

impl Component for OwnerPlayer {
    fn set_pb_data(&self, pb: &mut shorekeeper_protocol::EntityPb) {
        pb.player_id = self.0;
    }
}
