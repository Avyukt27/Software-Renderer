use crate::primitives::vertex::Vertex;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub direction: Vertex,
    pub intensity: f64,
}

impl Light {
    pub fn new(direction: Vertex, intensity: f64) -> Self {
        Self { direction: direction.normalize(), intensity }
    }
}