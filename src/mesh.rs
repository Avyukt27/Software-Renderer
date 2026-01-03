use std::path::Path;

use crate::{
    loader::load_wavefront,
    primitives::{material::Material, triangle::Triangle, vector::Vec3, vertex::Vertex},
};

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
    pub centre: Vec3,
    pub rotate_around_pivot: bool,
    pub pivot: Option<Vec3>,
    pub materials: Vec<Material>,
}

impl Mesh {
    pub fn custom(obj_path: &Path, centre: Vec3) -> Self {
        let mut mesh = load_wavefront(obj_path).expect("Error reading OBJ");
        mesh.centre = centre;
        mesh
    }
}
