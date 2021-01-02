use std::time::Duration;

use crate::world::entity::Entity;
use crate::world::entity::component::Component;
use crate::backend::input::entity::EntityController;

pub struct Controller {
    pub input_source: Box<dyn EntityController>,
    
}

impl Controller {
    pub fn new<T: 'static + EntityController>(controller: T) -> Self {
        Self {
            input_source: Box::new(controller)
        }
    }

    pub fn update(&self, entity: &Entity, delta: Duration) {
        self.input_source.update_entity(entity, delta);
    }
}

impl Component for Controller {
    fn enabled(&self) -> bool {
        unimplemented!()
    }
}
