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

pub const G: f32 = 0.000000000067f32;

pub struct GravitySystem {
    transforms: Vec<ComponentManager<Transform>>,
    rigid_bodies: Vec<ComponentManager<RigidBody>>
}

impl<'a> System<'a> for GravitySystem {
    type Environment = (Vec<SysEnvComponent<'a, Transform>>, Vec<SysEnvComponentMut<'a, RigidBody>>);

    fn new() -> Self{
        Self {
            transforms: Vec::new(),
            rigid_bodies: Vec::new()
        }
    }

    fn on_fetch<T: EntityContainer>(&mut self, source: &T) -> Result<(), SystemRuntimeError>{
        let iter = source
            .query_entities(true)
            .map(|entity| (entity.component::<Transform>(), entity.component::<RigidBody>()))
            .filter(|(a, b)| a.is_some() && b.is_some())
            .map(|(a,b)| (a.unwrap(), b.unwrap()));

        for (transform, rigid_body) in iter {
            self.transforms.push(transform);
            self.rigid_bodies.push(rigid_body);
        }

        if self.transforms.len() == self.rigid_bodies.len() {
            Result::Ok(())
        }
        else {
            Result::Err(SystemRuntimeError("Transforms != rigid bodies"))
        }
    }

    fn on_freeze(&'a self) -> Result<Self::Environment, SystemRuntimeError> {
        Result::Ok((
            self.transforms
                .iter()
                .map(|mgr| mgr.into())
                .collect(),
            self.rigid_bodies
                .iter()
                .map(|mgr| mgr.into())
                .collect(),
        ))
    }

    fn on_run(&self, (transforms, mut rigid_bodies): Self::Environment, delta: Duration) {
        for i in 0..transforms.len() {
            for j in (i+1)..transforms.len() {
                let transform_i = transforms.get(i).unwrap();
                let transform_j = transforms.get(j).unwrap();

                let pos_i = transform_i.pos;
                let pos_j = transform_j.pos;
                let dist_i_to_j = (pos_j - pos_i);
                let dist_j_to_i = (pos_i - pos_j);
                let mass_i = rigid_bodies.get_mut(i).unwrap().mass;
                let mass_j = rigid_bodies.get_mut(j).unwrap().mass;

                let r = dist_i_to_j.magnitude().abs();

                let F = G / (mass_i * mass_j) / (r * r);

                rigid_bodies.get_mut(i).unwrap().cast_force("gravity", dist_i_to_j.normalize() * F);
                rigid_bodies.get_mut(j).unwrap().cast_force("gravity", dist_j_to_i.normalize() * F);
            }
        }

    }
}
