use crate::texture::Texture;

#[allow(dead_code)]
pub struct Material {
    pub name: String,
    pub diffuse_texture: Texture,
    pub specular_texture: Texture,
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
