use crate::logic::ecs::component::ComponentContainer;
use shorekeeper_protocol::{EntityPb, PlayerSceneAoiData};

use crate::{logic::ecs::world::World, query_with};

pub fn build_scene_add_on_init_data(world: &World) -> PlayerSceneAoiData {
    let entities = query_with!(world, PlayerEntityMarker)
        .into_iter()
        .map(|(e, _)| e)
        .collect::<Vec<_>>();

    let mut aoi_data = PlayerSceneAoiData::default();
    for entity in entities {
        let mut pb = EntityPb::default();
        pb.id = entity.into();

        world
            .get_entity_components(entity)
            .into_iter()
            .for_each(|comp| comp.set_pb_data(&mut pb));

        aoi_data.entities.push(pb);
    }

    aoi_data
}
