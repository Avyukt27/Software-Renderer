use crate::math::is_back_facing;
use crate::primitives::material::Material;
use crate::primitives::{colour::Colour, vertex::Vertex};

#[derive(Debug)]
pub struct Renderer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u8>,
    pub depth: Vec<f64>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height * 4],
            depth: vec![f64::INFINITY; width * height],
        }
    }

    pub fn clear(&mut self, colour: Colour) {
        for px in self.buffer.chunks_exact_mut(4) {
            px[0] = colour.red;
            px[1] = colour.green;
            px[2] = colour.blue;
            px[3] = colour.alpha;
        }

        for depth in self.depth.iter_mut() {
            *depth = f64::INFINITY;
        }
    }
}

impl Renderer {
    pub fn put_pixel_depth(&mut self, x: usize, y: usize, z: f64, colour: Colour) {
        if x >= self.width || y >= self.height {
            return;
        }

        let idx = y * self.width + x;

        if z < self.depth[idx] {
            self.depth[idx] = z;

            let base = idx * 4;
            self.buffer[base] = colour.red;
            self.buffer[base + 1] = colour.green;
            self.buffer[base + 2] = colour.blue;
            self.buffer[base + 3] = colour.alpha;
        }
    }

    pub fn fill_triangle(&mut self, v0: &Vertex, v1: &Vertex, v2: &Vertex, m: &Material) {
        if is_back_facing(v0, v1, v2) {
            return;
        }

        let x0 = v0.x as i32;
        let y0 = v0.y as i32;
        let x1 = v1.x as i32;
        let y1 = v1.y as i32;
        let x2 = v2.x as i32;
        let y2 = v2.y as i32;

        let min_x = x0.min(x1).min(x2).clamp(0, self.width as i32 - 1);
        let max_x = x0.max(x1).max(x2).clamp(0, self.width as i32 - 1);
        let min_y = y0.min(y1).min(y2).clamp(0, self.height as i32 - 1);
        let max_y = y0.max(y1).max(y2).clamp(0, self.height as i32 - 1);

        #[inline]
        fn edge(ax: i32, ay: i32, bx: i32, by: i32, px: i32, py: i32) -> i32 {
            (px - ax) * (by - ay) - (py - ay) * (bx - ax)
        }

        let area = edge(x0, y0, x1, y1, x2, y2);
        if area == 0 {
            return;
        }

        let u0z = v0.u / v0.z;
        let v0z = v0.v / v0.z;
        let o0z = 1.0 / v0.z;

        let u1z = v1.u / v1.z;
        let v1z = v1.v / v1.z;
        let o1z = 1.0 / v1.z;

        let u2z = v2.u / v2.z;
        let v2z = v2.v / v2.z;
        let o2z = 1.0 / v2.z;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let w0 = edge(x1, y1, x2, y2, x, y);
                let w1 = edge(x2, y2, x0, y0, x, y);
                let w2 = edge(x0, y0, x1, y1, x, y);

                if (w0 >= 0 && w1 >= 0 && w2 >= 0) || (w0 <= 0 && w1 <= 0 && w2 <= 0) {
                    let a = w0 as f64 / area as f64;
                    let b = w1 as f64 / area as f64;
                    let c = w2 as f64 / area as f64;

                    let u_over_z = a * u0z + b * u1z + c * u2z;
                    let v_over_z = a * v0z + b * v1z + c * v2z;
                    let one_over_z = a * o0z + b * o1z + c * o2z;

                    let u = u_over_z / one_over_z;
                    let v = v_over_z / one_over_z;

                    let depth = 1.0 / one_over_z;

                    let colour = if let Some(t) = &m.kd_texture {
                        t.sample(u, v)
                    } else {
                        m.diffuse
                    };

                    self.put_pixel_depth(x as usize, y as usize, depth, colour);
                }
            }
        }
    }
}
