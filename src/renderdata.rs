use wgpu::{util::DeviceExt, Buffer, Device};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}


impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 1] =
        wgpu::vertex_attr_array![0 => Float32x3];

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
    position: [f32; 3],
    color: [f32; 3],
}

impl InstData {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
    wgpu::vertex_attr_array![1 => Float32x3, 2 => Float32x3];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
    },
];

pub const INDICES: &[u32] = &[
    0, 1, 2,
    0, 1, 3 
];

pub const INSTANCES: &[InstData] = &[
    InstData {
        position: [0.0,0.0,0.0],
        color: [1.0,1.0,1.0]
    } ,
    InstData {
        position: [-1.0,0.0,0.0],
        color: [1.0,0.0,0.0]
    },
    InstData {
        position: [-1.0,-1.0,0.0],
        color: [0.0,0.0,1.0]
    },
    InstData {
        position: [0.0,-1.0,0.0],
        color: [0.0,1.0,0.0]
    }
];

pub fn create_vb(device: &Device) -> Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    })
}


pub fn create_idx(device: &Device) -> Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: wgpu::BufferUsages::INDEX,
    })
}

pub fn create_inst(device: &Device) -> Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(INSTANCES),
        usage: wgpu::BufferUsages::VERTEX,
    })
}