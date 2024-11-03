use crate::logic::ecs::component::Component;
use shorekeeper_protocol::entity_component_pb::ComponentPb;
use shorekeeper_protocol::{DFsm, DFsmBlackBoard, EntityComponentPb, EntityFsmComponentPb, FsmCustomBlackboardDatas};

pub struct Fsm {
    pub fsms: Vec<DFsm>,
    pub hash_code: i32,
    pub common_hash_code: i32,
    pub black_board: Vec<DFsmBlackBoard>,
    pub fsm_custom_blackboard_datas: Option<FsmCustomBlackboardDatas>,
}

impl Component for Fsm {
    fn set_pb_data(&self, pb: &mut shorekeeper_protocol::EntityPb) {
        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::EntityFsmComponentPb(EntityFsmComponentPb {
                fsms: self.fsms.clone(),
                hash_code: self.hash_code,
                common_hash_code: self.common_hash_code,
                black_board: self.black_board.clone(),
                fsm_custom_blackboard_datas: self.fsm_custom_blackboard_datas.clone(),
            })),
        })
    }
}
