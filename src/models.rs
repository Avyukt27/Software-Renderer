use crate::texture::Texture;

#[allow(dead_code)]
pub struct Material {
    pub name: String,
    pub diffuse_texture: Texture,
    pub specular_texture: Texture,
    // TODO: Implement bump map
    // pub normal_texture: Texture,
    pub bind_group: wgpu::BindGroup,
}

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub material_name: String,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: std::collections::HashMap<String, Material>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelUniform {
    pub model_matrix: [[f32; 4]; 4],
}

impl From<glam::Mat4> for ModelUniform {
    fn from(value: glam::Mat4) -> Self {
        Self {
            model_matrix: value.to_cols_array_2d(),
        }
    }
}

pub struct Object {
    pub model: Model,
    pub position: glam::Vec3,
    pub rotation: glam::Vec3,
    pub scale: glam::Vec3,
    pub model_buffer: wgpu::Buffer,
    pub model_bind_group: wgpu::BindGroup,
}

impl Object {
    pub fn compute_matrix(&self) -> glam::Mat4 {
        let translation = glam::Mat4::from_translation(self.position);
        let rotation = glam::Mat4::from_rotation_x(self.rotation.x)
            * glam::Mat4::from_rotation_y(self.rotation.y)
            * glam::Mat4::from_rotation_z(self.rotation.z);
        let scale = glam::Mat4::from_scale(self.scale);

        translation * rotation * scale
    }
}
