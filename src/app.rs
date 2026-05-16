use std::sync::Arc;

use winit::{application::ApplicationHandler, event::KeyEvent, keyboard::Key, window::Window};

use crate::{camera::Camera, mesh::Mesh, renderer::Renderer, shapes::cube};

pub struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    camera: Option<Camera>,
    meshes: Vec<Mesh>,

    is_w_pressed: bool,
    is_s_pressed: bool,
    is_a_pressed: bool,
    is_d_pressed: bool,
    is_q_pressed: bool,
    is_e_pressed: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            renderer: None,
            camera: None,
            meshes: Vec::new(),
            is_w_pressed: false,
            is_s_pressed: false,
            is_a_pressed: false,
            is_d_pressed: false,
            is_q_pressed: false,
            is_e_pressed: false,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("Software Renderer [Window]"))
            .unwrap();
        let size = window.inner_size();
        let window = Arc::new(window);

        let renderer = pollster::block_on(Renderer::new(Arc::clone(&window)));
        self.window = Some(window);
        self.renderer = Some(renderer);
        self.camera = Some(Camera::new((size.width, size.height)));

        let cube = cube();

        if let Some(renderer) = self.renderer.as_mut() {
            self.meshes = vec![renderer.upload_mesh(&cube.0, &cube.1)];
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::Resized(size) => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.resize((size.width, size.height));
                }
                if let Some(camera) = self.camera.as_mut() {
                    camera.resize((size.width, size.height));
                }
            }
            winit::event::WindowEvent::RedrawRequested => {
                if let Some(renderer) = self.renderer.as_mut()
                    && let Some(camera) = self.camera.as_mut()
                {
                    let speed = 0.05_f32;
                    let forward = (camera.target - camera.position).normalize();
                    let right = forward.cross(glam::Vec3::Y).normalize();
                    let up = right.cross(forward).normalize();

                    if self.is_w_pressed {
                        camera.position += forward * speed;
                        camera.target += forward * speed;
                    }
                    if self.is_s_pressed {
                        camera.position -= forward * speed;
                        camera.target -= forward * speed;
                    }
                    if self.is_a_pressed {
                        camera.position -= right * speed;
                        camera.target -= right * speed;
                    }
                    if self.is_d_pressed {
                        camera.position += right * speed;
                        camera.target += right * speed;
                    }
                    if self.is_q_pressed {
                        camera.position += up * speed;
                        camera.target += up * speed;
                    }
                    if self.is_e_pressed {
                        camera.position -= up * speed;
                        camera.target -= up * speed;
                    }

                    renderer.render(&self.meshes, camera);
                }
                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            }
            winit::event::WindowEvent::KeyboardInput {
                event: KeyEvent {
                    logical_key, state, ..
                },
                ..
            } => match logical_key {
                Key::Character(ref c) if c == "w" => self.is_w_pressed = state.is_pressed(),
                Key::Character(ref c) if c == "s" => self.is_s_pressed = state.is_pressed(),
                Key::Character(ref c) if c == "a" => self.is_a_pressed = state.is_pressed(),
                Key::Character(ref c) if c == "d" => self.is_d_pressed = state.is_pressed(),
                Key::Character(ref c) if c == "q" => self.is_q_pressed = state.is_pressed(),
                Key::Character(ref c) if c == "e" => self.is_e_pressed = state.is_pressed(),
                _ => {}
            },
            _ => (),
        }
    }
}
