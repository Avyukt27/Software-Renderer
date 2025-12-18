use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};

use pixels::{Pixels, SurfaceTexture};

mod renderer;
mod vertex;

use renderer::Renderer;
use vertex::Vertex;

#[derive(Debug)]
struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    renderer: renderer::Renderer,
    vertices: Vec<Vertex>,
}

impl App {
    fn new(width: usize, height: usize) -> Self {
        Self {
            window: None,
            pixels: None,
            renderer: Renderer::new(width, height),
            vertices: vec![],
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

        let mut vertices = vec![Vertex {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        }];

        self.vertices.append(&mut vertices);
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
                for vertex in self.vertices.iter() {
                    self.renderer.draw_vertex(&vertex);
                }

                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();
                    frame.copy_from_slice(&self.renderer.buffer);
                    pixels.render().unwrap();
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

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut app = App::new(800, 600);
    event_loop.run_app(&mut app).unwrap();
}
