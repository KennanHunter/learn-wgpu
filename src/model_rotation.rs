use cgmath::{SquareMatrix, Vector4, Zero};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelRotationState {
    pub rotation: f32,
}

impl ModelRotationState {
    const ROTATION_STEP: f32 = 0.1;

    fn build_rotation_matrix(&self) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4 {
            x: Vector4::zero(),
            y: Vector4::zero(),
            z: Vector4::zero(),
            // todo: multiply by self.rotation?
            w: Vector4::unit_x(),
        }
    }

    fn apply_rotation(&mut self) {
        self.rotation += Self::ROTATION_STEP;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelRotationUniform {
    view_proj: [[f32; 4]; 4],
}

impl ModelRotationUniform {
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_rotation_proj(&mut self, model: &ModelRotationState) {
        self.view_proj = model.build_rotation_matrix().into();
    }
}
