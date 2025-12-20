use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

use pixels::{Pixels, SurfaceTexture};

use crate::{math::rotate_vertices, mesh::Mesh, renderer::Renderer, vertex::Vertex};

#[derive(Debug)]
pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    renderer: Renderer,
    meshes: Vec<Mesh>,
    angles: (f32, f32, f32),
}

impl App {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            window: None,
            pixels: None,
            renderer: Renderer::new(width, height),
            meshes: Vec::new(),
            angles: (0.0, 0.0, 0.0),
        }
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

        let mut meshes = vec![
            Mesh::cube(0.0, 5.0, 10.0, 1.0),
            Mesh::sphere(0.0, -1.0, 10.0, 5.0, 12),
        ];
        self.meshes.append(&mut meshes);
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

                for mesh in &self.meshes {
                    let rotated_vertices = rotate_vertices(&mesh.vertices, self.angles);

                    let transformed_vertices: Vec<Vertex> = rotated_vertices
                        .iter()
                        .map(|v| Vertex {
                            x: v.x + mesh.centre.x,
                            y: v.y + mesh.centre.y,
                            z: v.z + mesh.centre.z,
                        })
                        .collect();

                    for vertex in transformed_vertices.iter() {
                        self.renderer.draw_vertex(vertex);
                    }

                    for &(from, to) in &mesh.edges {
                        self.renderer.draw_edge(
                            &transformed_vertices[from],
                            &transformed_vertices[to],
                            255,
                            255,
                            255,
                            255,
                        );
                    }
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
