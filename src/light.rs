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

    pub fn to_uniform(&self) -> LightUniform {
        LightUniform {
            position: [self.position.x, self.position.y, self.position.z, 1.0],
            diffuse: [
                self.diffuse_colour.x,
                self.diffuse_colour.y,
                self.diffuse_colour.z,
                1.0,
            ],
            ambient: [
                self.ambient_colour.x,
                self.ambient_colour.y,
                self.ambient_colour.z,
                1.0,
            ],
            ambient_strength: self.ambient_strength,
            _padding: [0.0; 3],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 4],
    pub diffuse: [f32; 4],
    pub ambient: [f32; 4],
    pub ambient_strength: f32,
    pub _padding: [f32; 3],
}
