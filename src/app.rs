use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes},
};

use pixels::{Pixels, SurfaceTexture};

use crate::{
    camera::Camera, math::rotate_vertices, mesh::Mesh, renderer::Renderer, vertex::Vertex,
};

#[derive(Debug)]
pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    renderer: Renderer,
    camera: Camera,
    meshes: Vec<Mesh>,
    angles: (f32, f32, f32),
}

impl App {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            window: None,
            pixels: None,
            renderer: Renderer::new(width, height),
            camera: Camera::new(width, height),
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
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    logical_key, state, ..
                },
                ..
            } => {
                if state.is_pressed() {
                    match logical_key {
                        Key::Named(NamedKey::ArrowUp) => {
                            println!("Up Arrow Pressed!")
                        }
                        Key::Named(NamedKey::ArrowDown) => {
                            println!("Down Arrow Pressed!")
                        }
                        Key::Named(NamedKey::ArrowLeft) => {
                            println!("Left Arrow Pressed!")
                        }
                        Key::Named(NamedKey::ArrowRight) => {
                            println!("Right Arrow Pressed!")
                        }
                        Key::Character(ref c) if c == "w" => self.camera.position.y += 0.2,
                        Key::Character(ref c) if c == "s" => self.camera.position.y -= 0.2,
                        Key::Character(ref c) if c == "d" => self.camera.position.x += 0.2,
                        Key::Character(ref c) if c == "a" => self.camera.position.x -= 0.2,
                        Key::Character(ref c) if c == "q" => self.camera.position.z += 0.2,
                        Key::Character(ref c) if c == "e" => self.camera.position.z -= 0.2,
                        _ => {}
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                self.renderer.clear(0, 0, 0, 255);

                self.angles.0 += 0.01;
                self.angles.1 += 0.015;
                self.angles.2 += 0.012;

                for mesh in &self.meshes {
                    let rotated_vertices = rotate_vertices(&mesh.vertices, self.angles);

                    let world_vertices: Vec<Vertex> = rotated_vertices
                        .iter()
                        .map(|v| Vertex {
                            x: v.x + mesh.centre.x,
                            y: v.y + mesh.centre.y,
                            z: v.z + mesh.centre.z,
                        })
                        .collect();

                    let view_vertices: Vec<Option<Vertex>> = world_vertices
                        .iter()
                        .map(|v| self.camera.project_perspective(v))
                        .collect();

                    for vertex in view_vertices.iter().flatten() {
                        self.renderer.draw_vertex(vertex);
                    }

                    for &(from, to) in &mesh.edges {
                        if let (Some(v1), Some(v2)) = (&view_vertices[from], &view_vertices[to]) {
                            self.renderer.draw_edge(v1, v2, 255, 255, 255, 255);
                        }
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
