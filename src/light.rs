pub struct Light {
    pub position: glam::Vec3,
}

impl Light {
    pub fn new(position: glam::Vec3) -> Self {
        Self { position }
    }

    pub fn to_uniform(&self) -> LightUniform {
        LightUniform {
            position: [self.position.x, self.position.y, self.position.z],
            _padding: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    pub _padding: f32,
}
