@group(0)
@binding(0)
var<storage, read> v_indices: array<vec4<f32>>; 


struct InstanceInput {
    inst_pos: vec4<f32>,
    color: vec4<f32>,
    scale: vec4<f32>,
}

@group(0)
@binding(1)
var<storage, read_write> instances: array<InstanceInput>; 


@compute
@workgroup_size(1,1,1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>){
    let idx = global_id.x;

    instances[idx].color = v_indices[idx];
}