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
use crate::backend::BackendProxy;
use crate::backend::input::camera::MouseCameraController;

pub struct Player {}

impl Prefab for Player {
    fn apply(&self, builder: EntityBuilder, backend_proxy: &BackendProxy) -> EntityBuilder {
        let upper = Cube{ pos: Vector3 {x: 0.0, y: 3.0, z: 0.0}, mass: 5.0, rot: false }
            .instantiate(backend_proxy);

        let lower = Cube{ pos: Vector3 {x: 0.0, y: 0.0, z: 0.0}, mass: 0.0, rot: false }
            .instantiate(backend_proxy);

        builder
            .with_name("player")
            .with_child(upper)
            .with_child(lower)
            .with_component(Transform::new())
            .with_component(
                Camera::new(
                    CameraPerspective::ThirdPersonView{ distance: 25f32, angle_horiz: 0f32, angle_vert: 0f32 },
                    MouseCameraController::new(0.001f64, true)
                 )
            )
            .with_component(Controller::new(WASDEntityController::new(InputAccelerationMethod::Velocity(10f32))))
    }
}
