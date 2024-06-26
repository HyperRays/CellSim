use winit::{dpi::{LogicalSize, PhysicalSize}, event_loop::{ControlFlow, EventLoop}};

mod app;
mod compute;
mod egui;
mod renderdata;
mod state;
mod settings;

pub fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);
    let builder = winit::window::WindowBuilder::new().with_inner_size(PhysicalSize::new(1000, 1000)).with_resizable(false);
    let window = builder.build(&event_loop).unwrap();

    let app = app::App::default();
    pollster::block_on(app.run(event_loop, window));
}
