
use std::sync::Arc;
use std::mem::size_of;
use wgpu::{Buffer, BindGroup, BindGroupLayout, Queue, Device};

pub struct TransformSink {
    uniform_buffer: Buffer,
    pub bind_group: BindGroup,
    queue: Arc<Queue>
}

impl TransformSink {
    pub fn new(id: &'static str, device: Arc<Device>, queue: Arc<Queue>, bind_group_layout: &BindGroupLayout) -> TransformSink {
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: size_of::<ModelView>() as u64,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..)),
            }],
            label: Some("uniform_bind_group"),
        });

        TransformSink { uniform_buffer, bind_group, queue: queue.clone() }
    }

    pub fn update(&self, model_view: ModelView) {
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(vec![model_view].as_slice()));
    }
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModelView {
    #[allow(dead_code)]
    pub model: [[f32; 4]; 4],
}

unsafe impl bytemuck::Pod for ModelView {}
unsafe impl bytemuck::Zeroable for ModelView {}

pub fn bind_group_layout(device: &Arc<Device>) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer {
                    dynamic: false,
                    min_binding_size: None,
                },
                count: None,
            }
        ],
        label: Some("transform_bind_group_layout")
    })
}