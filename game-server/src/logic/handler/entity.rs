use shorekeeper_protocol::{EntityActiveRequest, EntityActiveResponse, EntityLoadCompleteRequest,
                           EntityLoadCompleteResponse, EntityOnLandedRequest,
                           EntityOnLandedResponse, EntityPb, EntityPositionRequest,
                           EntityPositionResponse, ErrorCode, MovePackagePush};

use crate::{logic, logic::ecs::component::ComponentContainer, logic::player::Player, query_components};

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

    let component_pbs = {
        let mut pb = EntityPb {
            id: request.entity_id,
            ..Default::default()
        };

        world.get_entity_components(request.entity_id as i32)
            .into_iter()
            .for_each(|comp| comp.set_pb_data(&mut pb));
        pb.component_pbs
    };

    // TODO: Remove attribute
    if let (Some(position), Some(_attribute)) =
        query_components!(world, request.entity_id, Position, Attribute)
    {
        response.is_visible = true;
        response.pos = Some(position.0.get_position_protobuf());
        response.rot = Some(position.0.get_rotation_protobuf());

        response.component_pbs.extend_from_slice(&component_pbs);

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
    for moving_entity in push.moving_entities {
        // Query components borrows world component so lets wrap it
        {
            let world_ref = player.world.borrow();
            let world = world_ref.get_world_entity();

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

        // TODO: review instance id vs map id in world
        let map = logic::utils::quadrant_util::get_map(player.location.instance_id);
        let quadrant_id = map.get_quadrant_id(
            player.location.position.position.x * 100.0,
            player.location.position.position.y * 100.0,
        );

        // TODO: This may require some changes for Co-Op
        if quadrant_id != player.quadrant_id {
            let (entities_to_remove, entities_to_add) = map.get_update_entities(player.quadrant_id, quadrant_id);
            player.quadrant_id = quadrant_id;
            logic::utils::world_util::remove_entities(player, &entities_to_remove);
            logic::utils::world_util::add_entities(player, &entities_to_add);
        }
    }
}
