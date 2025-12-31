use crate::primitives::colour::Colour;

#[derive(Debug, Clone)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Colour>,
}

impl Texture {
    pub fn sample(&self, u: f64, v: f64) -> Colour {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);
        let x = (u * (self.width - 1) as f64) as usize;
        let y = ((1.0 - v) * (self.height - 1) as f64) as usize;

        self.data[y * self.width + x]
    }
}
