use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

use pixels::{Pixels, SurfaceTexture};

use crate::{
    math::{rotate_vertex, rotate_vertices},
    mesh::Mesh,
    renderer::Renderer,
    vertex::Vertex,
};

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

        let cube = Mesh::cube(-5.0, -5.0, 1.0, 10.0);
        self.meshes.push(cube);

        let sphere = Mesh::sphere(5.0, 5.0, 3.0, 20, 10.0);
        self.meshes.push(sphere);
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

                let mut transformed_vertices: Vec<Vertex> = Vec::new();

                for mesh in &self.meshes {
                    let mut rotated_vertices = rotate_vertices(&mesh.vertices, self.angles);
                    transformed_vertices.append(&mut rotated_vertices);

                    for vertex in transformed_vertices.iter() {
                        self.renderer.draw_vertex(vertex);
                    }

                    for edge in mesh.edges.iter() {
                        self.renderer.draw_edge(
                            &transformed_vertices[edge.0],
                            &transformed_vertices[edge.1],
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
