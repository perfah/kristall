use std::any::Any;
use std::sync::{MutexGuard, Mutex, Arc, RwLockWriteGuard, RwLock, RwLockReadGuard};
use crate::world::entity::{Entity, EntityContainer, EntityIterator};
use std::marker::PhantomData;
use std::ops::{DerefMut, Deref};
use downcast_rs::Downcast;

pub mod camera;
pub mod transform;
pub mod model;
pub mod rigid_body;
pub mod controller;

enum ComponentMask {
    Tag,
    EntityName,
    Type
}

pub trait Component: Downcast + Any + Send + Sync {
    fn enabled(&self) -> bool;
}



impl_downcast!(Component);

impl Clone for Box<dyn Component>{
    fn clone(&self) -> Self {
        self.clone()
    }
}

#[derive(Clone)]
pub struct ComponentManager<C: Component>{
    inner: Arc<RwLock<Box<dyn Component>>>,
    phantom: PhantomData<C>
}

impl<C: Component> ComponentManager<C>{
    pub fn init(inner: Arc<RwLock<Box<dyn Component>>>) -> ComponentManager<C>{
        ComponentManager {
            inner,
            phantom: PhantomData
        }
    }

    pub fn peek<F, R>(&self, f: F) -> Option<R>
        where F: Fn(&C) -> R{

        if let Ok(lock) = self.inner.read() {
            let comp: &C = lock.downcast_ref().unwrap();
            Some(f(comp))
        }
        else{
            None
        }
    }

    pub fn peek_mut<F, R>(&self, mut f: F) -> Option<R>
        where F: FnMut(&mut C) -> R{

        if let Ok(mut lock) = self.inner.write() {
            let comp: &mut C = lock.downcast_mut().unwrap();

            Some(f(comp))
        }
        else{
            None
        }
    }

    pub fn lock_component_for_read<'a>(&'a self) -> ComponentReadAccess<'a, C>{
        ComponentReadAccess {
            guard: self.inner.read().unwrap(),
            phantom: PhantomData
        }
    }

    pub fn lock_component_for_write<'a>(&'a self) -> ComponentWriteAccess<'a, C>{
        ComponentWriteAccess {
            guard: self.inner.write().unwrap(),
            phantom: PhantomData
        }
    }
}

pub struct ComponentReadAccess<'a, C: Component>{
    guard: RwLockReadGuard<'a, Box<dyn Component>>,
    phantom: PhantomData<C>
}

impl<'a, C: Component> Deref for ComponentReadAccess<'a, C>{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        self.guard.deref().downcast_ref().unwrap()
    }
}

impl<'a, C: Component> From<&'a ComponentManager<C>> for ComponentReadAccess<'a, C>{
    fn from(manager: &'a ComponentManager<C>) -> Self {
        manager.lock_component_for_read()
    }
}

// ComponentWriteAccess
// ####################

pub struct ComponentWriteAccess<'a, C: Component>{
    guard: RwLockWriteGuard<'a, Box<dyn Component>>,
    phantom: PhantomData<C>
}

impl<'a, C: Component> Deref for ComponentWriteAccess<'a, C>{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        unsafe {
            self.guard.deref().downcast_ref().unwrap()
        }
    }
}

impl<'a, C: Component> DerefMut for ComponentWriteAccess<'a, C>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.deref_mut().downcast_mut().unwrap()
    }
}

impl<'a, C: Component> From<&'a ComponentManager<C>> for ComponentWriteAccess<'a, C>{
    fn from(accessor: &'a ComponentManager<C>) -> Self {
        accessor.lock_component_for_write()
    }
}

pub struct ComponentIterator<T: Component> {
    component: PhantomData<T>,
    entity: Entity,
    entity_iter: EntityIterator,
}

impl<T: Component> ComponentIterator<T> {
    pub fn new(entity: Entity, include_parent: bool) -> Self {
        Self {
            component: PhantomData,
            entity: entity.clone(),
            entity_iter: EntityIterator::new(entity.clone(), None, include_parent, false)
        }
    }

    pub fn reset(&mut self){
        self.entity_iter.reset();
    }
}

impl<T: Component> Iterator for ComponentIterator<T> {
    type Item = ComponentManager<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ref entity) = self.entity_iter.next() {
            if let Some(candidate) = entity.component() {
                return Some(candidate);
            }
        }

        return None;
    }
}


pub struct FilteredComponentIterator<T, F> where T: Component, F: Fn(&T) -> bool {
    iter: ComponentIterator<T>,
    predicate: F
}

impl<T, F> FilteredComponentIterator<T, F> where T: Component, F: Fn(&T) -> bool {
    pub fn new(entity: Entity, predicate: F, include_parent: bool) -> Self {
        Self {
            iter: ComponentIterator::new(entity, include_parent),
            predicate
        }
    }

    pub fn reset(&mut self){
        self.iter.reset();
    }
}

impl<T, F> Iterator for FilteredComponentIterator<T, F> where T: Component, F: Fn(&T) -> bool {
    type Item = ComponentManager<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mgr) = self.iter.next() {

            if mgr.peek(&self.predicate).map_or(false, |result| result) {
                return Some(mgr);
            }
        }

        return None;
    }
}