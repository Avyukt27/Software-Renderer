pub struct Light {
    position: glam::Vec3,
    diffuse_colour: glam::Vec3,
    specular_colour: glam::Vec3,
}

impl Light {
    pub fn new(
        position: glam::Vec3,
        diffuse_colour: glam::Vec3,
        specular_colour: glam::Vec3,
    ) -> Self {
        Self {
            position,
            diffuse_colour,
            specular_colour,
        }
    }
}

impl Into<LightUniform> for &Light {
    fn into(self) -> LightUniform {
        LightUniform {
            position: self.position.to_array(),
            _pad1: 0.0,
            diffuse: self.diffuse_colour.to_array(),
            _pad2: 0.0,
            specular: self.specular_colour.to_array(),
            _pad3: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    _pad1: f32,
    pub diffuse: [f32; 3],
    _pad2: f32,
    pub specular: [f32; 3],
    _pad3: f32,
}
