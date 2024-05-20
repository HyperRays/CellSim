use std::borrow::Cow;

use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, Buffer, ComputePipeline, Device, PushConstantRange};

use crate::renderdata::{create_grid_compute, GRID};

pub struct Compute {
    pub cs_pipeline: ComputePipeline,
    pub copy_pipeline: ComputePipeline,
    pub compute_buffer: Buffer,
    pub compute_bind_group: BindGroup,
    pub copy_bind_group: BindGroup,
}

impl Compute {
    pub fn new(device: &Device, inst_buffer: &Buffer) -> Self{

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
                &device.create_bind_group_layout(&BindGroupLayoutDescriptor{
                    label: None,
                    entries: &[BindGroupLayoutEntry{
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        count: None,
                        ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }
                    }]
                })
            ],
            push_constant_ranges: &[PushConstantRange {
                range: 0..8,
                stages: wgpu::ShaderStages::COMPUTE,
            }],
        });

        let copy_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &device.create_bind_group_layout(&BindGroupLayoutDescriptor{
                    label: None,
                    entries: &[BindGroupLayoutEntry{
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        count: None,
                        ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }
                    },
                    BindGroupLayoutEntry{
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        count: None,
                        ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }
                    }
                    ]
                })
            ],
            push_constant_ranges: &[PushConstantRange {
                range: 0..8,
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

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Compute Buffer"),
            contents: bytemuck::cast_slice(&create_grid_compute(GRID)),
            usage: wgpu::BufferUsages::STORAGE,
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
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: inst_buffer.as_entire_binding(),
            }],
        });

        Self {
            cs_pipeline: compute_pipeline,
            copy_pipeline: copy_pipeline,
            compute_buffer: buffer,
            compute_bind_group,
            copy_bind_group,
        }

    }
}