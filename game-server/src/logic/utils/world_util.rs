use crate::logic::ecs::world::World;
use crate::logic::player::Player;
use crate::logic::utils::entity_serializer;
use crate::logic::{
    components::{
        Attribute, EntityConfig, Equip, Movement, OwnerPlayer, PlayerEntityMarker, Position,
        Visibility, VisionSkill,
    },
    ecs::component::ComponentContainer,
};
use crate::query_with;
use shorekeeper_data::base_property_data;
use shorekeeper_protocol::{
    EEntityType, EntityConfigType, FightRoleInfo, FightRoleInfos, LivingStatus, SceneInformation,
    SceneMode, ScenePlayerInformation, SceneTimeInfo,
};

#[macro_export]
macro_rules! create_player_entity_pb {
    ($role_list:expr, $cur_map_id:expr, $world:expr, $player_id:expr, $position:expr, $explore_tools:expr) => {{
        let mut pbs = Vec::new();

        for role in $role_list {
            let role_id: i32 = role.role_id;
            let base_property = base_property_data::iter()
                .find(|d| d.id == role_id)
                .expect("macro create_role_entity_pb: Base property data not found");

            let entity = $world
                .create_entity(role_id, EEntityType::Player.into(), $cur_map_id)
                .with(ComponentContainer::PlayerEntityMarker(PlayerEntityMarker))
                .with(ComponentContainer::EntityConfig(EntityConfig {
                    config_id: role_id,
                    config_type: EntityConfigType::Character,
                }))
                .with(ComponentContainer::OwnerPlayer(OwnerPlayer($player_id)))
                .with(ComponentContainer::Position(Position($position)))
                .with(ComponentContainer::Visibility(Visibility(
                    role_id == role_id,
                )))
                .with(ComponentContainer::Attribute(Attribute::from_data(
                    base_property,
                )))
                .with(ComponentContainer::Movement(Movement::default()))
                .with(ComponentContainer::Equip(Equip {
                    weapon_id: role.equip_weapon,
                    weapon_breach_level: 90, // TODO: store this too
                }))
                .with(ComponentContainer::VisionSkill(VisionSkill {
                    skill_id: $explore_tools.active_explore_skill,
                }))
                .build();

            let mut pb = EntityPb {
                id: entity.entity_id as i64,
                ..Default::default()
            };

            $world
                .get_entity_components(entity.entity_id)
                .into_iter()
                .for_each(|comp| comp.set_pb_data(&mut pb));
            pbs.push(pb);
        }

        EntityAddNotify {
            entity_pbs: pbs,
            is_add: true,
        }
    }};
}

pub fn add_player_entities(player: &Player) {
    let mut world_ref = player.world.borrow_mut();
    let world = world_ref.get_mut_world_entity();

    let current_formation = player.formation_list.get(&player.cur_formation_id).unwrap();

    let role_vec = current_formation
        .role_ids
        .iter()
        .map(|role_id| player.role_list.get(&role_id).unwrap())
        .collect::<Vec<_>>();
    let cur_role_id = current_formation.cur_role;

    if world.active_entity_empty() {
        for role in role_vec {
            let entity = world
                .create_entity(
                    role.role_id,
                    EEntityType::Player.into(),
                    player.basic_info.cur_map_id,
                )
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
                .with(ComponentContainer::Equip(Equip {
                    weapon_id: role.equip_weapon,
                    weapon_breach_level: 0, // TODO: store this too
                }))
                .with(ComponentContainer::VisionSkill(VisionSkill {
                    skill_id: player.explore_tools.active_explore_skill,
                }))
                .build();

            tracing::debug!(
                "created player entity, id: {}, role_id: {}",
                entity.entity_id,
                role.role_id
            );
        }
    }
}

pub fn build_scene_information(player: &Player) -> SceneInformation {
    SceneInformation {
        scene_id: String::new(),
        instance_id: player.location.instance_id,
        owner_id: player.basic_info.id,
        dynamic_entity_list: Vec::new(),
        blackboard_params: Vec::new(),
        end_time: 0,
        aoi_data: Some(entity_serializer::build_scene_add_on_init_data(player)),
        player_infos: build_player_info_list(&player.world.borrow_mut()),
        mode: SceneMode::Single.into(),
        time_info: Some(SceneTimeInfo {
            owner_time_clock_time_span: 0,
            hour: 8,
            minute: 0,
        }),
        cur_context_id: player.basic_info.id as i64,
        ..Default::default()
    }
}

fn build_player_info_list(world: &World) -> Vec<ScenePlayerInformation> {
    world
        .players()
        .map(|sp| {
            let (cur_role_id, transform, _equip) = query_with!(
                world.get_world_entity(),
                PlayerEntityMarker,
                OwnerPlayer,
                Visibility,
                EntityConfig,
                Position,
                Equip
            )
            .into_iter()
            .find_map(|(_, _, owner, visibility, conf, pos, equip)| {
                (sp.player_id == owner.0 && visibility.0).then_some((
                    conf.config_id,
                    pos.0.clone(),
                    equip.weapon_id,
                ))
            })
            .unwrap_or_default();

            let active_characters = query_with!(
                world.get_world_entity(),
                PlayerEntityMarker,
                OwnerPlayer,
                EntityConfig
            )
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
