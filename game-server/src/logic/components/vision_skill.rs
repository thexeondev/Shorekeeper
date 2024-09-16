use shorekeeper_protocol::{
    entity_component_pb::ComponentPb, EntityComponentPb, VisionSkillComponentPb,
    VisionSkillInformation,
};

use crate::logic::ecs::component::Component;

pub struct VisionSkill {
    pub skill_id: i32,
}

impl Component for VisionSkill {
    fn set_pb_data(&self, pb: &mut shorekeeper_protocol::EntityPb) {
        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::VisionSkillComponent(VisionSkillComponentPb {
                vision_skill_infos: vec![VisionSkillInformation {
                    skill_id: self.skill_id,
                    ..Default::default()
                }],
            })),
        })
    }
}
