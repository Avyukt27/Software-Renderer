use std::sync::Arc;

use winit::{application::ApplicationHandler, window::Window};

use crate::renderer::Renderer;

pub struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            renderer: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes().with_title("Software Renderer [Window]"),
                )
                .unwrap(),
        );
        let renderer = pollster::block_on(Renderer::new(Arc::clone(&window)));
        self.window = Some(window);
        self.renderer = Some(renderer);
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
            }
            winit::event::WindowEvent::RedrawRequested => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.render();
                }
                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}
