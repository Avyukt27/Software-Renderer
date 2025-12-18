use winit::event_loop::EventLoop;

mod app;
mod renderer;
mod vertex;

use app::App;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut app = App::new(800, 600);
    event_loop.run_app(&mut app).unwrap();
}
