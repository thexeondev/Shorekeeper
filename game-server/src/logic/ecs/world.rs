use super::component::ComponentContainer;
use super::entity::{Entity, EntityBuilder, EntityManager};
use crate::logic::player::InWorldPlayer;
use std::cell::{RefCell, RefMut};
use std::collections::hash_map::{Keys, Values};
use std::collections::HashMap;

pub struct WorldEntity {
    components: HashMap<i32, Vec<RefCell<ComponentContainer>>>,
    entity_manager: EntityManager,
}

pub struct World {
    pub player_cur_map_id: i32,
    pub world_entitys: HashMap<i32, WorldEntity>, // i32 -> map_id
    pub in_world_players: HashMap<i32, InWorldPlayer>, // joined players metadata
}

impl World {
    pub fn new() -> Self {
        Self {
            player_cur_map_id: 8,
            world_entitys: HashMap::new(),
            in_world_players: HashMap::new(),
        }
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

    pub fn get_mut_world_entity(&mut self) -> &mut WorldEntity {
        self.world_entitys
            .get_mut(&self.player_cur_map_id)
            .unwrap_or_else(|| panic!("Failed to get cur map data: {}", self.player_cur_map_id))
    }

    pub fn get_world_entity(&self) -> &WorldEntity {
        self.world_entitys
            .get(&self.player_cur_map_id)
            .unwrap_or_else(|| panic!("Failed to get cur map data: {}", self.player_cur_map_id))
    }
}

impl WorldEntity {
    pub fn create_entity(
        &mut self,
        config_id: i32,
        entity_type: i32,
        map_id: i32,
    ) -> EntityBuilder {
        let entity = self.entity_manager.create(config_id, entity_type, map_id);
        EntityBuilder::builder(
            entity,
            self.components
                .entry(entity.entity_id)
                .or_insert(Vec::new()),
        )
    }

    pub fn is_in_all_world_map(&self, entity_id: i32) -> bool {
        self.entity_manager.get_all_entity_id().contains(&entity_id)
    }

    pub fn is_in_world_map(&self, entity_id: i32, map_id: i32) -> bool {
        self.entity_manager
            .get_entity_ids_by_map(map_id)
            .contains(&entity_id)
    }

    pub fn get_entity_id(&self, config_id: i32) -> i64 {
        self.entity_manager.get_entity_id(config_id) as i64
    }

    pub fn get_config_id(&self, entity_id: i32) -> i32 {
        self.entity_manager.get_config_id(entity_id)
    }

    pub fn get_entity(&self, config_id: i32) -> Entity {
        self.entity_manager.get(config_id)
    }

    pub fn components(&self) -> &HashMap<i32, Vec<RefCell<ComponentContainer>>> {
        &self.components
    }

    pub fn get_entity_components(&self, entity_id: i32) -> Vec<RefMut<ComponentContainer>> {
        if let Some(components) = self.components.get(&entity_id) {
            components.iter().map(|rc| rc.borrow_mut()).collect()
        } else {
            Vec::new()
        }
    }

    pub fn remove_entity(&mut self, entity_id: i32) -> bool {
        self.components.remove(&entity_id).is_some() && self.entity_manager.remove(entity_id)
    }

    pub fn active_entity_empty(&self) -> bool {
        self.entity_manager.active_entity_empty()
    }
}

impl Default for WorldEntity {
    fn default() -> Self {
        Self {
            components: HashMap::new(),
            entity_manager: EntityManager::default(),
        }
    }
}
