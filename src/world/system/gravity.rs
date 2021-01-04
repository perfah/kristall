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
use cgmath::MetricSpace;
use cgmath::InnerSpace;

pub const G: f64 = 0.000000000067f64;

pub struct GravitySystem {
    rigid_bodies: Vec<ComponentManager<RigidBody>>
}

impl<'a> System<'a> for GravitySystem {
    type Environment = Vec<SysEnvComponentMut<'a, RigidBody>>;

    fn new() -> Self{
        Self {
            rigid_bodies: Vec::new()
        }
    }

    fn on_fetch<T: EntityContainer>(&mut self, source: &T) -> Result<(), SystemRuntimeError>{
        let iter = source
            .query_entities(true)
            .map(|entity| entity.component::<RigidBody>())
            .filter(|a| a.is_some())
            .map(|a| a.unwrap());

        for rigid_body in iter {
            self.rigid_bodies.push(rigid_body);
        }
        
        Result::Ok(())
    }

    fn on_freeze(&'a self) -> Result<Self::Environment, SystemRuntimeError> {
        Result::Ok(
            self.rigid_bodies
                .iter()
                .map(|mgr| mgr.into())
                .collect(),
        )
    }

    fn on_run(&self, mut rigid_bodies: Self::Environment, _delta: Duration) {
        for i in 0..rigid_bodies.len() {
            for j in (i+1)..rigid_bodies.len() {
                if rigid_bodies.get(i).unwrap().mass <= 0f32 || 
                   rigid_bodies.get(j).unwrap().mass <= 0f32 {
                    continue;
                }

                let pos_i = rigid_bodies.get(i).unwrap().last_absolute_position;
                let pos_j = rigid_bodies.get(j).unwrap().last_absolute_position;
                let dist_i_to_j = pos_j - pos_i;
                let dist_j_to_i = dist_i_to_j * -1.0;
                let mass_i = rigid_bodies.get_mut(i).unwrap().mass as f64;
                let mass_j = rigid_bodies.get_mut(j).unwrap().mass as f64;

                let r = dist_i_to_j.magnitude().abs() as f64;
                
                let force = G * (mass_i * mass_j) / (r as f64 * r as f64);

                // Apply forces:
                rigid_bodies.get_mut(i).unwrap().commit_force("gravity", dist_i_to_j.normalize() * force as f32);                
                rigid_bodies.get_mut(j).unwrap().commit_force("gravity", dist_j_to_i.normalize() * force as f32);
                
            }
        }

    }
}
