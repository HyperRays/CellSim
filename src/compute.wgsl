var<push_constant> grid: vec2<u32>;

struct Comp {
    state: u32,
    copy: u32,
    _pad: vec2<u32>,
}   

@group(0)
@binding(0)
var<storage, read_write> v_indices: array<Comp>; 

fn get_idx(pos: vec2<u32>) -> u32{
    return pos.x + pos.y * grid.x;
}

fn check_bounds(pos: vec2<u32>) -> bool {
    let x = (pos.x < grid.x) & (pos.x >= 0);
    let y = (pos.y < grid.y) & (pos.y >= 0);
    return x & y;
}

@compute
@workgroup_size(10,10)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>){
    let idx = get_idx(global_id.xy);
    let cell = v_indices[idx];

    let neighbors: array<vec2<u32>, 4> = array( 
        vec2<u32>(global_id.x,global_id.y+1),
        vec2<u32>(global_id.x+1,global_id.y),
        vec2<u32>(global_id.x,global_id.y-1),
        vec2<u32>(global_id.x-1,global_id.y),
    );

    let in_bounds: array<bool, 4> = array(check_bounds(neighbors[0]),
                                        check_bounds(neighbors[1]),
                                        check_bounds(neighbors[2]), 
                                        check_bounds(neighbors[3]));


    v_indices[idx].copy = cell.state;

    // if cell.state == 200 {
        // var ill_neigh: u32 = 0;
        // var uill_neigh: u32 = 0;
        // var sum_neigh: u32 = 0;

        // if in_bounds[0] { 
        //     let neigh_val = v_indices[get_idx(neighbors[0])].state;
        //     ill_neigh += u32(neigh_val > 0 & neigh_val < 200);
        //     uill_neigh += u32(neigh_val == 200);     
        //     sum_neigh += neigh_val;
        // }
        // if in_bounds[1] { 
        //     let neigh_val = v_indices[get_idx(neighbors[1])].state;
        //     ill_neigh += u32(neigh_val > 0 & neigh_val < 200);     
        //     uill_neigh += u32(neigh_val == 200);
        //     sum_neigh += neigh_val;
        // }
        // if in_bounds[2] { 
        //     let neigh_val = v_indices[get_idx(neighbors[2])].state;
        //     ill_neigh += u32(neigh_val > 0 & neigh_val < 200);    
        //     uill_neigh += u32(neigh_val == 200); 
        //     sum_neigh += neigh_val;
        // }
        // if in_bounds[3] { 
        //     let neigh_val = v_indices[get_idx(neighbors[3])].state;
        //     ill_neigh += u32(neigh_val > 0 & neigh_val < 200); 
        //     uill_neigh += u32(neigh_val == 200); 
        //     sum_neigh += neigh_val;
        // }

        // var new_state: u32 = ill_neigh / 3 + uill_neigh / 3;
        // let s = new_state + sum_neigh;
        // new_state = (s / (ill_neigh + uill_neigh + 1)) + 42;
        // if new_state > 200 {
        //     new_state = u32(200);
        // }
        
        // v_indices[idx].copy = new_state; 

    // }
}