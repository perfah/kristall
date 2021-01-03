use crate::world::entity::builder::EntityBuilder;
use crate::backend::BackendProxy;

pub mod player;
pub mod car;
pub mod cube;
pub mod rand_tile;

pub trait Prefab {
    fn instantiate(&self, backend_proxy: &BackendProxy) -> EntityBuilder {
        self.apply(EntityBuilder::new(), backend_proxy)
    }

    fn apply(&self, builder: EntityBuilder, backend_proxy: &BackendProxy) -> EntityBuilder;
}
