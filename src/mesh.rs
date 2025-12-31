use crate::primitives::{colour::Colour, texture::Texture, triangle::Triangle, vertex::Vertex};
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
    pub centre: Vertex,
    pub rotate_around_pivot: bool,
    pub pivot: Option<Vertex>,
    pub texture: Option<Texture>,
    pub colour: Option<Colour>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            centre: Vertex::new(0.0, 0.0, 0.0, 0.0, 0.0),
            rotate_around_pivot: false,
            pivot: None,
            texture: None,
            colour: Some(Colour::new(255, 255, 255, 255)),
        }
    }

    pub fn cube(centre_x: f64, centre_y: f64, centre_z: f64, size: f64, colour: &Colour) -> Self {
        let mut mesh = Self::new();
        mesh.create_cube(size);
        mesh.centre = Vertex::new(centre_x, centre_y, centre_z, 0.0, 0.0);
        mesh.colour = Some(*colour);
        mesh
    }

    pub fn sphere(
        centre_x: f64,
        centre_y: f64,
        centre_z: f64,
        radius: f64,
        segments: usize,
        colour: &Colour,
    ) -> Self {
        let mut mesh = Self::new();
        mesh.create_sphere(radius, segments);
        mesh.centre = Vertex::new(centre_x, centre_y, centre_z, 0.0, 0.0);
        mesh.colour = Some(*colour);
        mesh
    }
}

impl Mesh {
    fn create_cube(&mut self, size: f64) {
        let half_size = size / 2.0;
        let start = self.vertices.len();

        // Front face
        self.vertices.extend([
            Vertex::new(-half_size, -half_size, half_size, 0.0, 1.0),
            Vertex::new(half_size, -half_size, half_size, 1.0, 1.0),
            Vertex::new(half_size, half_size, half_size, 1.0, 0.0),
            Vertex::new(-half_size, half_size, half_size, 0.0, 0.0),
        ]);

        // Back face
        self.vertices.extend([
            Vertex::new(half_size, -half_size, -half_size, 0.0, 1.0),
            Vertex::new(-half_size, -half_size, -half_size, 1.0, 1.0),
            Vertex::new(-half_size, half_size, -half_size, 1.0, 0.0),
            Vertex::new(half_size, half_size, -half_size, 0.0, 0.0),
        ]);

        // Left face
        self.vertices.extend([
            Vertex::new(-half_size, -half_size, -half_size, 0.0, 1.0),
            Vertex::new(-half_size, -half_size, half_size, 1.0, 1.0),
            Vertex::new(-half_size, half_size, half_size, 1.0, 0.0),
            Vertex::new(-half_size, half_size, -half_size, 0.0, 0.0),
        ]);

        // Right face
        self.vertices.extend([
            Vertex::new(half_size, -half_size, half_size, 0.0, 1.0),
            Vertex::new(half_size, -half_size, -half_size, 1.0, 1.0),
            Vertex::new(half_size, half_size, -half_size, 1.0, 0.0),
            Vertex::new(half_size, half_size, half_size, 0.0, 0.0),
        ]);

        // Top face
        self.vertices.extend([
            Vertex::new(-half_size, half_size, half_size, 0.0, 1.0),
            Vertex::new(half_size, half_size, half_size, 1.0, 1.0),
            Vertex::new(half_size, half_size, -half_size, 1.0, 0.0),
            Vertex::new(-half_size, half_size, -half_size, 0.0, 0.0),
        ]);

        // Bottom face
        self.vertices.extend([
            Vertex::new(-half_size, -half_size, -half_size, 0.0, 1.0),
            Vertex::new(half_size, -half_size, -half_size, 1.0, 1.0),
            Vertex::new(half_size, -half_size, half_size, 1.0, 0.0),
            Vertex::new(-half_size, -half_size, half_size, 0.0, 0.0),
        ]);

        for i in 0..6 {
            let base = start + i * 4;
            self.triangles.extend([
                Triangle::new(base, base + 1, base + 2),
                Triangle::new(base, base + 2, base + 3),
            ]);
        }
    }

    fn create_sphere(&mut self, radius: f64, segments: usize) {
        self.vertices.clear();
        self.triangles.clear();

        for i in 0..=segments {
            let theta = i as f64 * PI / segments as f64;

            for j in 0..segments {
                let phi = j as f64 * 2.0 * PI / segments as f64;

                let x = radius * theta.sin() * phi.cos();
                let y = radius * theta.cos();
                let z = radius * theta.sin() * phi.sin();

                self.vertices.push(Vertex::new(x, y, z, 0.0, 0.0));
            }
        }

        let ring_size = segments;

        for i in 0..segments {
            let current_ring = i * ring_size;
            let next_ring = (i + 1) * ring_size;

            for j in 0..ring_size {
                let next_j = (j + 1) % ring_size;

                let a = current_ring + j;
                let b = current_ring + next_j;
                let c = next_ring + j;
                let d = next_ring + next_j;

                self.triangles.push(Triangle::new(a, c, b));
                self.triangles.push(Triangle::new(b, c, d));
            }
        }
    }
}
