use crate::vertex::Vertex;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<(usize, usize)>,
    pub centre: Vertex,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            centre: Vertex::default(),
        }
    }

    pub fn cube(centre_x: f64, centre_y: f64, centre_z: f64, size: f64) -> Self {
        let mut mesh = Self::new();
        mesh.create_box(size);
        mesh.centre = Vertex {
            x: centre_x,
            y: centre_y,
            z: centre_z,
        };
        mesh
    }

    pub fn sphere(
        centre_x: f64,
        centre_y: f64,
        centre_z: f64,
        radius: f64,
        segments: usize,
    ) -> Self {
        let mut mesh = Self::new();
        mesh.create_sphere(radius, segments);
        mesh.centre = Vertex {
            x: centre_x,
            y: centre_y,
            z: centre_z,
        };
        mesh
    }
}

impl Mesh {
    fn create_box(&mut self, size: f64) {
        let mut vertices = vec![
            Vertex {
                x: -size / 2.0,
                y: -size / 2.0,
                z: size / 2.0,
            },
            Vertex {
                x: size / 2.0,
                y: -size / 2.0,
                z: size / 2.0,
            },
            Vertex {
                x: size / 2.0,
                y: size / 2.0,
                z: size / 2.0,
            },
            Vertex {
                x: -size / 2.0,
                y: size / 2.0,
                z: size / 2.0,
            },
            Vertex {
                x: -size / 2.0,
                y: -size / 2.0,
                z: -size / 2.0,
            },
            Vertex {
                x: size / 2.0,
                y: -size / 2.0,
                z: -size / 2.0,
            },
            Vertex {
                x: size / 2.0,
                y: size / 2.0,
                z: -size / 2.0,
            },
            Vertex {
                x: -size / 2.0,
                y: size / 2.0,
                z: -size / 2.0,
            },
        ];
        self.vertices.append(&mut vertices);

        let mut edges = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ];
        self.edges.append(&mut edges);
    }

    fn create_sphere(&mut self, radius: f64, segments: usize) {
        self.vertices.clear();
        self.edges.clear();

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

        for i in 0..=segments {
            for j in 0..segments {
                let current = i * ring_size + j;

                let next_j = (j + 1) % ring_size;
                let horizontal = i * ring_size + next_j;
                self.edges.push((current, horizontal));

                if i < segments {
                    let vertical = (i + 1) * ring_size + j;
                    self.edges.push((current, vertical));
                }
            }
        }
    }
}
