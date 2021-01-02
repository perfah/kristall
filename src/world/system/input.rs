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
use crate::world::entity::component::controller::Controller;

pub struct InputSystem {
    resources: Vec<(ComponentManager<Controller>, Entity)>
}

impl<'a> System<'a> for InputSystem {
    type Environment = 
        Vec<(SysEnvComponent<'a, Controller>, Entity)>
    ;

    fn new() -> Self{
        Self { resources: Vec::new() }
    }

    fn on_fetch<T: EntityContainer>(&mut self, source: &T) -> Result<(), SystemRuntimeError>{
        self.resources.clear();

        let mut resources = source.query_entities(true)
            .map(|entity| (entity.component::<Controller>(), entity))
            .filter(|(a, _)| a.is_some())
            .map(|(a, b)| (a.unwrap(), b))
            .collect();

        self.resources.append(&mut resources);

        Result::Ok(())
    }

    fn on_freeze(&'a self) -> Result<Self::Environment, SystemRuntimeError> {
        Result::Ok(
            self.resources
                .iter()
                .map(|(a, b)| ( a.into(), b.clone() ))
                .collect()
        )
    }

    fn on_run(&self, environment: Self::Environment, delta: Duration) {
        for (ref controller, ref mut entity) in environment {
            controller.update(entity, delta);            
        }
    }
}
