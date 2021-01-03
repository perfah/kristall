use crate::world::system::{System, SysEnvComponentMut, SystemRuntimeError, SysEnvComponent};
use crate::world::entity::component::{Component, ComponentManager, ComponentWriteAccess};
use crate::world::entity::component::transform::Transform;
use crate::world::entity::component::model::GraphicsModel;
use crate::world::entity::{Entity, EntityContainer, EntityIterator};
use std::sync::Arc;
use crate::world::World;
use std::ops::DerefMut;
use std::time::Duration;
use cgmath::Vector3;
use futures::StreamExt;
use crate::world::entity::component::rigid_body::RigidBody;

pub struct TranslateSystem {
    iter: Option<EntityIterator>
}

impl<'a> System<'a> for TranslateSystem {
    type Environment = &'a mut Option<EntityIterator>;

    fn new() -> Self{
        Self { iter: None }
    }

    fn on_fetch<T: EntityContainer>(&mut self, source: &T) -> Result<(), SystemRuntimeError>{
        self.iter = Some(source.clone().query_entities(true));
        Result::Ok(())
    }

    fn on_freeze(&'a self) -> Result<Self::Environment, SystemRuntimeError> {
        Result::Ok(&mut self.iter)
    }

    fn on_run(&self, environment: Self::Environment, delta: Duration) {
        let delta = delta.as_secs_f32();

        
        

    }
}
