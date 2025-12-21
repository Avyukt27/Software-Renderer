use crate::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vertex,
    pub rotation: (f32, f32, f32),
    pub fov: f64,
    pub near: f64,
    pub far: f64,
    pub screen_width: usize,
    pub screen_height: usize,
}

impl Camera {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            position: Vertex {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: (0.0, 0.0, 0.0),
            fov: 90.0,
            near: 0.1,
            far: 1000.0,
            screen_width: screen_width,
            screen_height: screen_height,
        }
    }
}
