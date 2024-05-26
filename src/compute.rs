use std::borrow::Cow;

use wgpu::Queue;
use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayoutDescriptor, BindGroupLayoutEntry, Buffer,
    CommandEncoder, ComputePipeline, Device, PushConstantRange,
};

use std::time::{Duration, Instant};

use crate::renderdata::create_grid_compute;
use crate::settings::*;

pub struct Compute {
    pub cs_pipeline: ComputePipeline,
    pub copy_pipeline: ComputePipeline,
    pub compute_buffer: Buffer,
    pub compute_bind_group: BindGroup,
    pub copy_bind_group: BindGroup,
    pub time: Instant,
    pub duration: u64,
    pub var: [u32; 3],
    pub sim_step: u32,
}

impl Compute {
    pub fn new(device: &Device, inst_buffer: &Buffer) -> Self {
        let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("computation shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("compute.wgsl"))),
        });

        let copy_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Copy compute shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("copy.wgsl"))),
        });

        let compute_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        count: None,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                    }],
                }),
            ],
            push_constant_ranges: &[PushConstantRange {
                range: 0..4 * 5,
                stages: wgpu::ShaderStages::COMPUTE,
            }],
        });

        let copy_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            count: None,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            count: None,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                        },
                    ],
                }),
            ],
            push_constant_ranges: &[PushConstantRange {
                range: 0..4 * 5,
                stages: wgpu::ShaderStages::COMPUTE,
            }],
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute pipeline"),
            layout: Some(&compute_layout),
            module: &cs_module,
            entry_point: "main",
            compilation_options: Default::default(),
        });

        let copy_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Copy pipeline"),
            layout: Some(&copy_layout),
            module: &copy_module,
            entry_point: "main",
            compilation_options: Default::default(),
        });

        log::debug!("Length: {:?}", create_grid_compute(GRID).len());

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Compute Buffer"),
            contents: bytemuck::cast_slice(&create_grid_compute(GRID)),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        log::debug!("Length of compute Buffer {:?}", buffer.size());

        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("compute bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        let bind_group_layout = copy_pipeline.get_bind_group_layout(0);
        let copy_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("copy bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: inst_buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            cs_pipeline: compute_pipeline,
            copy_pipeline,
            compute_buffer: buffer,
            compute_bind_group,
            copy_bind_group,
            duration: 60,
            time: Instant::now(),
            var: [3, 3, 28],
            sim_step: 0,
        }
    }

    pub fn compute(&mut self, encoder: &mut CommandEncoder) {
        let now = Instant::now();
        if Instant::now() - self.time >= Duration::from_millis(self.duration) {
            {
                let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("Compute Pass 0"),
                    timestamp_writes: None,
                });

                cpass.set_pipeline(&self.cs_pipeline);
                cpass.set_bind_group(0, &self.compute_bind_group, &[]);
                cpass.set_push_constants(
                    0,
                    bytemuck::cast_slice(&[GRID.0, GRID.1, self.var[0], self.var[1], self.var[2]]),
                );
                cpass.insert_debug_marker("use compute shader");
                cpass.dispatch_workgroups(GRID.0, GRID.1, 1);
            }

            {
                let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("Compute Pass 1"),
                    timestamp_writes: None,
                });

                cpass.set_pipeline(&self.copy_pipeline);
                cpass.set_bind_group(0, &self.copy_bind_group, &[]);
                cpass.set_push_constants(
                    0,
                    bytemuck::cast_slice(&[GRID.0, GRID.1, self.var[0], self.var[1], self.var[2]]),
                );
                cpass.insert_debug_marker("copy to instance buffer");
                cpass.dispatch_workgroups(GRID.0, GRID.1, 1);
            }
            self.time = now;
            self.sim_step += 1;
        }
    }

    pub fn reset(&mut self, queue: &Queue) {
        queue.write_buffer(
            &self.compute_buffer,
            0,
            bytemuck::cast_slice(&create_grid_compute(GRID)),
        );
        self.sim_step = 0;
    }
}
