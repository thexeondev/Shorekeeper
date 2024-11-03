use crate::logic::ecs::component::Component;
use shorekeeper_protocol::entity_component_pb::ComponentPb;
use shorekeeper_protocol::{EntityComponentPb, MonsterAiComponentPb};

pub struct MonsterAi {
    pub weapon_id: i32,
    pub hatred_group_id: i64,
    pub ai_team_init_id: i32,
    pub combat_message_id: i64,
}

impl Component for MonsterAi {
    fn set_pb_data(&self, pb: &mut shorekeeper_protocol::EntityPb) {
        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::MonsterAiComponentPb(MonsterAiComponentPb {
                weapon_id: self.weapon_id,
                hatred_group_id: self.hatred_group_id,
                ai_team_init_id: self.ai_team_init_id,
                combat_message_id: self.combat_message_id,
            })),
        })
    }
}
