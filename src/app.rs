use std::sync::Arc;

use winit::{application::ApplicationHandler, event::KeyEvent, keyboard::Key, window::Window};

use crate::{camera::Camera, loaders::obj::load_obj, mesh::Mesh, renderer::Renderer};

pub struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    camera: Option<Camera>,
    meshes: Vec<Mesh>,

    is_w_pressed: bool,
    is_s_pressed: bool,
    is_a_pressed: bool,
    is_d_pressed: bool,
    is_space_pressed: bool,
    is_shift_pressed: bool,
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
            is_space_pressed: false,
            is_shift_pressed: false,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("Software Renderer [Window]"))
            .unwrap();
        let _ = window.set_cursor_grab(winit::window::CursorGrabMode::Locked);
        window.set_cursor_visible(false);
        let size = window.inner_size();
        let window = Arc::new(window);

        let renderer = pollster::block_on(Renderer::new(Arc::clone(&window)));
        self.window = Some(window);
        self.renderer = Some(renderer);
        self.camera = Some(Camera::new((size.width, size.height)));

        let cube = load_obj("models/basic_cube/basic_cube.obj");

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
                    let forward = camera.get_forawrd();
                    let right = forward.cross(glam::Vec3::Y).normalize();
                    let up = right.cross(forward).normalize();

                    if self.is_w_pressed {
                        camera.position += forward * speed;
                    }
                    if self.is_s_pressed {
                        camera.position -= forward * speed;
                    }
                    if self.is_a_pressed {
                        camera.position -= right * speed;
                    }
                    if self.is_d_pressed {
                        camera.position += right * speed;
                    }
                    if self.is_space_pressed {
                        camera.position += up * speed;
                    }
                    if self.is_shift_pressed {
                        camera.position -= up * speed;
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
                Key::Named(winit::keyboard::NamedKey::Space) => {
                    self.is_space_pressed = state.is_pressed()
                }
                Key::Named(winit::keyboard::NamedKey::Shift) => {
                    self.is_shift_pressed = state.is_pressed()
                }
                _ => {}
            },
            _ => (),
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let winit::event::DeviceEvent::MouseMotion { delta } = event {
            if let Some(camera) = self.camera.as_mut() {
                let sensitivity = 0.002_f32;
                camera.yaw += (delta.0 as f32) * sensitivity;
                camera.pitch -= (delta.1 as f32) * sensitivity;
                camera.pitch = camera
                    .pitch
                    .clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());
            }
        }
    }
}
