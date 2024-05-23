use std::time::Instant;

use winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod renderdata;
mod state;
mod compute;

pub fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = app::App::default();
    let _ = event_loop.run_app(&mut app);
}
