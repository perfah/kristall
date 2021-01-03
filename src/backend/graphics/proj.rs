#[repr(C)] // We need this for Rust to store our data correctly for the shaders
#[derive(Debug, Copy, Clone)] // This is so we can store this in a buffer
pub struct Uniforms {
    view_proj: [[f32; 4]; 4],
    //pub model: cgmath::Matrix4<f32>,
}

impl Uniforms {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
      //      model: cgmath::Matrix4::identity()
        }
    }

    pub fn update_view_proj(&mut self, build_projection_matrix: cgmath::Matrix4<f32>) {
        self.view_proj = build_projection_matrix.into();
    }
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}