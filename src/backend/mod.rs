use std::sync::Arc;
use wgpu::{Device, Queue, BindGroupLayout};

pub mod graphics;
//pub mod audio;
pub mod input;

use graphics::model_view::{ModelView, bind_group_layout};

pub struct BackendProxy {
    device: Arc<Device>,
    queue: Arc<Queue>,
    transform_bind_group_layout: BindGroupLayout
}

impl BackendProxy {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> BackendProxy {
        let transform_bind_group_layout = bind_group_layout(&device);
        BackendProxy { device, queue, transform_bind_group_layout }
    }

    pub fn instantiate_model_view(&self) -> ModelView {
        ModelView::new(self.device.clone(), self.queue.clone(), &self.transform_bind_group_layout)
    }
}
