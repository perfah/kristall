use std::sync::Arc;
use crate::world::entity::component::{Component};
use crate::backend::graphics::model_view::ModelView;
use crate::backend::BackendProxy;

pub struct GraphicsModel {
    pub path_to_obj: &'static str,
    pub view: Arc<ModelView> 
}

impl GraphicsModel {
    pub fn new(path_to_obj: &'static str, backend_proxy: &BackendProxy) -> GraphicsModel {
        GraphicsModel {
            path_to_obj,
            view: Arc::new(backend_proxy.instantiate_model_view())
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