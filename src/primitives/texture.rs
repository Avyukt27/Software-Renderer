use crate::primitives::colour::Colour;

#[derive(Debug, Clone)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Colour>,
}

impl Texture {
    pub fn from_file(path: &str) -> Result<Self, String> {
        let img = image::open(path).map_err(|e| e.to_string())?.to_rgba8();
        let (width, height) = img.dimensions();
        let mut pixels = Vec::with_capacity((width * height) as usize);

        for pixel in img.pixels() {
            pixels.push(Colour::new(pixel[0], pixel[1], pixel[2], pixel[3]));
        }

        Ok(Self {
            width: width as usize,
            height: height as usize,
            data: pixels,
        })
    }

    pub fn sample(&self, u: f64, v: f64) -> Colour {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);
        let v = 1.0 - v;

        let x = (u * (self.width - 1) as f64) as usize;
        let y = ((1.0 - v) * (self.height - 1) as f64) as usize;

        self.data[y * self.width + x]
    }
}

impl Texture {
    pub fn checkerboard(size: usize) -> Self {
        let mut pixels = Vec::new();

        for y in 0..size {
            for x in 0..size {
                let c = if (x / 8 + y / 8) % 2 == 0 {
                    Colour::new(255, 255, 255, 255)
                } else {
                    Colour::new(0, 0, 0, 255)
                };
                pixels.push(c);
            }
        }

        Self {
            width: size,
            height: size,
            data: pixels,
        }
    }
}
