use crate::world::entity::builder::EntityBuilder;
use crate::backend::BackendProxy;

pub mod player;
pub mod car;
pub mod cube;
pub mod rand_tile;

pub trait Prefab {
    fn instantiate(&self, backend_proxy: &BackendProxy) -> EntityBuilder {
        let mut builder = EntityBuilder::new();
        self.apply(&mut builder, backend_proxy);
        builder
    }

    fn apply(&self, builder: &mut EntityBuilder, backend_proxy: &BackendProxy);
}
