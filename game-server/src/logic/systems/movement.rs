use shorekeeper_protocol::{MovePackageNotify, MovingEntityData};

use crate::{logic::ecs::component::ComponentContainer, query_components};
use std::cell::RefMut;

use crate::{
    logic::{ecs::world::World, player::Player},
    query_with,
};

use super::System;

pub(super) struct MovementSystem;

impl System for MovementSystem {
    fn tick(&self, world: &mut World, players: &mut [RefMut<Player>]) {
        let mut notify = MovePackageNotify::default();
        let world_entity = world.get_world_entity();

        for (entity, mut movement, mut position) in query_with!(world_entity, Movement, Position) {
            if movement.pending_movement_vec.is_empty() {
                continue;
            }

            let mut moving_entity_data = MovingEntityData {
                entity_id: entity.into(),
                ..Default::default()
            };

            while let Some(info) = movement.pending_movement_vec.pop_front() {
                if let Some(location) = info.location.as_ref() {
                    position.0.set_position_from_protobuf(location);
                }

                if let Some(rotation) = info.rotation.as_ref() {
                    position.0.set_rotation_from_protobuf(rotation);
                }

                moving_entity_data.move_infos.push(info);
            }

            tracing::debug!(
                "MovementSystem: entity with id {} moved to {:?}",
                i64::from(entity),
                &position.0.position
            );

            notify.moving_entities.push(moving_entity_data);

            if let (Some(_), Some(owner)) = query_components!(
                world_entity,
                i64::from(entity),
                PlayerEntityMarker,
                OwnerPlayer
            ) {
                if let Some(player) = players.iter_mut().find(|pl| pl.basic_info.id == owner.0) {
                    player.location.position = position.0.clone();
                }
            }
        }

        if !notify.moving_entities.is_empty() {
            players
                .iter()
                .for_each(|player| player.notify(notify.clone()))
        }
    }
}

impl MovementSystem {
    pub fn new() -> Self {
        Self
    }
}
