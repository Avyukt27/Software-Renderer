use winit::event_loop::EventLoop;

mod app;
mod camera;
mod loader;
mod math;
mod mesh;
mod primitives;
mod renderer;

use app::App;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut app = App::new(600, 600);
    event_loop.run_app(&mut app).unwrap();
}
