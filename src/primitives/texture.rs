use crate::primitives::colour::Colour;

#[derive(Debug, Clone, Copy)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl Texture {
    pub fn sample(&self, u: f64, v: f64) -> Colour {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);
        let x = (u * (self.width - 1) as f64) as usize;
        let y = (v * (self.height - 1) as f64) as usize;

        let idx = (y * self.width + x) * 4;

        Colour::new(
            self.data[idx],
            self.data[idx + 1],
            self.data[idx + 2],
            self.data[idx + 3],
        )
    }
}
