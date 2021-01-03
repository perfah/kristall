pub mod transform;
pub mod proj;
pub mod texture;
pub mod camera;
pub mod model;

use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{Window, WindowBuilder},
};
use std::{mem, iter};
use std::sync::Arc;
use std::time::{Instant, Duration};
use model::DrawModel;
use crate::backend::graphics::transform::TransformSink;
use crate::backend::graphics::texture::Texture;
use crate::backend::graphics::camera::{Camera, CameraPerspective};
use crate::backend::graphics::proj::Uniforms;
use crate::backend::graphics::transform::ModelView;
use crate::backend::graphics::model::Vertex;
use crate::backend::graphics::model::Model;
use cgmath::{Rotation3, InnerSpace, Zero};
use crate::world::World;
use crate::world::entity::{EntityContainer, EntityIterator};
use crate::world::entity::component::transform::Transform;
use crate::world::entity::component::model::GraphicsModel;
use crate::world::entity::component::{Component, ComponentManager};
use std::collections::HashMap;
use wgpu::{BufferAddress, BindGroupLayout, BufferDescriptor, CommandEncoder, RenderPass, Device, Buffer, BindGroup, TextureView};
use wgpu::util::DeviceExt;

mod ui;

pub const WHOLE_SIZE: BufferAddress = !0;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct WGPUState {
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    render_pipeline: wgpu::RenderPipeline,
    pub texture_bind_group_layout: BindGroupLayout,
    depth_texture: Texture,
    uniforms: Uniforms,
    proj_view_ubuff: wgpu::Buffer,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
    uniform_bind_group: wgpu::BindGroup,
    size: winit::dpi::PhysicalSize<u32>
}

impl WGPUState {
    pub async fn new(window: &Window, vsync: bool) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: if vsync {wgpu::PresentMode::Fifo } else {wgpu::PresentMode::Immediate},
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        dimension: wgpu::TextureViewDimension::D2,
                        component_type: wgpu::TextureComponentType::Uint,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let mut uniforms = Uniforms::new();

        let proj_view_ubuff = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("proj_view_ubuff"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let uniform_bind_group_layout =
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
                label: Some("uniform_bind_group_layout"),
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(proj_view_ubuff.slice(..)),
            }],
            label: Some("uniform_bind_group"),
        });
        
        let depth_texture = texture::Texture::create_depth_texture(&device, &sc_desc, "depth_texture");

        let mut compiler = shaderc::Compiler::new().unwrap();

        let vs_src = include_str!("../../../res/shader/geometry.vert");
        let fs_src = include_str!("../../../res/shader/geometry.frag");

        let vs_spirv = compiler.compile_into_spirv(vs_src, shaderc::ShaderKind::Vertex, "shader.vert", "main", None).unwrap();
        let fs_spirv = compiler.compile_into_spirv(fs_src, shaderc::ShaderKind::Fragment, "shader.frag", "main", None).unwrap();

        let vs_data = wgpu::util::make_spirv(vs_spirv.as_binary_u8());
        let fs_data = wgpu::util::make_spirv(fs_spirv.as_binary_u8());

        let vs_module = device.create_shader_module(vs_data);
        let fs_module = device.create_shader_module(fs_data);

        let device = Arc::new(device);
        
        let transform_bind_group_layout = crate::backend::graphics::transform::bind_group_layout(&device);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &uniform_bind_group_layout, &transform_bind_group_layout],
                push_constant_ranges: &[],
            });
            
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilStateDescriptor::default(),
            }),
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint32,
                vertex_buffers: &[model::ModelVertex::desc()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });
        

        Self {
            adapter,
            surface,
            device,
            queue: Arc::new(queue),
            sc_desc,
            swap_chain,
            render_pipeline,
            uniform_bind_group_layout,
            uniform_bind_group,
            uniforms,
            proj_view_ubuff,
            depth_texture,
            size,
            texture_bind_group_layout
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        // must be after sc update:
        self.depth_texture =
            texture::Texture::create_depth_texture(&self.device, &self.sc_desc, "depth_texture");
    }

    pub fn update(&mut self, build_projection_matrix: cgmath::Matrix4<f32>) {
        self.uniforms.update_view_proj(build_projection_matrix);

        self.queue.write_buffer(
            &self.proj_view_ubuff,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }

    pub fn render(&mut self, graphics_cache: &HashMap<&'static str, Vec<Arc<TransformSink>>>, 
                             loaded_models: &HashMap<&'static str, Model>,
                             fps: u128) {
        
        let optional_frame = self.swap_chain.get_current_frame();
        if optional_frame.is_err() { return; }

        let frame = optional_frame.unwrap();

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.output.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.01,
                            g: 0.01,
                            b: 0.01,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            
            render_pass.set_pipeline(&self.render_pipeline);

            for model_str in graphics_cache.keys() {
                let model = loaded_models.get(model_str).unwrap();

                for transform_sink in graphics_cache.get(model_str).unwrap() {
                    render_pass.draw_model(model, &self.uniform_bind_group, &transform_sink.bind_group);
                }
            }
        }
        
        ui::text::render_text(&self.device, 
                              &self.queue,
                              &mut encoder,
                              &frame.output.view,
                              &self.sc_desc,
                              format!("FPS: {}", fps),
                              (self.sc_desc.width as f32 - 200f32, 0.0));

        self.queue.submit(iter::once(encoder.finish()));
    }
}


