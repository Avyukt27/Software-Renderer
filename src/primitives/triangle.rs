use crate::primitives::vertex::Vertex;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    v0: Vertex,
    v1: Vertex,
    v2: Vertex,
}

impl Triangle {
    pub fn new(v0: Vertex, v1: Vertex, v2: Vertex) -> Self {
        Self { v0, v1, v2 }
    }
}
