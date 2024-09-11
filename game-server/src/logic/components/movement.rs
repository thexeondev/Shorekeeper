use std::collections::VecDeque;

use shorekeeper_protocol::MoveReplaySample;

use crate::logic::ecs::component::Component;

#[derive(Default)]
pub struct Movement {
    pub pending_movement_vec: VecDeque<MoveReplaySample>,
}

impl Component for Movement {
    fn set_pb_data(&self, _: &mut shorekeeper_protocol::EntityPb) {}
}
