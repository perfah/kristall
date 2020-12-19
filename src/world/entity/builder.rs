use crate::world::entity::component::Component;
use super::prefab::Prefab;
use crate::world::entity::Entity;
use std::collections::HashMap;
use std::any::TypeId;
use std::sync::{Arc, Mutex, RwLock};
use futures::StreamExt;

pub struct EntityBuilder {
    name: &'static str,
    children: Vec<Entity>,
    components: HashMap<TypeId, Arc<RwLock<Box<dyn Component>>>>
}

impl EntityBuilder{
    pub fn new() -> Self {
        EntityBuilder{
            name: "Unnamed",
            children: Vec::new(),
            components: HashMap::new()
        }
    }

    pub fn with_name(&mut self, name: &'static str) -> &mut Self {
        self.name = name;
        self
    }

    pub fn with_child(&mut self, child: Entity) -> &mut Self {
        self.children.push(child);
        self
    }

    pub fn with_children(&mut self, children: &mut Vec<Entity>) -> &mut Self {
        self.children.append(children);
        self
    }

    pub fn with_component<C: Component>(&mut self, component: C) -> &mut Self {
        self.components.insert(TypeId::of::<C>(), Arc::new(RwLock::new(Box::new(component))));
        self
    }

    pub fn build(&self) -> Entity {
        println!("Building entity '{} (w.{} children)", self.name, self.children.len());

        Entity {
            enabled: Arc::new(Mutex::new(true)),
            invalidated: Arc::new(Mutex::new(false)),
            name: Arc::new(Mutex::new(self.name.to_string())),
            children: Arc::new(Mutex::new(self.children.clone())),
            components: Arc::new(self.components.clone())
        }
    }
}

impl From<Entity> for EntityBuilder {
    fn from(entity: Entity) -> Self {
        Self {
            name: "",
            children: vec![],
            components: (*entity.components).clone()
        }
    }
}