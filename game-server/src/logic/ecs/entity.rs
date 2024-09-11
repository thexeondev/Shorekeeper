use std::{cell::RefCell, collections::HashSet};

use super::component::ComponentContainer;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Entity(i64);

pub struct EntityBuilder<'comp>(Entity, &'comp mut Vec<RefCell<ComponentContainer>>);

#[derive(Default)]
pub struct EntityManager {
    entity_id_counter: i64,
    active_entity_set: HashSet<Entity>,
}

impl EntityManager {
    pub fn create(&mut self) -> Entity {
        self.entity_id_counter += 1;
        let entity = Entity(self.entity_id_counter);

        self.active_entity_set.insert(entity);
        entity
    }

    pub fn get(&self, id: i64) -> Option<Entity> {
        self.active_entity_set.get(&Entity(id)).copied()
    }

    #[expect(dead_code)]
    pub fn remove(&mut self, entity: Entity) -> bool {
        self.active_entity_set.remove(&entity)
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
        value.0
    }
}
