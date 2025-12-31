use crate::{
    loader::load_wavefront,
    primitives::{texture::Texture, triangle::Triangle, vector::Vec3, vertex::Vertex},
};

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
    pub centre: Vec3,
    pub rotate_around_pivot: bool,
    pub pivot: Option<Vec3>,
    pub texture: Option<Texture>,
}

impl Mesh {
    pub fn custom(obj_path: &str, texture_path: Option<&str>, centre: Vec3) -> Self {
        let mut mesh = load_wavefront(obj_path).expect("Error reading OBJ");
        match texture_path {
            Some(p) => {
                mesh.texture = Some(Texture::from_file(p).expect("Error reading texture"));
            }
            None => {}
        }
        mesh.centre = centre;
        mesh
    }
}
