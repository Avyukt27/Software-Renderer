use crate::primitives::{colour::Colour, triangle::Triangle, vertex::Vertex};
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
    pub centre: Vertex,
    pub rotate_around_pivot: bool,
    pub pivot: Option<Vertex>,
    pub colour: Colour,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            centre: Vertex::new(0.0, 0.0, 0.0),
            rotate_around_pivot: false,
            pivot: None,
            colour: Colour::new(255, 255, 255, 255),
        }
    }

    pub fn cube(centre_x: f64, centre_y: f64, centre_z: f64, size: f64, colour: &Colour) -> Self {
        let mut mesh = Self::new();
        mesh.create_cube(size);
        mesh.centre = Vertex {
            x: centre_x,
            y: centre_y,
            z: centre_z,
        };
        mesh.colour = *colour;
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
        mesh.centre = Vertex {
            x: centre_x,
            y: centre_y,
            z: centre_z,
        };
        mesh.colour = *colour;
        mesh
    }
}

impl Mesh {
    fn create_cube(&mut self, size: f64) {
        let mut vertices = vec![
            Vertex::new(-size / 2.0, -size / 2.0, size / 2.0),
            Vertex::new(size / 2.0, -size / 2.0, size / 2.0),
            Vertex::new(size / 2.0, size / 2.0, size / 2.0),
            Vertex::new(-size / 2.0, size / 2.0, size / 2.0),
            Vertex::new(-size / 2.0, -size / 2.0, -size / 2.0),
            Vertex::new(size / 2.0, -size / 2.0, -size / 2.0),
            Vertex::new(size / 2.0, size / 2.0, -size / 2.0),
            Vertex::new(-size / 2.0, size / 2.0, -size / 2.0),
        ];
        self.vertices.append(&mut vertices);

        self.triangles.extend([
            Triangle::new(0, 1, 2),
            Triangle::new(0, 2, 3),
            Triangle::new(5, 4, 7),
            Triangle::new(5, 7, 6),
            Triangle::new(4, 0, 3),
            Triangle::new(4, 3, 7),
            Triangle::new(1, 5, 6),
            Triangle::new(1, 6, 2),
            Triangle::new(3, 2, 6),
            Triangle::new(3, 6, 7),
            Triangle::new(4, 5, 1),
            Triangle::new(4, 1, 0),
        ]);
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

                self.vertices.push(Vertex { x, y, z });
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
