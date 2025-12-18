use crate::vertex::Vertex;

#[derive(Debug)]
pub struct Renderer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u8>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height * 4],
        }
    }

    pub fn clear(&mut self, red: u8, green: u8, blue: u8, alpha: u8) {
        for px in self.buffer.chunks_exact_mut(4) {
            px[0] = red;
            px[1] = green;
            px[2] = blue;
            px[3] = alpha;
        }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, red: u8, green: u8, blue: u8, alpha: u8) {
        if x >= self.width || y >= self.height {
            return;
        }

        let idx = (y * self.width + x) * 4;
        self.buffer[idx] = red;
        self.buffer[idx + 1] = green;
        self.buffer[idx + 2] = blue;
        self.buffer[idx + 3] = alpha;
    }

    pub fn put_circle(
        &mut self,
        centre_x: usize,
        centre_y: usize,
        radius: usize,
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    ) {
        let radius_sq = (radius * radius) as isize;

        for dy in -(radius as isize)..=(radius as isize) {
            for dx in -(radius as isize)..=(radius as isize) {
                if dx * dx + dy * dy <= radius_sq {
                    let x = centre_x as isize + dx;
                    let y = centre_y as isize + dy;

                    if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
                        self.put_pixel(x as usize, y as usize, red, green, blue, alpha);
                    }
                }
            }
        }
    }

    pub fn draw_vertex(&mut self, vertex: &Vertex) {
        if let Some((x, y)) = self.project(vertex) {
            self.put_circle(x, y, 5, 255, 255, 255, 255);
        }
    }

    pub fn project(&self, vertex: &Vertex) -> Option<(usize, usize)> {
        if vertex.z <= 0.0 {
            return None;
        }

        let fov = 90.0_f32.to_radians();
        let f = 1.0 / (fov / 2.0).tan() as f64;

        let x_ndc = (vertex.x * f) / vertex.z;
        let y_ndc = (vertex.y * f) / vertex.z;

        let x_screen = ((x_ndc + 1.0) * 0.5 * self.width as f64) as isize;
        let y_screen = ((1.0 - y_ndc) * 0.5 * self.height as f64) as isize;

        if x_screen < 0
            || y_screen < 0
            || x_screen >= self.width as isize
            || y_screen >= self.height as isize
        {
            return None;
        }

        Some((x_screen as usize, y_screen as usize))
    }
}
