use super::Prefab;
use crate::world::entity::component::model::GraphicsModel;
use crate::world::entity::builder::EntityBuilder;
use crate::world::entity::component::transform::Transform;
use crate::world::entity::component::{Component, ComponentManager};
use crate::world::entity::component::rigid_body::RigidBody;
use cgmath::Vector3;

pub struct Cube {
    pub pos: Vector3<f32>,
    pub mass: f32,
    pub rot: bool
}

impl Prefab for Cube {
    fn apply(&self, builder: &mut EntityBuilder) {
        builder
            .with_name("cubeyboi")
            .with_component(Transform::new()
                .with_position(self.pos.clone()))
            .with_component(GraphicsModel::from("/home/perfah/Programming/kristall/res/model/cube.obj"))
            .with_component(RigidBody::new(self.mass));
    }
}
