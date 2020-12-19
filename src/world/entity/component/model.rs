use crate::world::entity::component::{Component};
use crate::util::wgpu::uniform::SpatialInfo;
use failure::_core::any::Any;

pub struct GraphicsModel {
    pub path_to_obj: &'static str
}

impl From<&'static str> for GraphicsModel {
    fn from(path_to_obj: &'static str) -> Self {
        GraphicsModel {
            path_to_obj
        }
    }
}

impl Component for GraphicsModel {
    fn enabled(&self) -> bool {
        true
    }
}

impl Clone for GraphicsModel {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}