use std::sync::Arc;
use bytemuck::bytes_of;
use wgpu::ShaderStages;
use winit::application::ApplicationHandler;

use winit::event::{WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use crate::renderdata::GRID;

use super::state::*;

#[derive(Default)]
pub struct App<'a> {
    pub window: Option<Arc<Window>>,
    pub state: Option<State<'a>>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        ));

        self.state = Some(pollster::block_on(State::new(
            (&self).window.as_ref().unwrap().clone(),
        )));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                log::info!("The close button was pressed; stopping");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                let state = self.state.as_ref().unwrap();

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
                {
                    let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{
                        label: Some("Compute Pass 0"),
                        timestamp_writes: None,
                    });

                    cpass.set_pipeline(&state.compute.cs_pipeline);
                    cpass.set_bind_group(0, &state.compute.compute_bind_group, &[]);
                    cpass.set_push_constants(0, bytemuck::cast_slice(&[GRID.0,GRID.1]));
                    cpass.insert_debug_marker("use compute shader");
                    cpass.dispatch_workgroups(GRID.0*GRID.1, 1, 1);
                }

                {
                    let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{
                        label: Some("Compute Pass 1"),
                        timestamp_writes: None,
                    });

                    cpass.set_pipeline(&state.compute.copy_pipeline);
                    cpass.set_bind_group(0, &state.compute.copy_bind_group, &[]);
                    cpass.set_push_constants(0, bytemuck::cast_slice(&[GRID.0,GRID.1]));
                    cpass.insert_debug_marker("copy to instance buffer");
                    cpass.dispatch_workgroups(GRID.0*GRID.1, 1, 1);
                }

                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass 0"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    }); 
                    rpass.set_pipeline(&state.render_pipeline);

                    // set vertex and instance buffers
                    rpass.set_vertex_buffer(0, state.vertex_buffer.slice(..));
                    rpass.set_vertex_buffer(1, state.instance_buffer.slice(..));

                    // Index buffer
                    rpass.set_index_buffer(state.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

                    // pass window scale though push const.
                    let size = <[f32; 2]>::from(self.window.as_ref().unwrap().inner_size());
                    rpass.set_push_constants(ShaderStages::VERTEX, 0, bytemuck::cast_slice(&size));

                    rpass.draw_indexed(0..state.index_len as u32, 0, 0..state.instance_len as u32);
                }

                state.queue.submit(Some(encoder.finish()));
                frame.present();
                self.window.as_ref().unwrap().request_redraw();

                // call update with state mut
                self.state.as_mut().unwrap().update();
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
}
