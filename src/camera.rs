use crate::primitives::vertex::Vertex;

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
            position: Vertex::new(0.0, 0.0, 0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0),
            fov: 90.0,
            near: 0.1,
            far: 1000.0,
            screen_width: screen_width,
            screen_height: screen_height,
        }
    }

    pub fn world_to_view(&self, world: &Vertex) -> Vertex {
        let mut x = world.x - self.position.x;
        let mut y = world.y - self.position.y;
        let mut z = world.z - self.position.z;

        let pitch = self.rotation.0;
        let yaw = self.rotation.1;

        let cosy = yaw.cos() as f64;
        let siny = yaw.sin() as f64;

        let xz = cosy * x + siny * z;
        let zz = -siny * x + cosy * z;

        x = xz;
        z = zz;

        let cosp = pitch.cos() as f64;
        let sinp = pitch.sin() as f64;

        let yz = cosp * y - sinp * z;
        let zz = sinp * y + cosp * z;

        y = yz;
        z = zz;

        Vertex::new(x, y, z, world.u, world.v)
    }

    pub fn project_perspective(&self, world: &Vertex) -> Option<Vertex> {
        let v = self.world_to_view(world);

        if v.z <= self.near || v.z >= self.far {
            return None;
        }

        let aspect = self.screen_width as f64 / self.screen_height as f64;
        let f = 1.0 / (self.fov.to_radians() * 0.5).tan();

        let x = (v.x * f / aspect) / v.z;
        let y = (v.y * f) / v.z;

        Some(Vertex::new(
            x * self.screen_width as f64 * 0.5 + self.screen_width as f64 * 0.5,
            -y * self.screen_height as f64 * 0.5 + self.screen_height as f64 * 0.5,
            v.z,
            v.u,
            v.v,
        ))
    }
}
