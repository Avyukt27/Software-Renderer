use crate::vertex::Vertex;

pub struct Mesh {
    vertices: Vec<Vertex>,
    edges: Vec<(usize, usize)>,
}
