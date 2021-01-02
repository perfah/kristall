use crate::world::entity::component::Component;
use super::prefab::Prefab;
use crate::world::entity::Entity;
use std::collections::HashMap;
use std::any::TypeId;
use std::sync::{Arc, Mutex, RwLock};
use futures::StreamExt;

pub struct EntityBuilder {
    entity: Entity,
    components: HashMap<TypeId, Arc<RwLock<Box<dyn Component>>>>
}

impl EntityBuilder{
    pub fn new() -> Self {
        EntityBuilder{
            entity: Entity::new("Unnamed"),
            components:  HashMap::new()
        }
    }

    pub fn with_name(self, name: &'static str) -> Self {
        if let Ok(mut current_name) = self.entity.name.lock() {
            *current_name = String::from(name);
        }
        self
    }

    pub fn with_child<T: Into<EntityBuilder>>(self, child: T) -> Self {
        if let Ok(mut children) = self.entity.children.lock() {
            children.push(child.into().build());
        }
        
        self
    }

    pub fn with_children<T: Into<EntityBuilder>>(self, mut children: Vec<T>) -> Self {
        let mut built_children: Vec<Entity> = children
            .drain(..)
            .map(|child| child.into().build())
            .collect::<Vec<Entity>>(); 
        
        if let Ok(mut children) = self.entity.children.lock() {
            children.append(&mut built_children);
        }

        self
    }

    pub fn with_component<C: Component>(mut self, component: C) -> Self {
        self.components.insert(TypeId::of::<C>(), Arc::new(RwLock::new(Box::new(component))));
        self
    }

    pub fn build(mut self) -> Entity {
        self.entity.components = Arc::new(self.components);
        self.entity
    }
}

impl From<Entity> for EntityBuilder {
    fn from(entity: Entity) -> Self {
        EntityBuilder{
            entity: entity.clone(),
            components: entity.components
                .iter()
                .map(|(a,b)| (a.clone(), b.clone()))
                .collect::<HashMap<_,_>>()
        }
    }
}
