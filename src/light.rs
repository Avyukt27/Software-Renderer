pub struct Light {
    position: glam::Vec3,
    diffuse_colour: glam::Vec3,
    ambient_colour: glam::Vec3,
    ambient_strength: f32,
}

impl Light {
    pub fn new(
        position: glam::Vec3,
        diffuse_colour: glam::Vec3,
        ambient_colour: glam::Vec3,
        ambient_strength: f32,
    ) -> Self {
        Self {
            position,
            diffuse_colour,
            ambient_colour,
            ambient_strength,
        }
    }
}

impl Into<LightUniform> for &Light {
    fn into(self) -> LightUniform {
        LightUniform {
            position: [self.position.x, self.position.y, self.position.z],
            _pad1: 0.0,
            diffuse: [
                self.diffuse_colour.x,
                self.diffuse_colour.y,
                self.diffuse_colour.z,
            ],
            _pad2: 0.0,
            ambient: [
                self.ambient_colour.x,
                self.ambient_colour.y,
                self.ambient_colour.z,
            ],
            ambient_strength: self.ambient_strength,
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
    pub ambient: [f32; 3],
    pub ambient_strength: f32,
}
