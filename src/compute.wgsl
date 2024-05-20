var<push_constant> grid: vec2<u32>;

struct Comp {
    color: vec3<f32>,
    state: u32,
}   

@group(0)
@binding(0)
var<storage, read_write> v_indices: array<Comp>; 

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>){
    let idx = global_id.x + global_id.y;
    v_indices[idx].color = vec3<f32>(0.1,0.74,0.4);
}