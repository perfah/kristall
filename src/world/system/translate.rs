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
    components: Vec<(ComponentManager<Transform>, Option<ComponentManager<RigidBody>>)>
}

impl<'a> System<'a> for TranslateSystem {
    type Environment = 
        Vec<(SysEnvComponentMut<'a, Transform>,
             Option<SysEnvComponent<'a, RigidBody>>)>
    ;

    fn new() -> Self{
        Self { components: Vec::new() }
    }

    fn on_fetch<T: EntityContainer>(&mut self, source: &T) -> Result<(), SystemRuntimeError>{
        self.components.clear();

        let mut new_components = source.query_entities(true)
            .map(|entity| (entity.component::<Transform>(), entity.component::<RigidBody>()))
            .filter(|(a,_)| a.is_some())
            .map(|(a,b)| (a.unwrap(), b))
            .collect();

        self.components.append(&mut new_components);

        Result::Ok(())
    }

    fn on_freeze(&'a self) -> Result<Self::Environment, SystemRuntimeError> {
        Result::Ok(
            self.components
                .iter()
                .map(|(a, b)| (
                    a.into(),
                    if let Some(c) = b { Some(c.into()) } else { None }
                ))
                .collect()
        )
    }

    fn on_run(&self, environment: Self::Environment, delta: Duration) {
        let delta = delta.as_secs_f32();

        for (mut transform, opt_rigid_body) in environment {
            let Transform {
                ref mut pos,
                ref mut vel,
                ref mut acc,
                ref mut rot,
                ref mut rot_vel ,
                ref mut rot_acc, ..} = *transform;

            *pos += *vel * delta;
            *vel += *acc * delta;

            *rot += *rot_vel * delta;
            *rot_vel += *rot_acc * delta;

            if let Some(ref rigid_body) = opt_rigid_body {
                *acc = rigid_body.net_force();
            }

            transform.flush();
        }
    }
}
