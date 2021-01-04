
use std::sync::Arc;
use std::mem::size_of;
use wgpu::{Buffer, BindGroup, BindGroupLayout, Queue, Device};
use crate::world::entity::component::transform::Transform;

pub struct ModelView {
    uniform_buffer: Buffer,
    pub bind_group: BindGroup,
    queue: Arc<Queue>
}

impl ModelView {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>, bind_group_layout: &BindGroupLayout) -> ModelView {
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

        ModelView { uniform_buffer, bind_group, queue: queue.clone() }
    }

    pub fn translate(&self, transform: &Transform) {
        let c = 2.0 * std::f32::consts::PI;

        let raw: [[f32; 4]; 4] = (
            cgmath::Matrix4::from_translation(transform.position) *
            cgmath::Matrix4::from_angle_x(cgmath::Rad(transform.angular_rotation.x % c)) *
            cgmath::Matrix4::from_angle_y(cgmath::Rad(transform.angular_rotation.y % c)) *
            cgmath::Matrix4::from_angle_z(cgmath::Rad(transform.angular_rotation.z % c))
        ).into();

        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(vec![raw].as_slice()));
    }
}

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