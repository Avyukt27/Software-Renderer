#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub u: f64,
    pub v: f64,
}

impl Vertex {
    pub fn new(x: f64, y: f64, z: f64, u: f64, v: f64) -> Self {
        Self { x, y, z, u, v }
    }

    pub fn normalize(&self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if length == 0.0 {
            return Self::new(0.0, 0.0, 0.0);
        }
        Self::new(self.x / length, self.y / length, self.z / length)
    }
}
