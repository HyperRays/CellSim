@group(0)
@binding(0)
var<storage, read_write> v_indices: array<vec3<f32>>; 

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>){
    v_indices[global_id.x] = vec3<f32>(0.1,0.74,0.4);
}