use shorekeeper_data::base_property_data;
use shorekeeper_protocol::{
    EntityConfigType, FightRoleInfo, FightRoleInfos, LivingStatus, SceneInformation, SceneMode,
    ScenePlayerInformation, SceneTimeInfo,
};

use crate::{
    logic::{
        components::{
            Attribute, EntityConfig, Movement, OwnerPlayer, PlayerEntityMarker, Position,
            Visibility, Weapon
        },
        ecs::{component::ComponentContainer, world::World},
        player::Player,
    },
    query_with,
};
use super::entity_serializer;

pub fn add_player_entities(world: &mut World, player: &Player) {
    let cur_role_id = player.get_cur_role_id();

    for role in player.get_current_formation_role_list() {
        let id = world
            .create_entity()
            .with(ComponentContainer::PlayerEntityMarker(PlayerEntityMarker))
            .with(ComponentContainer::EntityConfig(EntityConfig {
                config_id: role.role_id,
                config_type: EntityConfigType::Character,
            }))
            .with(ComponentContainer::OwnerPlayer(OwnerPlayer(
                player.basic_info.id,
            )))
            .with(ComponentContainer::Position(Position(
                player.location.position.clone(),
            )))
            .with(ComponentContainer::Visibility(Visibility(
                role.role_id == cur_role_id,
            )))
            .with(ComponentContainer::Attribute(Attribute::from_data(
                base_property_data::iter()
                    .find(|d| d.id == role.role_id)
                    .unwrap(),
            )))
            .with(ComponentContainer::Movement(Movement::default()))
            .with(ComponentContainer::Weapon(Weapon {
                weapon_id: role.equip_weapon,
                weapon_breach_level: 0, // TODO: store this too
            }))
            .build();

        tracing::debug!(
            "created player entity, id: {}, role_id: {}",
            i64::from(id),
            role.role_id
        );
    }
}

pub fn build_scene_information(world: &World, instance_id: i32, owner_id: i32) -> SceneInformation {
    SceneInformation {
        scene_id: String::new(),
        instance_id,
        owner_id,
        dynamic_entity_list: Vec::new(),
        blackboard_params: Vec::new(),
        end_time: 0,
        aoi_data: Some(entity_serializer::build_scene_add_on_init_data(world)),
        player_infos: build_player_info_list(world),
        mode: SceneMode::Single.into(),
        time_info: Some(SceneTimeInfo {
            owner_time_clock_time_span: 0,
            hour: 8,
            minute: 0,
        }),
        cur_context_id: owner_id as i64,
        ..Default::default()
    }
}

fn build_player_info_list(world: &World) -> Vec<ScenePlayerInformation> {
    world
        .players()
        .map(|sp| {
            let (cur_role_id, transform) = query_with!(
                world,
                PlayerEntityMarker,
                OwnerPlayer,
                Visibility,
                EntityConfig,
                Position
            )
            .into_iter()
            .find_map(|(_, _, owner, visibility, conf, pos)| {
                (sp.player_id == owner.0 && visibility.0).then_some((conf.config_id, pos.0.clone()))
            })
            .unwrap_or_default();

            let active_characters =
                query_with!(world, PlayerEntityMarker, OwnerPlayer, EntityConfig)
                    .into_iter()
                    .filter(|(_, _, owner, _)| owner.0 == sp.player_id);

            ScenePlayerInformation {
                cur_role: cur_role_id,
                group_type: sp.group_type,
                player_id: sp.player_id,
                player_icon: sp.player_icon,
                player_name: sp.player_name.clone(),
                level: sp.level,
                location: Some(transform.get_position_protobuf()),
                rotation: Some(transform.get_rotation_protobuf()),
                fight_role_infos: Vec::from([FightRoleInfos {
                    group_type: sp.group_type,
                    living_status: LivingStatus::Alive.into(),
                    cur_role: cur_role_id,
                    is_retain: true,
                    fight_role_infos: active_characters
                        .map(|(id, _, _, conf)| FightRoleInfo {
                            entity_id: id.into(),
                            role_id: conf.config_id,
                        })
                        .collect(),
                    ..Default::default()
                }]),
                ..Default::default()
            }
        })
        .collect()
}
