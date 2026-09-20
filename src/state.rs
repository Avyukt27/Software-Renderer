use std::{collections::HashSet, sync::Arc, time::Instant};

use winit::{
    event::{DeviceEvent, WindowEvent},
    keyboard::KeyCode,
    window::Window,
};

use crate::{
    camera::Camera,
    light::Light,
    loaders::obj::load_obj,
    models::{Model, Object},
    renderer::Renderer,
};

pub struct State {
    window: Arc<Window>,
    renderer: Renderer,
    camera: Camera,
    models: Vec<Model>,
    objects: Vec<Object>,
    lights: Vec<Light>,

    pressed_keys: HashSet<KeyCode>,
    start_time: Instant,
}

impl State {
    pub async fn new(window: Arc<Window>, renderer: Renderer) -> Self {
        let size = window.inner_size();

        let cube = load_obj(
            "models/basic_cube/basic_cube.obj",
            renderer.device(),
            renderer.queue(),
            renderer.texture_bind_group_layout(),
        );
        let models = vec![cube];

        let objects = vec![
            Object {
                model_index: 0,
                position: glam::Vec3::new(-1.5, 0.0, 0.0),
                rotation: glam::Vec3::ZERO,
                scale: glam::Vec3::ONE,
            },
            Object {
                model_index: 0,
                position: glam::Vec3::new(1.5, 0.0, 0.0),
                rotation: glam::Vec3::ZERO,
                scale: glam::Vec3::splat(0.5),
            },
        ];

        let lights = vec![Light::new(
            glam::Vec3::new(3.0, 0.0, 0.0),
            glam::Vec3::new(1.0, 1.0, 1.0),
            glam::Vec3::new(0.5, 0.5, 0.5),
            0.1,
        )];

        Self {
            window,
            renderer,
            camera: Camera::new((size.width, size.height)),
            models,
            objects,
            lights,
            pressed_keys: HashSet::new(),
            start_time: Instant::now(),
        }
    }

    fn render(&mut self) -> anyhow::Result<()> {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        if let Some(obj1) = self.objects.get_mut(0) {
            obj1.rotation.y = elapsed * 1.0;
        }
        if let Some(obj2) = self.objects.get_mut(1) {
            obj2.rotation.z = elapsed * 0.5;
        }

        self.renderer
            .render(&self.models, &self.objects, &self.camera, &self.lights)?;
        self.window.request_redraw();

        Ok(())
    }

    pub fn handle_event(&mut self, event: WindowEvent) {
        match event {
            winit::event::WindowEvent::Resized(size) => {
                self.renderer.resize((size.width, size.height));
                self.camera.resize((size.width, size.height));
            }
            winit::event::WindowEvent::RedrawRequested => {
                let _ = self.render();
            }
            winit::event::WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key: winit::keyboard::PhysicalKey::Code(code),
                        state,
                        ..
                    },
                ..
            } => match (code, state.is_pressed()) {
                (key, true) => {
                    self.pressed_keys.insert(key);
                }
                (key, false) => {
                    self.pressed_keys.remove(&key);
                }
            },
            _ => {}
        }
    }

    pub fn handle_device_event(&mut self, event: DeviceEvent) {
        if let winit::event::DeviceEvent::MouseMotion { delta } = event {
            self.camera.rotate_view(delta);
        }
    }

    pub fn handle_key(&mut self) {
        let speed = 0.05_f32;

        for key in &self.pressed_keys {
            match key {
                KeyCode::KeyW => self.camera.move_forward(speed),
                KeyCode::KeyS => self.camera.move_forward(-speed),
                KeyCode::KeyA => self.camera.move_strafe(-speed),
                KeyCode::KeyD => self.camera.move_strafe(speed),
                KeyCode::KeyQ => self.camera.move_up(speed),
                KeyCode::KeyE => self.camera.move_up(-speed),
                _ => {}
            }
        }
    }
}
