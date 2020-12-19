use super::Prefab;
use crate::world::entity::component::model::GraphicsModel;
use crate::world::entity::builder::EntityBuilder;
use crate::world::entity::component::transform::Transform;
use crate::world::entity::component::{Component, ComponentManager};
use crate::world::entity::component::rigid_body::RigidBody;
use cgmath::Vector3;
use crate::world::entity::prefab::cube::Cube;

pub struct Player {}

impl Prefab for Player {
    fn apply(&self, builder: &mut EntityBuilder) {

        let upper = Cube{
            pos: Vector3 {x: 0.0, y: 5.0, z: 0.0},
            mass: 0.0,
            rot: false
        }.instantiate().build();

        let lower = Cube{
            pos: Vector3 {x: 0.0, y: 3.0, z: 0.0},
            mass: 10000000000000000.0,
            rot: false
        }.instantiate().build();

        builder
            .with_name("player")
            .with_child(upper)
            .with_child(lower);
    }
}
