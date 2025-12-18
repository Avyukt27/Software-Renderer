use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

use pixels::{Pixels, SurfaceTexture};

use crate::renderer::Renderer;
use crate::vertex::Vertex;

#[derive(Debug)]
pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    renderer: Renderer,
    vertices: Vec<Vertex>,
    edges: Vec<(u8, u8)>,
    angles: (f32, f32, f32),
}

impl App {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            window: None,
            pixels: None,
            renderer: Renderer::new(width, height),
            vertices: vec![],
            edges: vec![],
            angles: (0.0, 0.0, 0.0),
        }
    }

    pub fn rotate_vertex(&self, vertex: &Vertex) -> Vertex {
        let center = Vertex {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        };

        let mut rotated = Vertex {
            x: vertex.x - center.x,
            y: vertex.y - center.y,
            z: vertex.z - center.z,
        };

        rotated = self.renderer.rotate_x(&rotated, self.angles.0);
        rotated = self.renderer.rotate_y(&rotated, self.angles.1);
        rotated = self.renderer.rotate_z(&rotated, self.angles.2);
        Vertex {
            x: rotated.x + center.x,
            y: rotated.y + center.y,
            z: rotated.z + center.z,
        }
    }

    fn create_cube(&mut self) {
        let mut vertices = vec![
            Vertex {
                x: -1.0,
                y: -1.0,
                z: 11.0,
            },
            Vertex {
                x: 1.0,
                y: -1.0,
                z: 11.0,
            },
            Vertex {
                x: 1.0,
                y: 1.0,
                z: 11.0,
            },
            Vertex {
                x: -1.0,
                y: 1.0,
                z: 11.0,
            },
            Vertex {
                x: -1.0,
                y: -1.0,
                z: 9.0,
            },
            Vertex {
                x: 1.0,
                y: -1.0,
                z: 9.0,
            },
            Vertex {
                x: 1.0,
                y: 1.0,
                z: 9.0,
            },
            Vertex {
                x: -1.0,
                y: 1.0,
                z: 9.0,
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
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("Window")
                    .with_inner_size(winit::dpi::LogicalSize::new(
                        self.renderer.width as f64,
                        self.renderer.height as f64,
                    )),
            )
            .unwrap();

        let window = Arc::new(window);

        let surface = SurfaceTexture::new(
            self.renderer.width as u32,
            self.renderer.height as u32,
            window.clone(),
        );

        let pixels = Pixels::new(
            self.renderer.width as u32,
            self.renderer.height as u32,
            surface,
        )
        .unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);

        self.create_cube();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                self.renderer.clear(0, 0, 0, 255);

                self.angles.0 += 0.01;
                self.angles.1 += 0.015;
                self.angles.2 += 0.012;

                let rotated_vertices: Vec<Vertex> = self
                    .vertices
                    .iter()
                    .map(|v| self.rotate_vertex(v))
                    .collect();

                for vertex in rotated_vertices.iter() {
                    self.renderer.draw_vertex(vertex);
                }

                for edge in self.edges.iter() {
                    self.renderer.draw_edge(
                        &rotated_vertices[edge.0 as usize],
                        &rotated_vertices[edge.1 as usize],
                        255,
                        255,
                        255,
                        255,
                    );
                }

                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();
                    frame.copy_from_slice(&self.renderer.buffer);
                    pixels.render().unwrap();
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::Resized(size) => {
                self.renderer = Renderer::new(size.width as usize, size.height as usize);
                if let Some(pixels) = &mut self.pixels {
                    pixels.resize_surface(size.width, size.height).unwrap();
                    pixels.resize_buffer(size.width, size.height).unwrap();
                }
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}
