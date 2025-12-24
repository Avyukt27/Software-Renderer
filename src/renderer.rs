use std::f64::INFINITY;

use crate::math::is_back_facing;
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
            depth: vec![INFINITY; width * height],
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
            *depth = INFINITY;
        }
    }
}

impl Renderer {
    pub fn put_pixel(&mut self, x: usize, y: usize, colour: Colour) {
        if x >= self.width || y >= self.height {
            return;
        }

        let idx = (y * self.width + x) * 4;
        self.buffer[idx] = colour.red;
        self.buffer[idx + 1] = colour.green;
        self.buffer[idx + 2] = colour.blue;
        self.buffer[idx + 3] = colour.alpha;
    }

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
}

impl Renderer {
    pub fn draw_edge(&mut self, vertex_1: &Vertex, vertex_2: &Vertex, colour: Colour) {
        let (mut x1, mut y1) = (vertex_1.x as isize, vertex_1.y as isize);
        let (x2, y2) = (vertex_2.x as isize, vertex_2.y as isize);

        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x1 >= 0 && y1 >= 0 && x1 < self.width as isize && y1 < self.height as isize {
                self.put_pixel(x1 as usize, y1 as usize, colour);
            }

            if x1 == x2 && y1 == y2 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x1 += sx;
            }
            if e2 <= dx {
                err += dx;
                y1 += sy;
            }
        }
    }

    pub fn draw_triangles(&mut self, a: &Vertex, b: &Vertex, c: &Vertex, colour: Colour) {
        if is_back_facing(a, b, c) {
            return;
        }

        self.draw_edge(a, b, colour);
        self.draw_edge(b, c, colour);
        self.draw_edge(c, a, colour);
    }

    pub fn fill_triangle(&mut self, v0: &Vertex, v1: &Vertex, v2: &Vertex, colour: Colour) {
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

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let w0 = edge(x1, y1, x2, y2, x, y);
                let w1 = edge(x2, y2, x0, y0, x, y);
                let w2 = edge(x0, y0, x1, y1, x, y);

                if (w0 >= 0 && w1 >= 0 && w2 >= 0) || (w0 <= 0 && w1 <= 0 && w2 <= 0) {
                    let alpha = w0 as f64 / area as f64;
                    let beta = w1 as f64 / area as f64;
                    let gamma = w2 as f64 / area as f64;

                    let depth = alpha * v0.z + beta * v1.z + gamma * v2.z;

                    self.put_pixel_depth(x as usize, y as usize, depth, colour);
                }
            }
        }
    }
}
