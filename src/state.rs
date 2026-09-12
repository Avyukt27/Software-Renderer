use std::sync::Arc;

use winit::{
    event::{DeviceEvent, WindowEvent},
    window::Window,
};

use crate::{camera::Camera, loaders::obj::load_obj, models::Model, renderer::Renderer};

pub struct State {
    window: Arc<Window>,
    renderer: Renderer,
    camera: Camera,
    models: Vec<Model>,

    is_w_pressed: bool,
    is_s_pressed: bool,
    is_a_pressed: bool,
    is_d_pressed: bool,
    is_space_pressed: bool,
    is_shift_pressed: bool,
}

impl State {
    pub async fn new(window: Arc<Window>, renderer: Renderer) -> Self {
        let size = window.inner_size();

        let cube = load_obj(
            "models/two_textured_cube/two_textured_cube.obj",
            renderer.device(),
            renderer.queue(),
            renderer.texture_bind_group_layout(),
        );
        let models = vec![cube];

        Self {
            window,
            renderer,
            camera: Camera::new((size.width, size.height)),
            models,
            is_w_pressed: false,
            is_s_pressed: false,
            is_a_pressed: false,
            is_d_pressed: false,
            is_space_pressed: false,
            is_shift_pressed: false,
        }
    }

    pub fn handle_event(&mut self, event: WindowEvent) {
        match event {
            winit::event::WindowEvent::Resized(size) => {
                self.renderer.resize((size.width, size.height));
                self.camera.resize((size.width, size.height));
            }
            winit::event::WindowEvent::RedrawRequested => {
                // TODO: Move rendering into its own method
                let speed = 0.05_f32;
                let forward = self.camera.get_forawrd();
                let right = forward.cross(glam::Vec3::Y).normalize();
                let up = right.cross(forward).normalize();

                // TODO: Move camera movement code into camera struct
                // TODO: Use HashSet instead of bools
                if self.is_w_pressed {
                    self.camera.position += forward * speed;
                }
                if self.is_s_pressed {
                    self.camera.position -= forward * speed;
                }
                if self.is_a_pressed {
                    self.camera.position -= right * speed;
                }
                if self.is_d_pressed {
                    self.camera.position += right * speed;
                }
                if self.is_space_pressed {
                    self.camera.position += up * speed;
                }
                if self.is_shift_pressed {
                    self.camera.position -= up * speed;
                }

                self.renderer.render(&self.models, &self.camera);
                self.window.request_redraw();
            }
            winit::event::WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key: winit::keyboard::PhysicalKey::Code(code),
                        state,
                        ..
                    },
                ..
            } => {
                // TODO: Handle keyboard input
            }
            _ => {}
        }
    }

    pub fn handle_device_event(&mut self, event: DeviceEvent) {
        // TODO: Move this into camera struct
        if let winit::event::DeviceEvent::MouseMotion { delta } = event {
            let sensitivity = 0.002_f32;
            self.camera.yaw += (delta.0 as f32) * sensitivity;
            self.camera.pitch -= (delta.1 as f32) * sensitivity;
            self.camera.pitch = self
                .camera
                .pitch
                .clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());
        }
    }

    pub fn handle_key(&mut self) {
        // TODO: Implement this
    }
}
