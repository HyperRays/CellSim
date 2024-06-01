use std::sync::Arc;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::Window;

use super::state::State;

#[derive(Default)]
pub struct App<'a> {
    pub window: Option<Arc<Window>>,
    pub state: Option<State<'a>>,
}

impl<'a> App<'a> {
    async fn resumed(&mut self) {
        self.state = Some(State::new(self.window.as_ref().unwrap().clone()).await);
    }

    fn window_event(&mut self, event: WindowEvent, event_loop: &EventLoopWindowTarget<()>) {
        let _ = self
            .state
            .as_mut()
            .unwrap()
            .egui_renderer
            .handle_input(self.window.as_ref().unwrap(), &event);

        match event {
            WindowEvent::CloseRequested => {
                log::info!("The close button was pressed; stopping");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                let state = self.state.as_mut().unwrap();

                let frame = state
                    .surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = state
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                // state.compute.compute(&mut encoder);
                state.render(&mut encoder, &view, self.window.as_ref().unwrap());

                let win = self.window.as_ref().unwrap();
                let gui_screen_descriptor = egui_wgpu::ScreenDescriptor {
                    size_in_pixels: win.inner_size().into(),
                    pixels_per_point: win.scale_factor() as f32,
                };

                state.egui_renderer.draw(
                    &state.device,
                    &state.queue,
                    &mut encoder,
                    win,
                    &view,
                    gui_screen_descriptor,
                    |ctx| {
                        egui::Window::new("Settings").show(ctx, |ui| {
                            ui.label(format!(
                                "Grid Size: {:?}",
                                (crate::settings::GRID.0, crate::settings::GRID.1)
                            ));
                        });

                        egui::Window::new("Presets").show(ctx, |ui| {});
                    },
                );

                state.queue.submit(Some(encoder.finish()));
                frame.present();
                self.window.as_ref().unwrap().request_redraw();

                // call update with state mut
                self.state.as_mut().unwrap().update();
                // let time = time::Duration::from_millis(60);
                // thread::sleep(time);
            }

            WindowEvent::Resized(physical_size) => {
                log::debug!("Window dimensions changed to: {:?}", physical_size);
                self.state.as_mut().unwrap().resize(physical_size);
            }

            WindowEvent::ScaleFactorChanged { .. } => {
                log::debug!("Resize event occured");
            }

            _ => (),
        }
    }

    pub async fn run(mut self, event_loop: EventLoop<()>, window: Window) {
        self.window = Some(Arc::new(window));
        self.resumed().await;

        let _ = event_loop.run(move |event, target| {
            if let Event::WindowEvent {
                window_id: _,
                event,
            } = event
            {
                self.window_event(event, target);
            }
        });
    }
}
