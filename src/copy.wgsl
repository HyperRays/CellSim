@group(0)
@binding(0)
var<storage, read_write> v_indices: array<vec3<f32>>; 


struct InstanceInput {
    inst_pos: vec4<f32>,
    color: vec4<f32>,
    scale: vec4<f32>,
}

@group(0)
@binding(1)
var<storage, read_write> instances: array<InstanceInput>; 


@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>){
    // instances[global_id.x].color.x = v_indices[global_id.x].x;
    // instances[global_id.x].color.y = v_indices[global_id.x].y;
    // instances[global_id.x].color.z = v_indices[global_id.x].z;

    // instances[global_id.x].color = instances[global_id.x].color;
    instances[global_id.x].color = vec4<f32>(v_indices[global_id.x], instances[global_id.x].color.w); 
}