use crate::logic::ecs::component::ComponentContainer;
use shorekeeper_protocol::{
    VisionExploreSkillSetRequest, VisionExploreSkillSetResponse, VisionSkillChangeNotify,
    VisionSkillInformation,
};

use crate::{logic::player::Player, query_with};

pub fn on_vision_explore_skill_set_request(
    player: &mut Player,
    request: VisionExploreSkillSetRequest,
    response: &mut VisionExploreSkillSetResponse,
) {
    player.explore_tools.active_explore_skill = request.skill_id;

    for (entity, owner, mut vision_skill) in query_with!(
        player.world.borrow().get_world_entity(),
        OwnerPlayer,
        VisionSkill
    ) {
        if owner.0 == player.basic_info.id {
            vision_skill.skill_id = request.skill_id;
            player.notify(VisionSkillChangeNotify {
                entity_id: entity.into(),
                vision_skill_infos: vec![VisionSkillInformation {
                    skill_id: request.skill_id,
                    ..Default::default()
                }],
                ..Default::default()
            })
        }
    }

    response.skill_id = request.skill_id;
}
