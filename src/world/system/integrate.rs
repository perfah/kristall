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

pub struct IntegrateSystem {
    components: Vec<(ComponentManager<Transform>, ComponentManager<RigidBody>)>
}

impl<'a> System<'a> for IntegrateSystem {
    type Environment = 
        Vec<(SysEnvComponentMut<'a, Transform>,
             SysEnvComponentMut<'a, RigidBody>)>
    ;

    fn new() -> Self{
        Self { components: Vec::new() }
    }

    fn on_fetch<T: EntityContainer>(&mut self, source: &T) -> Result<(), SystemRuntimeError>{
        self.components.clear();

        let mut new_components = source.query_entities(true)
            .map(|entity| (entity.component::<Transform>(), entity.component::<RigidBody>()))
            .filter(|(a, b)| a.is_some() && b.is_some())
            .map(|(a, b)| (a.unwrap(), b.unwrap()))
            .collect();

        self.components.append(&mut new_components);

        Result::Ok(())
    }

    fn on_freeze(&'a self) -> Result<Self::Environment, SystemRuntimeError> {
        Result::Ok(
            self.components
                .iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect()
        )
    }

    fn on_run(&self, environment: Self::Environment, delta: Duration) {
        let delta = delta.as_secs_f32();

        for (mut transform, mut rigid_body) in environment {
            if !rigid_body.movable { continue; }

            let Transform { ref mut position, ref mut angular_rotation, ..} = *transform;

            let net_force = rigid_body.net_force();

            let RigidBody { ref mut velocity, 
                            ref mut acceleration, 
                            ref mut angular_velocity, 
                            ref mut angular_acceleration, mass, ..} = *rigid_body;

            *position += *velocity * delta;
            *velocity += *acceleration * delta;
            *acceleration = net_force / mass;

            // TODO: Perhaps incorrect:
            *angular_rotation += *angular_velocity * delta;
            *angular_velocity += *angular_acceleration * delta;
        }
    }
}
