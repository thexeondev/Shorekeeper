use crate::logic::ecs::component::ComponentContainer;
use shorekeeper_protocol::{EEntityType, EntityPb, PlayerSceneAoiData};
use std::collections::HashSet;

use crate::logic::components::Visibility;
use crate::logic::player::Player;
use crate::{modify_component, query_hn_with};

pub fn build_scene_add_on_init_data(player: &Player) -> PlayerSceneAoiData {
    let mut world_ref = player.world.borrow_mut();
    let world = world_ref.get_mut_world_entity();

    let entities = query_hn_with!(world, PlayerEntityMarker)
        .into_iter()
        .map(|(entity_id, _)| {
            let res_map: (EEntityType, i32);
            match EEntityType::try_from(
                world.get_entity(world.get_config_id(entity_id)).entity_type,
            ) {
                Ok(EEntityType::Player) => {
                    res_map = (EEntityType::Player, entity_id);
                }
                _ => {
                    res_map = (EEntityType::default(), -1);
                }
            }
            res_map
        })
        .collect::<HashSet<(EEntityType, i32)>>();

    let mut aoi_data = PlayerSceneAoiData::default();

    entities
        .iter()
        .filter(|&&(_, entity_id)| entity_id != -1)
        .for_each(|&(entity_type, entity_id)| {
            match entity_type {
                EEntityType::Player => {
                    let config_id = world.get_config_id(entity_id);
                    modify_component!(
                        world.get_entity_components(entity_id),
                        Visibility,
                        |vis: &mut Visibility| {
                            let cur_role_id = player
                                .formation_list
                                .get(&player.cur_formation_id)
                                .unwrap()
                                .cur_role;
                            vis.0 = if config_id == cur_role_id {
                                true
                            } else {
                                false
                            };
                        }
                    );

                    if world.get_entity(config_id).entity_type == EEntityType::Player as i32 {
                        let mut pb = EntityPb {
                            id: entity_id as i64,
                            ..Default::default()
                        };
                        world
                            .get_entity_components(entity_id)
                            .into_iter()
                            .for_each(|comp| comp.set_pb_data(&mut pb));

                        aoi_data.entities.push(pb);
                    }
                }
                _ => {}
            };
        });

    aoi_data
}
