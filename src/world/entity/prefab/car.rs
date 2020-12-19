use super::Prefab;
use crate::world::entity::component::model::GraphicsModel;
use crate::world::entity::builder::EntityBuilder;
use crate::world::entity::component::transform::Transform;
use crate::world::entity::component::{Component, ComponentManager};
use crate::world::entity::component::rigid_body::RigidBody;
use cgmath::Vector3;

pub struct Car;

impl Prefab for Car {
    fn apply(&self, builder: &mut EntityBuilder){
        builder
            .with_name("bugatti")
            .with_component(Transform::new()
                .with_position(Vector3{x: 0.0, y: 0.0, z: 0.0}))
            .with_component(RigidBody::new(10.0))
            .with_component(GraphicsModel::from("/home/perfah/Programming/kristall/res/bugatti.obj"));
    }
}

