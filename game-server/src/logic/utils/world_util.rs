use std::cell::{BorrowMutError, RefMut};

use shorekeeper_data::{base_property_data, LevelEntityConfigData};
use shorekeeper_protocol::{DFsm, EEntityType, EntityAddNotify, EntityConfigType, EntityPb,
                           EntityRemoveInfo, EntityRemoveNotify, EntityState, FightRoleInfo,
                           FightRoleInfos, LivingStatus, SceneInformation, SceneMode,
                           ScenePlayerInformation, SceneTimeInfo};

use crate::logic::{
    components::{
        Attribute, EntityConfig, Equip, Movement, OwnerPlayer, PlayerEntityMarker, Position,
        Visibility, VisionSkill,
    },
    ecs::component::ComponentContainer,
};
use crate::logic::components::{Fsm, MonsterAi};
use crate::logic::ecs::entity::Entity;
use crate::logic::ecs::world::{World, WorldEntity};
use crate::logic::math::Transform;
use crate::logic::player::Player;
use crate::logic::utils::entity_serializer;
use crate::query_with;

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
                    entity_type: EEntityType::Player.into(),
                    entity_state: EntityState::Default
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
                    entity_type: EEntityType::Player.into(),
                    entity_state: EntityState::Default,
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

pub fn build_monster_entity(world: &mut WorldEntity, config_id: i32, map_id: i32, transform: Transform) -> Entity {
    // TODO: Check for more components, AI and so
    world.create_entity(config_id, EEntityType::Monster.into(), map_id)
        .with(ComponentContainer::EntityConfig(EntityConfig {
            config_id,
            config_type: EntityConfigType::Level,
            entity_type: EEntityType::Monster.into(),
            entity_state: EntityState::Born,
        }))
        .with(ComponentContainer::Position(Position(transform)))
        .with(ComponentContainer::Visibility(Visibility(true)))
        .with(ComponentContainer::Attribute(Attribute::from_data(
            base_property_data::iter()
                .find(|d| d.id == 600000100) // TODO: Implement monster stats
                .unwrap(),
        )))
        .with(ComponentContainer::MonsterAi(MonsterAi {
            weapon_id: 0,
            hatred_group_id: 0,
            ai_team_init_id: 100,
            combat_message_id: 0,
        }))
        .with(ComponentContainer::Fsm(Fsm {
            fsms: vec![
                DFsm {
                    fsm_id: 10007,
                    current_state: 10013,
                    flag: 0,
                    k_ts: 0,
                },
                DFsm {
                    fsm_id: 10007,
                    current_state: 10015,
                    flag: 0,
                    k_ts: 0,
                },
                DFsm {
                    fsm_id: 10007,
                    current_state: 10012,
                    flag: 0,
                    k_ts: 0,
                },
            ],
            hash_code: 0,
            common_hash_code: 0,
            black_board: vec![],
            fsm_custom_blackboard_datas: None,
        }))
        .with(ComponentContainer::Movement(Movement::default()))
        .build()
}

pub fn remove_entities(player: &Player, entities: &[&LevelEntityConfigData]) {
    let mut removed_entities = Vec::with_capacity(entities.len());
    // Enclose to drop borrow mut ASAP
    {
        let mut world_ref = player.world.borrow_mut();
        let world = world_ref.get_mut_world_entity();

        for entity in entities {
            let entity_id = entity.entity_id as i32; // TODO: Should be i64
            if world.remove_entity(entity_id) {
                removed_entities.push(world.get_entity_id(entity_id));
            }
        }
    }
    for entity_id in removed_entities {
        player.notify(EntityRemoveNotify {
            remove_infos: vec![EntityRemoveInfo { entity_id, r#type: 0 }],
            is_remove: true,
        });
    }
}

pub fn add_entities(player: &Player, entities: &[&LevelEntityConfigData]) {
    let mut added_entities = Vec::with_capacity(entities.len());
    // Enclose to drop borrow mut ASAP
    {
        let mut world_ref = player.world.borrow_mut();
        let world = world_ref.get_mut_world_entity();

        for entity in entities {
            // TODO: review other types
            tracing::debug!("Entity to be added of type: {}", entity.blueprint_type);
            if entity.blueprint_type.contains("Monster") {
                added_entities.push(build_monster_entity(
                    world,
                    entity.entity_id as i32, // TODO: Should be i64
                    entity.map_id,
                    Transform::from(&entity.transform[..]),
                ));
            }
        }
    }

    let mut world_ref = player.world.borrow();
    let world = world_ref.get_world_entity();
    // Since kuro has issues, we can only send one
    for entity in added_entities {
        let mut pb = EntityPb {
            id: entity.entity_id as i64, // TODO: Should be i64
            ..Default::default()
        };

        world.get_entity_components(entity.entity_id)
            .into_iter()
            .for_each(|comp| comp.set_pb_data(&mut pb));

        player.notify(EntityAddNotify {
            entity_pbs: vec![pb],
            is_add: true,
        });
    }
}