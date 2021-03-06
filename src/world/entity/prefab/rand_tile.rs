use crate::world::World;
use rand_core::SeedableRng;
use crate::world::entity::EntityContainer;
use crate::world::entity::prefab::cube::Cube;
use crate::world::entity::prefab::car::Car;
use crate::world::entity::prefab::Prefab;
use cgmath::Vector3;
use crate::world::entity::builder::EntityBuilder;
use crate::world::entity::component::ComponentManager;
use crate::world::entity::component::transform::Transform;
use crate::world::entity::component::camera::{Camera, CameraPerspective};
use cgmath::num_traits::real::Real;
use crate::world::entity::prefab::player::Player;
use crate::world::entity::component::rigid_body::RigidBody;
use crate::backend::BackendProxy;

const N: usize = 101;
pub struct RandomTileSeed(pub [u8; N]);
pub struct RandomTile(RandomTileSeed);

impl Prefab for RandomTile {
    fn apply(&self, builder: EntityBuilder, backend_proxy: &BackendProxy) -> EntityBuilder {
        let mut entities = Vec::new();

        let player = Player{}.instantiate(backend_proxy);

        entities.push(player);

        let rand_nums = (self.0).0;

        for i in 0..5 {
            for j in 0..5 {
                for k in 0..5 {
                    let cube = Cube {
                        pos: Vector3 {
                            x: 4.0 + i as f32 * 5.0,
                            y: rand_nums[i+j] as f32 / 100.0 + k as f32 * 10f32,
                            z: 4.0 + j as f32 * 10.0
                        },
                        mass: 9000000.0,
                        rot: false,
                    };

                    println!("Entity pos = {:?}", cube.pos);

                    entities.push(cube.instantiate(backend_proxy));
                }
            }
        }

        builder
            .with_name("RandomTile world")
            .with_children(entities)
    }
}

impl Default for RandomTileSeed {
    fn default() -> RandomTileSeed {
        RandomTileSeed([0; N])  
    }
}

impl AsMut<[u8]> for RandomTileSeed {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl SeedableRng for RandomTile {
    type Seed = RandomTileSeed;

    fn from_seed(seed: RandomTileSeed) -> RandomTile {
        RandomTile(seed)
    }
}