use super::renderdata::*;
use std::borrow::Cow;
use std::sync::Arc;
use wgpu::{
    core::instance, naga::back::glsl::PushConstantItem, Adapter, Device, Instance,
    PushConstantRange, Queue, RenderPipeline, ShaderStages, Surface, SurfaceConfiguration,
};
use winit::window::Window;

pub struct State<'a> {
    pub surface: Surface<'a>,
    pub device: Device,
    pub render_pipeline: RenderPipeline,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_len: usize,
    pub instance_buffer: wgpu::Buffer,
    pub instance_len: usize,
}

impl<'a> State<'a> {
    async fn adapter(surface: &Surface<'a>, instance: &Instance) -> Adapter {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        log::info!("Selected adapter: {:?}", adapter.get_info());
        log::debug!("Push constant limit: {:?}", adapter.limits());

        adapter
    }

    async fn device_queue(adapter: &Adapter) -> (Device, Queue) {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::PUSH_CONSTANTS,
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    required_limits: wgpu::Limits {
                        max_push_constant_size: 8,
                        ..wgpu::Limits::downlevel_webgl2_defaults()
                            .using_resolution(adapter.limits())
                    },
                },
                None,
            )
            .await
            .expect("Failed to create device")
    }
    async fn config_pipeline(
        device: &Device,
        surface: &Surface<'a>,
        adapter: &Adapter,
    ) -> RenderPipeline {
        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[PushConstantRange {
                range: 0..8,
                stages: ShaderStages::VERTEX,
            }],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc(), InstData::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..wgpu::PrimitiveState::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
    }

    pub async fn new(window: Arc<Window>) -> Self {
        let mut size = window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);

        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window).unwrap();
        let adapter = Self::adapter(&surface, &instance).await;

        // Create the logical device and command queue
        let (device, queue) = Self::device_queue(&adapter).await;

        let render_pipeline = Self::config_pipeline(&device, &surface, &adapter).await;

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &config);

        let vertex_buffer = create_vb(&device);
        let index_buffer = create_idx(&device);
        let index_len = INDICES.len();
        let instance_buffer = create_inst(&device);
        let instance_len = INSTCOUNT;

        Self {
            surface,
            device,
            render_pipeline,
            queue,
            config,
            vertex_buffer,
            index_buffer,
            index_len,
            instance_buffer,
            instance_len,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn update(&mut self) {
        match update_inst_buffer(&self.device, &self.queue, &self.instance_buffer) {
            Some(buffer) => {self.instance_buffer = buffer},
            None => {}
        }
    }
}
