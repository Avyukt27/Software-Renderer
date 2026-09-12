use std::sync::Arc;

use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event::WindowEvent, window::Window,
};

use crate::{renderer::Renderer, state::State};

pub struct App {
    state: Option<State>,
}

impl App {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let options = Window::default_attributes()
            .with_title("Renderer")
            .with_inner_size(PhysicalSize::new(800.0, 600.0))
            .with_resizable(false);
        let window = event_loop.create_window(options).unwrap();
        let _ = window.set_cursor_grab(winit::window::CursorGrabMode::Locked);
        window.set_cursor_visible(false);
        let window = Arc::new(window);
        let renderer = pollster::block_on(Renderer::new(Arc::clone(&window)));
        self.state = Some(pollster::block_on(State::new(window, renderer)));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            self.state = None;
            event_loop.exit();
            return;
        } else if let WindowEvent::KeyboardInput {
            event:
                winit::event::KeyEvent {
                    physical_key:
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape),
                    state: winit::event::ElementState::Pressed,
                    ..
                },
            ..
        } = event
        {
            self.state = None;
            event_loop.exit();
            return;
        }

        let state = match &mut self.state {
            Some(state) => state,
            None => return,
        };
        state.handle_event(event);
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let Some(state) = &mut self.state {
            state.handle_device_event(event);
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(state) = &mut self.state {
            state.handle_key();
        }
    }
}
