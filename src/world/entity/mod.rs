use crate::world::entity::component::{Component, ComponentManager, ComponentIterator, FilteredComponentIterator};
use crate::world::entity::component::transform::Transform;

pub mod component;
pub mod prefab;
pub mod builder;

use std::any::{Any, TypeId};
use std::sync::{RwLock, RwLockWriteGuard, Arc, Mutex};
use std::collections::HashMap;
use std::slice::Iter;
use std::error::Error;
use std::fmt::Display;
use failure::_core::fmt::Formatter;
use std::marker::PhantomData;
use std::ops::Deref;
use std::fmt;

pub struct Entity {
    pub enabled: Arc<Mutex<bool>>,
    pub invalidated: Arc<Mutex<bool>>,
    pub name: Arc<Mutex<String>>,
    children: Arc<Mutex<Vec<Entity>>>,
    components: Arc<HashMap<TypeId, Arc<RwLock<Box<dyn Component>>>>>
}

impl Entity{
    pub fn new(name: &'static str) -> Entity {
        Entity{
            enabled: Arc::new(Mutex::new(true)),
            invalidated: Arc::new(Mutex::new(false)),
            name: Arc::new(Mutex::new(name.to_string())),
            children: Arc::new(Mutex::new(Vec::new())),
            components: Arc::new(HashMap::new())
        }
    }

    pub fn component<C: Component>(&self) -> Option<ComponentManager<C>> {
        self.components
            .get(&TypeId::of::<C>())
            .map_or(
                None,
                |component| Some(ComponentManager::init(component.clone()))
            )
    }
}

impl Clone for Entity {
    fn clone(&self) -> Self {
        Entity {
            enabled: self.enabled.clone(),
            invalidated: self.invalidated.clone(),
            name: self.name.clone(),
            children: self.children.clone(),
            components: self.components.clone()
        }
    }
}

impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Entity '{}' ({} children) (has component: {:?})", self.name.lock().unwrap(), self.children.lock().unwrap().len(), self.components.keys())
    }
}

pub trait EntityContainer: Sync + Send + Clone + IntoIterator<Item = Entity, IntoIter = EntityIterator> {
    fn query_entities(&self, include_parent: bool) -> EntityIterator;
    fn query_entity_by_name(&self, name: &'static str, include_parent: bool) -> EntityIterator;
    fn query_components<T: Component>(&self, include_parent: bool) -> ComponentIterator<T>;
    fn query_components_by_predicate<T: Component, F: Fn(&T) -> bool>(&self, filter_predicate: F, include_parent: bool) -> FilteredComponentIterator<T, F>;
    fn spawn_entity(&self, entity: Entity);
}

impl EntityContainer for Entity {
    fn query_entities(&self, include_parent: bool) -> EntityIterator {
        EntityIterator::new(self.clone(), None, include_parent)
    }

    fn query_entity_by_name(&self, name: &'static str, include_parent: bool) -> EntityIterator {
        EntityIterator::new(self.clone(), Some(name.to_string()), include_parent)
    }

    fn query_components<T: Component>(&self, include_parent: bool) -> ComponentIterator<T> {
        ComponentIterator::new(self.clone(), include_parent)
    }

    fn query_components_by_predicate<T: Component, F: Fn(&T) -> bool>(&self, filter_predicate: F, include_parent: bool) -> FilteredComponentIterator<T, F> {
        FilteredComponentIterator::new(self.clone(), filter_predicate, include_parent)
    }

    fn spawn_entity(&self, entity: Entity) {
        let mut children = self.children.lock().expect("Couldn't spawn child!");

        children.push(entity);
    }
}

impl IntoIterator for Entity {
    type Item = Entity;
    type IntoIter = EntityIterator;

    fn into_iter(self) -> Self::IntoIter {
        EntityIterator::new(self.clone(), None, false)
    }
}

pub struct EntityIterator {
    parent: Entity,
    yield_parent: bool,
    children_iter: Option<Box<dyn Iterator<Item = Entity>>>,
    child_iter: Option<Box<dyn Iterator<Item = Entity>>>,
    name_filter: Option<String>
}

impl EntityIterator {
    pub fn new(parent: Entity, name_filter: Option<String>, include_parent: bool) -> Self {
        Self {
            parent: parent.clone(),
            yield_parent: include_parent,
            children_iter: {
                let children = (*parent.children.lock().unwrap()).clone();
                Some(Box::new(children.into_iter()))
            },
            child_iter: None,
            name_filter
        }
    }

    pub fn reset(&mut self){
        unimplemented!()
    }
}

impl Iterator for EntityIterator {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = None;

        if self.yield_parent {
            next = Some(self.parent.clone());
            self.yield_parent = false;
        }

        while next.is_none(){
            if let Some(first_child) = self.child_iter
                .as_mut()
                .map_or(None, |mut iter| iter.next()) {
                let entity_ok = self.name_filter
                    .as_ref()
                    .map_or(
                        true,
                        |name_filter| name_filter.eq(first_child.name.lock().unwrap().deref())
                    );

                if entity_ok {
                    next = Some(first_child.clone());
                }
            }
            else if let Some(second_child) = self.children_iter.as_mut().unwrap().next() {
                self.child_iter = Some(Box::new(second_child.clone().query_entities(true)));
            }
            else{
                break;
            }
        }

        next
    }
}