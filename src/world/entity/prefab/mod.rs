use crate::world::entity::builder::EntityBuilder;

pub mod player;
pub mod car;
pub mod cube;
pub mod rand_tile;

pub trait Prefab {
    fn instantiate(&self) -> EntityBuilder {
        let mut builder = EntityBuilder::new();
        self.apply(&mut builder);
        builder
    }

    fn apply(&self, builder: &mut EntityBuilder);
}
