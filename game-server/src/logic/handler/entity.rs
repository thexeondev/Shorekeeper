use crate::{logic::ecs::component::ComponentContainer, logic::player::Player, query_components};
use shorekeeper_protocol::entity_component_pb::ComponentPb;
use shorekeeper_protocol::{
    EntityActiveRequest, EntityActiveResponse, EntityComponentPb, EntityLoadCompleteRequest,
    EntityLoadCompleteResponse, EntityOnLandedRequest, EntityOnLandedResponse,
    EntityPositionRequest, EntityPositionResponse, ErrorCode, MovePackagePush,
};

pub fn on_entity_active_request(
    player: &Player,
    request: EntityActiveRequest,
    response: &mut EntityActiveResponse,
) {
    let world_ref = player.world.borrow();
    let world = world_ref.get_world_entity();

    if !world.is_in_all_world_map(request.entity_id as i32) {
        tracing::debug!(
            "EntityActiveRequest: entity with id {} doesn't exist, player_id: {}",
            request.entity_id,
            player.basic_info.id
        );
        return;
    };

    if let (Some(position), Some(attribute)) =
        query_components!(world, request.entity_id, Position, Attribute)
    {
        response.is_visible = true;
        response.pos = Some(position.0.get_position_protobuf());
        response.rot = Some(position.0.get_rotation_protobuf());

        response.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::AttributeComponent(
                attribute.build_entity_attribute(),
            )),
        });

        response.error_code = ErrorCode::Success.into();
    } else {
        tracing::error!(
            "EntityActiveRequest: entity with id {} not found",
            request.entity_id
        );
        response.error_code = ErrorCode::ErrEntityNotFound.into(); // TODO: replace with appropriate error code
        return;
    };
}

pub fn on_entity_on_landed_request(
    _: &Player,
    request: EntityOnLandedRequest,
    _: &mut EntityOnLandedResponse,
) {
    // TODO: More implementation?
    tracing::debug!(
        "EntityOnLandedRequest: entity with id {} landed",
        request.entity_id
    );
}

pub fn on_entity_position_request(
    _: &Player,
    request: EntityPositionRequest,
    _: &mut EntityPositionResponse,
) {
    // TODO: Implement this
    tracing::debug!(
        "EntityPositionRequest: config with id {} for map {} position requested",
        request.config_id,
        request.map_id
    );
}

pub fn on_entity_load_complete_request(
    _: &Player,
    request: EntityLoadCompleteRequest,
    _: &mut EntityLoadCompleteResponse,
) {
    // TODO: Implement this
    tracing::debug!(
        "EntityLoadCompleteRequest: for ids {:?}",
        request.entity_ids
    );
}

pub fn on_move_package_push(player: &mut Player, push: MovePackagePush) {
    let world_ref = player.world.borrow();
    let world = world_ref.get_world_entity();

    for moving_entity in push.moving_entities {
        if !world.is_in_all_world_map(moving_entity.entity_id as i32) {
            tracing::debug!(
                "MovePackage: entity with id {} doesn't exist",
                moving_entity.entity_id
            );
            continue;
        }

        let Some(mut movement) = query_components!(world, moving_entity.entity_id, Movement).0
        else {
            tracing::warn!(
                "MovePackage: entity {} doesn't have movement component",
                moving_entity.entity_id
            );
            continue;
        };

        movement
            .pending_movement_vec
            .extend(moving_entity.move_infos);
    }
}
