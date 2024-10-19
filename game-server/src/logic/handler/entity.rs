use shorekeeper_protocol::{EntityActiveRequest, EntityActiveResponse, EntityLoadCompleteRequest,
                           EntityLoadCompleteResponse, EntityOnLandedRequest,
                           EntityOnLandedResponse, EntityPositionRequest, EntityPositionResponse,
                           ErrorCode, MovePackagePush,
};

use crate::{logic::ecs::component::ComponentContainer, logic::player::Player, query_components};

pub fn on_entity_active_request(
    player: &Player,
    request: EntityActiveRequest,
    response: &mut EntityActiveResponse,
) {
    let world = player.world.borrow();

    if !world.is_in_world(request.entity_id) {
        tracing::debug!(
            "EntityActiveRequest: entity with id {} doesn't exist, player_id: {}",
            request.entity_id,
            player.basic_info.id
        );
        return;
    };

    if let Some(position) = query_components!(world, request.entity_id, Position).0 {
        // TODO: proper entity "activation" logic
        response.pos = Some(position.0.get_position_protobuf());
        response.rot = Some(position.0.get_rotation_protobuf());
    }

    response.component_pbs = Vec::new(); // not implemented
    response.error_code = ErrorCode::Success.into();
}

pub fn on_entity_on_landed_request(
    _: &Player,
    request: EntityOnLandedRequest,
    _: &mut EntityOnLandedResponse,
) {
    // TODO: More implementation?
    tracing::debug!("EntityOnLandedRequest: entity with id {} landed", request.entity_id);
}

pub fn on_entity_position_request(_: &Player,
                                  request: EntityPositionRequest,
                                  _: &mut EntityPositionResponse,
) {
    // TODO: Implement this
    tracing::debug!(
        "EntityPositionRequest: config with id {} for map {} position requested",
        request.config_id, request.map_id
    );
}

pub fn on_entity_load_complete_request(_: &Player,
                                       request: EntityLoadCompleteRequest,
                                       _: &mut EntityLoadCompleteResponse,
) {
    // TODO: Implement this
    tracing::debug!("EntityLoadCompleteRequest: for ids {:?}", request.entity_ids);
}

pub fn on_move_package_push(player: &mut Player, push: MovePackagePush) {
    let world = player.world.borrow();

    for moving_entity in push.moving_entities {
        if !world.is_in_world(moving_entity.entity_id) {
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
