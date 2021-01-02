use crate::world::entity::builder::EntityBuilder;

pub mod player;
pub mod car;
pub mod cube;
pub mod rand_tile;

pub trait Prefab {
    fn instantiate(&self) -> EntityBuilder {
        self.apply(EntityBuilder::new())
    }

    fn apply(&self, builder: EntityBuilder) -> EntityBuilder;
}
