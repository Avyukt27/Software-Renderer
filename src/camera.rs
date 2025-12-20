use crate::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vertex,
    pub rotation: (f32, f32, f32),
    pub zoom: f64,
}
