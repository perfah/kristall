use cgmath::Vector3;

use super::Prefab;
use crate::world::entity::component::model::GraphicsModel;
use crate::world::entity::builder::EntityBuilder;
use crate::world::entity::component::transform::Transform;
use crate::world::entity::component::rigid_body::RigidBody;
use crate::backend::BackendProxy;

pub struct Cube {
    pub pos: Vector3<f32>,
    pub mass: f32,
    pub rot: bool,
}

impl Prefab for Cube {
    fn apply(&self, builder: EntityBuilder, backend_proxy: &BackendProxy) -> EntityBuilder {
        let mut builder = builder
            .with_name("cubeyboi")
            .with_component(Transform::new().with_position(self.pos.clone()))
            .with_component(GraphicsModel::new("/home/perfah/Programming/kristall/res/model/cube.obj", backend_proxy));

        if self.mass > 0.0 {
            builder = builder.
                with_component(RigidBody::new(self.mass));
        }

        builder
    }
}
