use rand::Rng;
use wgpu::{util::DeviceExt, Buffer, Device, Queue};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstData {
    position: [f32; 3],
    scale: f32,
    color: [f32; 3],
    _pad: u32,
}

impl InstData {
    const ATTRIBS: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![1 => Float32x3, 2 => Float32, 3 => Float32x3, 4 => Uint32];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ComputeData {
    state: u32,
    copy: u32,
    _pad: [u32; 2],
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
    },
    Vertex {
        position: [0.0, -1.0, 0.0],
    },
];

pub const INDICES: &[u32] = &[0, 1, 2, 0, 2, 3];
pub const GRID: (u32, u32) = (1000, 1000);
pub const SIZE: f32 = 2.0;
pub const INSTCOUNT: usize = (GRID.0 * GRID.1) as usize;

pub fn create_grid(grid: (u32, u32), size: f32) -> Vec<InstData> {
    let mut tmp: Vec<InstData> = Vec::new();
    tmp.reserve_exact((grid.0 * grid.1) as usize);

    for x in 0..grid.0 {
        for y in 0..grid.1 {
            tmp.push(InstData {
                position: [x as f32 * size, y as f32 * -size, 0.0],
                color: [0.0, 0.0, 0.0],
                scale: size,
                _pad: 0,
            })
        }
    }

    tmp
}

pub fn create_grid_compute(grid: (u32, u32)) -> Vec<ComputeData> {
    let mut tmp = Vec::new();
    tmp.reserve_exact((grid.0 * grid.1) as usize);
    let mut rng = rand::thread_rng();

    for _x in 0..grid.0 {
        for _y in 0..grid.1 {
            tmp.push(ComputeData {
                state: rng.gen_range(0..=200),
                copy: rng.gen_range(0..=200),
                _pad: Default::default(),
            })
        }
    }

    tmp
}

pub fn create_vb(device: &Device) -> Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    })
}

pub fn create_idx(device: &Device) -> Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: wgpu::BufferUsages::INDEX,
    })
}

pub fn create_inst(device: &Device) -> Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(&create_grid(GRID, SIZE)),
        usage: wgpu::BufferUsages::VERTEX
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::STORAGE,
    })
}
