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

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstData {
    position: [f32; 4],
    color: [f32; 4],
    scale: [f32; 4],
}

impl InstData {
    const ATTRIBS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![1 => Float32x4, 2 => Float32x4, 3 => Float32x4];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ComputeData {
    color: [f32; 4]
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

pub const INDICES: &[u32] = &[0,1,2,0,2,3];
pub const GRID: (u32,u32) = (100,100);
pub const SIZE: f32 = 10.0;
pub const INSTCOUNT: usize = (GRID.0*GRID.1) as usize;



pub fn create_grid(grid: (u32,u32), size: f32) -> Vec<InstData> {
    
    let mut tmp: Vec<InstData> = Vec::new();
    tmp.reserve_exact((grid.0*grid.1) as usize);

    for x in 0..grid.0 {
        for y in 0..grid.1 {
            tmp.push(
                InstData {
                    position: [x as f32 * size, y as f32 * -size, 0.0, /*padding*/0.0],
                    color:  [0.0,0.0,1.0,/*padding*/0.0],
                    scale: [size,/*padding*/0.0,0.0,0.0],
                }
            )
        }
    }

    tmp
}

pub fn create_grid_compute(grid: (u32,u32)) -> Vec<ComputeData> {
    
    let mut tmp = Vec::new();
    tmp.reserve_exact((grid.0*grid.1) as usize);

    for _x in 0..grid.0 {
        for _y in 0..grid.1 {
            tmp.push(
                ComputeData {
                    color: [1.0,1.0,1.0,0.0]
                }
            )
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
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
    })
}
