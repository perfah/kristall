use crate::world::entity::component::Component;

pub trait InputSource: Send + Sync {
    fn handle_event(&self, ) -> bool;
}

pub struct Controller {
    input_source: Box<dyn InputSource>
}

impl Controller {
    pub fn new(input_source: Box<dyn InputSource>) -> Self {
        Self {
            input_source
        }
    }
}

impl Component for Controller {
    fn enabled(&self) -> bool {
        unimplemented!()
    }
}

pub struct WASDController;

impl InputSource for WASDController {
    fn handle_event(&self) -> bool {
        unimplemented!()
    }
}