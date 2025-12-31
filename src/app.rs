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
    camera::Camera,
    loader::load_wavefront,
    math::{rotate_around_pivot, rotate_vertex},
    mesh::Mesh,
    primitives::{colour::Colour, texture::Texture, vertex::Vertex},
    renderer::Renderer,
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

        let mut cube1 = load_wavefront("assets/objects/cube.obj").expect("Failed to load OBJ");
        cube1.texture = Some(Texture::debug(1000));
        cube1.centre.z = 10.0;
        let mut cube2 = Mesh::cube(10.0, 10.0, 10.0, 5.0);
        cube2.texture = Some(Texture::debug(1000));

        self.meshes.push(cube1);
        self.meshes.push(cube2);
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
                const ROT_SPEED: f32 = 0.02;

                if state.is_pressed() {
                    match logical_key {
                        Key::Named(NamedKey::ArrowUp) => {
                            self.camera.rotation.0 += ROT_SPEED;
                            self.camera.rotation.0 = self.camera.rotation.0.clamp(-1.5, 1.5)
                        }
                        Key::Named(NamedKey::ArrowDown) => {
                            self.camera.rotation.0 -= ROT_SPEED;
                            self.camera.rotation.0 = self.camera.rotation.0.clamp(-1.5, 1.5)
                        }
                        Key::Named(NamedKey::ArrowLeft) => self.camera.rotation.1 += ROT_SPEED,
                        Key::Named(NamedKey::ArrowRight) => self.camera.rotation.1 -= ROT_SPEED,

                        Key::Character(ref c) if c == "w" => self.camera.position.z += 0.2,
                        Key::Character(ref c) if c == "s" => self.camera.position.z -= 0.2,

                        Key::Character(ref c) if c == "d" => self.camera.position.x += 0.2,
                        Key::Character(ref c) if c == "a" => self.camera.position.x -= 0.2,

                        Key::Character(ref c) if c == "q" => self.camera.position.y += 0.2,
                        Key::Character(ref c) if c == "e" => self.camera.position.y -= 0.2,

                        Key::Character(ref c) if c == "x" => self.angles.0 += 0.2,
                        Key::Character(ref c) if c == "c" => self.angles.1 += 0.2,
                        Key::Character(ref c) if c == "z" => self.angles.2 += 0.2,
                        _ => {}
                    }
                }
            }

            WindowEvent::RedrawRequested => {
                let bg_colour = Colour::new(0, 0, 0, 255);

                self.renderer.clear(bg_colour);

                for mesh in &self.meshes {
                    let pivot = if mesh.rotate_around_pivot {
                        Some(mesh.pivot.as_ref().unwrap())
                    } else {
                        None
                    };

                    let world_vertices: Vec<Vertex> = mesh
                        .vertices
                        .iter()
                        .map(|v| {
                            let local_rotated = rotate_vertex(v, self.angles);

                            let mut world = Vertex::new(
                                local_rotated.x + mesh.centre.x,
                                local_rotated.y + mesh.centre.y,
                                local_rotated.z + mesh.centre.z,
                                local_rotated.u,
                                local_rotated.v,
                            );

                            if let Some(p) = pivot {
                                world = rotate_around_pivot(&world, p, self.angles);
                            }

                            world
                        })
                        .collect();

                    let view_vertices: Vec<Option<Vertex>> = world_vertices
                        .iter()
                        .map(|v| self.camera.project_perspective(v))
                        .collect();

                    for triangle in &mesh.triangles {
                        if let (Some(v0), Some(v1), Some(v2)) = (
                            &view_vertices[triangle.i0],
                            &view_vertices[triangle.i1],
                            &view_vertices[triangle.i2],
                        ) {
                            self.renderer.fill_triangle(v0, v1, v2, &mesh.texture);
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
