var<push_constant> grid: array<u32, 5>;

struct Comp {
    state: u32,
    copy: u32,
    _pad: vec2<u32>,
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


fn get_idx(x: u32,y: u32) -> u32{
    return x + y * grid[0];
}

@compute
@workgroup_size(1,1,1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>){
    let idx = get_idx(global_id.x,global_id.y);
    instances[idx].color = vec3<f32>(0.4,0,0.8) * 1.0/200.0 * f32(v_indices[idx].copy);
    v_indices[idx].state = v_indices[idx].copy;
}