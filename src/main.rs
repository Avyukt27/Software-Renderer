use winit::event_loop::EventLoop;

mod app;
mod camera;
mod loaders;
mod mesh;
mod renderer;
mod texture;
mod vertex;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = app::App::new();
    event_loop.run_app(&mut app).unwrap();
}
