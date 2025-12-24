use crate::math::is_back_facing;
use crate::primitives::{colour::Colour, vertex::Vertex};

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

    pub fn clear(&mut self, colour: Colour) {
        for px in self.buffer.chunks_exact_mut(4) {
            px[0] = colour.red;
            px[1] = colour.green;
            px[2] = colour.blue;
            px[3] = colour.alpha;
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

    pub fn put_circle(&mut self, centre_x: usize, centre_y: usize, radius: usize, colour: Colour) {
        let radius_sq = (radius * radius) as isize;

        for dy in -(radius as isize)..=(radius as isize) {
            for dx in -(radius as isize)..=(radius as isize) {
                if dx * dx + dy * dy <= radius_sq {
                    let x = centre_x as isize + dx;
                    let y = centre_y as isize + dy;

                    if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
                        self.put_pixel(x as usize, y as usize, colour);
                    }
                }
            }
        }
    }
}

impl Renderer {
    pub fn draw_vertex(&mut self, vertex: &Vertex) {
        let x = vertex.x as isize;
        let y = vertex.y as isize;

        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return;
        }

        self.put_circle(x as usize, y as usize, 1, Colour::new(255, 255, 255, 255));
    }

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
}
