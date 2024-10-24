use super::component::ComponentContainer;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Entity {
    pub entity_id: i32,
    pub entity_type: i32,
    pub map_id: i32,
}

pub struct EntityBuilder<'comp>(Entity, &'comp mut Vec<RefCell<ComponentContainer>>);

pub struct EntityManager {
    active_entity_set: HashMap<i32, Vec<Entity>>,
    next_id: AtomicI32,
    recycled_ids: HashMap<i32, VecDeque<i32>>,
}

impl EntityManager {
    pub fn create(&mut self, config_id: i32, entity_type: i32, map_id: i32) -> Entity {
        let entity_id = self
            .recycled_ids
            .get_mut(&config_id)
            .and_then(|ids| ids.pop_front())
            .unwrap_or_else(|| self.next_id.fetch_add(1, Ordering::Relaxed));

        let entity = Entity {
            entity_id,
            entity_type,
            map_id,
        };

        self.active_entity_set
            .entry(config_id)
            .or_default()
            .push(entity);

        entity
    }

    pub fn get_entity_id(&self, config_id: i32) -> i32 {
        self.active_entity_set
            .get(&config_id)
            .and_then(|entities| entities.first())
            .map(|entity| entity.entity_id)
            .unwrap_or_else(|| {
                tracing::error!("Entity Configuration ID {} not found.", config_id);
                -1
            })
    }

    pub fn get_config_id(&self, entity_id: i32) -> i32 {
        self.active_entity_set
            .iter()
            .find_map(|(config_id, entities)| {
                entities
                    .iter()
                    .any(|e| e.entity_id == entity_id)
                    .then_some(*config_id)
            })
            .unwrap_or_else(|| {
                tracing::error!("Entity ID {} not found.", entity_id);
                -1
            })
    }

    pub fn get(&self, config_id: i32) -> Entity {
        self.active_entity_set
            .get(&config_id)
            .and_then(|entities| entities.first())
            .cloned()
            .unwrap_or_else(|| {
                tracing::error!("Entity Configuration ID {} not found.", config_id);
                Entity::default()
            })
    }

    pub fn get_all_entity_id(&self) -> Vec<i32> {
        self.active_entity_set
            .iter()
            .flat_map(|(_, entities)| entities.iter().map(|e| e.entity_id))
            .collect()
    }

    pub fn active_entity_empty(&self) -> bool {
        self.active_entity_set.is_empty()
    }

    pub fn get_entity_ids_by_map(&self, map_id: i32) -> Vec<i32> {
        self.active_entity_set
            .iter()
            .flat_map(|(_, entities)| {
                entities
                    .iter()
                    .filter(|e| e.map_id == map_id)
                    .map(|e| e.entity_id)
            })
            .collect()
    }

    #[inline(always)]
    pub fn remove(&mut self, entity_id: i32) -> bool {
        for (config_id, entities) in self.active_entity_set.iter_mut() {
            if let Some(index) = entities.iter().position(|e| e.entity_id == entity_id) {
                let entity = entities.remove(index);
                self.recycled_ids
                    .entry(*config_id)
                    .or_default()
                    .push_back(entity.entity_id);
                return true;
            }
        }
        false
    }
}

impl<'comp> EntityBuilder<'comp> {
    pub fn builder(
        entity: Entity,
        components: &'comp mut Vec<RefCell<ComponentContainer>>,
    ) -> Self {
        Self(entity, components)
    }

    pub fn with(self, component: ComponentContainer) -> Self {
        self.1.push(RefCell::new(component));
        self
    }

    pub fn build(self) -> Entity {
        self.0
    }
}

impl From<Entity> for i64 {
    fn from(value: Entity) -> Self {
        value.entity_id as i64
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self {
            active_entity_set: HashMap::new(),
            next_id: AtomicI32::new(1),
            recycled_ids: HashMap::new(),
        }
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            entity_id: -1,
            entity_type: -1,
            map_id: 8,
        }
    }
}
