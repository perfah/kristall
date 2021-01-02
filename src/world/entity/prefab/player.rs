use super::Prefab;
use crate::world::entity::component::model::GraphicsModel;
use crate::world::entity::builder::EntityBuilder;
use crate::world::entity::component::transform::Transform;
use crate::world::entity::component::camera::{Camera, CameraPerspective};
use crate::world::entity::component::{Component, ComponentManager};
use crate::world::entity::component::rigid_body::RigidBody;
use crate::world::entity::component::controller::Controller;
use crate::backend::input::entity::{WASDEntityController, InputAccelerationMethod};
use cgmath::Vector3;
use crate::world::entity::prefab::cube::Cube;

pub struct Player {}

impl Prefab for Player {
    fn apply(&self, builder: EntityBuilder) -> EntityBuilder {
        let upper = Cube{ pos: Vector3 {x: 0.0, y: 3.0, z: 0.0}, mass: 5.0, rot: false, player: true }
            .instantiate()
            .with_component(Controller::new(WASDEntityController::new(InputAccelerationMethod::Force(10f32))));

        let lower = Cube{ pos: Vector3 {x: 0.0, y: 0.0, z: 0.0}, mass: 0.0, rot: false, player: false }
            .instantiate();

        builder
            .with_name("player")
            .with_child(upper)
            .with_child(lower)
    }
}
