var<push_constant> grid: vec2<u32>;

struct Comp {
    color: vec3<f32>,
    state: u32,
}   

@group(0)
@binding(0)
var<storage, read_write> v_indices: array<Comp>; 


struct InstanceInput {
    inst_pos: vec3<f32>,
    scale: f32,
    color: vec3<f32>,
    _pad: u32,
}

@group(0)
@binding(1)
var<storage, read_write> instances: array<InstanceInput>; 


@compute
@workgroup_size(1,1,1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>){
    let idx = global_id.x + global_id.y;
    instances[idx].color.x = v_indices[idx].color.x;
    instances[idx].color.y = v_indices[idx].color.y;
    instances[idx].color.z = v_indices[idx].color.z;
}