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

    pub fn project_orthographic(&self, world: &Vertex) -> Vertex {
        let x = world.x - self.position.x;
        let y = world.y - self.position.y;

        Vertex {
            x: x + self.screen_width as f64 / 2.0,
            y: y + self.screen_height as f64 / 2.0,
            z: world.z,
        }
    }

    pub fn project_perspective(&self, world: &Vertex) -> Option<Vertex> {
        let x = world.x - self.position.x;
        let y = world.y - self.position.y;
        let z = world.z - self.position.z;

        if z <= self.near {
            return None;
        }

        let fov_rad = self.fov.to_radians();
        let scale = (self.screen_width as f64 / 2.0) / (fov_rad / 2.0).tan();

        let screen_x = (x * scale / z) + self.screen_width as f64 / 2.0;
        let screen_y = (-y * scale / z) + self.screen_height as f64 / 2.0;

        Some(Vertex {
            x: screen_x,
            y: screen_y,
            z,
        })
    }
}
