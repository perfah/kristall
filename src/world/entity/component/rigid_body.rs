use crate::world::entity::component::Component;
use cgmath::Vector3;
use std::collections::HashMap;


pub struct RigidBody {
    pub mass: f32,
    pub movable: bool,
    pub last_absolute_position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub acceleration: Vector3<f32>,
    pub angular_velocity: Vector3<f32>,
    pub angular_acceleration: Vector3<f32>,

    // Internal:
    forces: HashMap<&'static str, Vector3<f32>>
}

impl RigidBody{
    pub fn new(mass: f32) -> RigidBody {
        RigidBody { 
            mass, 
            movable: true,
            last_absolute_position: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            velocity: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            acceleration: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            angular_velocity: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            angular_acceleration: Vector3 {x: 0.0, y: 0.0, z: 0.0},

            forces: HashMap::new(),
         }
    }

    pub fn commit_force(&mut self, force_desc: &'static str, mut force: Vector3<f32>) {
        if f32::is_nan(force.x) { force.x = 0.0; }
        if f32::is_nan(force.y) { force.y = 0.0; }
        if f32::is_nan(force.z) { force.z = 0.0; }

        self.forces.insert(force_desc, force);
    }

    pub fn net_force(&self) -> Vector3<f32> {
        self.forces.values().map(|force| force).sum()
    }
}

impl Component for RigidBody {
    fn enabled(&self) -> bool {
        unimplemented!()
    }
}