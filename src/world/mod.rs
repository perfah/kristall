use crate::world::entity::{Entity, EntityContainer, EntityIterator};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::world::entity::component::{Component, ComponentManager, ComponentIterator, FilteredComponentIterator};
use crate::world::entity::builder::EntityBuilder;
use crate::world::system::integrate::IntegrateSystem;
use crate::world::system::input::InputSystem;
use std::thread;
use crate::world::system::{System, start_system_in_parallel};
use crate::world::entity::prefab::cube::Cube;
use crate::world::entity::prefab::Prefab;
use crate::world::system::gravity::GravitySystem;
use cgmath::Vector3;
use crate::world::entity::component::camera::Camera;
use crate::world::entity::component::transform::Transform;
use crate::world::entity::prefab::car::Car;
use crate::backend::BackendProxy;


pub mod entity;
pub mod system;

pub struct World {
    root: Entity,
}

impl World {
    pub fn new<T: Prefab>(prefab: T, backend_proxy: &BackendProxy) -> Self {
        let world_builder = prefab.instantiate(backend_proxy);

        let root = world_builder.build();

        start_system_in_parallel::<IntegrateSystem, Entity>(root.clone());
        start_system_in_parallel::<GravitySystem, Entity>(root.clone());
        start_system_in_parallel::<InputSystem, Entity>(root.clone());

        World { root }
    }
}

impl Clone for World {
    fn clone(&self) -> Self {
        Self {
            root: self.root.clone()
        }
    }
}

impl EntityContainer for World {
    fn query_entities(&self, include_parent: bool) -> EntityIterator {
        EntityIterator::new(self.root.clone(), None, include_parent)
    }

    fn query_entity_by_name(&self, name: &'static str, include_parent: bool) -> EntityIterator {
        EntityIterator::new(self.root.clone(), Some(name.to_string()), include_parent)
    }

    fn query_components<T: Component>(&self, include_parent: bool) -> ComponentIterator<T> {
        ComponentIterator::new(self.root.clone(), include_parent)
    }

    fn query_components_by_predicate<T: Component, F: Fn(&T) -> bool>(&self, filter_predicate: F, include_parent: bool) -> FilteredComponentIterator<T, F> {
        FilteredComponentIterator::new(self.root.clone(), filter_predicate, include_parent)
    }

    fn spawn_entity(&self, entity: Entity) {
        self.root.spawn_entity(entity);
    }
}

impl IntoIterator for World {
    type Item = Entity;
    type IntoIter = EntityIterator;

    fn into_iter(self) -> Self::IntoIter {
        unimplemented!()
    }
}
