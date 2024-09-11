use std::cell::{RefCell, RefMut};
use std::collections::hash_map::{Keys, Values};
use std::collections::HashMap;

use crate::logic::player::InWorldPlayer;

use super::component::ComponentContainer;
use super::entity::{Entity, EntityBuilder, EntityManager};

pub struct World {
    components: HashMap<Entity, Vec<RefCell<ComponentContainer>>>,
    entity_manager: EntityManager,
    in_world_players: HashMap<i32, InWorldPlayer>, // joined players metadata
}

impl World {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            entity_manager: EntityManager::default(),
            in_world_players: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        let entity = self.entity_manager.create();
        EntityBuilder::builder(entity, self.components.entry(entity).or_insert(Vec::new()))
    }

    pub fn is_in_world(&self, entity_id: i64) -> bool {
        self.entity_manager.get(entity_id).is_some()
    }

    pub fn components(&self) -> &HashMap<Entity, Vec<RefCell<ComponentContainer>>> {
        &self.components
    }

    pub fn get_entity_components(&self, entity: Entity) -> Vec<RefMut<ComponentContainer>> {
        let Some(components) = self.components.get(&entity) else {
            return Vec::with_capacity(0);
        };

        components.iter().map(|rc| rc.borrow_mut()).collect()
    }

    pub fn player_ids(&self) -> Keys<'_, i32, InWorldPlayer> {
        self.in_world_players.keys()
    }

    pub fn players(&self) -> Values<'_, i32, InWorldPlayer> {
        self.in_world_players.values()
    }

    pub fn set_in_world_player_data(&mut self, in_world_player: InWorldPlayer) {
        self.in_world_players
            .insert(in_world_player.player_id, in_world_player);
    }
}
