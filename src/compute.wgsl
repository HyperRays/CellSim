var<push_constant> grid: array<u32, 5>;

struct Comp {
    state: u32,
    copy: u32,
    _pad: vec2<u32>,
}   

@group(0)
@binding(0)
var<storage, read_write> v_indices: array<Comp>; 

fn get_idx(pos: vec2<u32>) -> u32{
    return pos.x + pos.y * grid[0];
}

fn check_bounds(pos: vec2<u32>) -> bool {
    let x: bool = (pos.x < grid[0]) & (pos.x >= u32(0));
    let y: bool = (pos.y < grid[1]) & (pos.y >= u32(0));
    return x & y;
}

@compute
@workgroup_size(1,1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>){
    let idx = get_idx(global_id.xy);
    let cell = v_indices[idx];

    let max_state: u32 = u32(200);

    let neighbors: array<vec2<u32>, 8> = array( 
        vec2<u32>(global_id.x,global_id.y+1),
        vec2<u32>(global_id.x+1,global_id.y),
        vec2<u32>(global_id.x,global_id.y-1),
        vec2<u32>(global_id.x-1,global_id.y),

        vec2<u32>(global_id.x+1,global_id.y+1),
        vec2<u32>(global_id.x+1,global_id.y-1),
        vec2<u32>(global_id.x-1,global_id.y-1),
        vec2<u32>(global_id.x-1,global_id.y+1),
    );

    let in_bounds: array<bool, 8> = array(check_bounds(neighbors[0]),
                                        check_bounds(neighbors[1]),
                                        check_bounds(neighbors[2]), 
                                        check_bounds(neighbors[3]),
                                        check_bounds(neighbors[4]),
                                        check_bounds(neighbors[5]),
                                        check_bounds(neighbors[6]), 
                                        check_bounds(neighbors[7]));


    // v_indices[idx].copy = cell.state;

    if cell.state == max_state {
        v_indices[idx].copy = u32(0); 
    } else {
        var ill_neigh: u32 = u32(0);
        var uill_neigh: u32 = u32(0);
        var sum_neigh: u32 = u32(0);

        if in_bounds[0] { 
            let neigh_val = v_indices[get_idx(neighbors[0])].state;
            ill_neigh += u32((neigh_val > 0) && (neigh_val < max_state)); 
            uill_neigh += u32(neigh_val == max_state);     
            sum_neigh += neigh_val;
        }
        if in_bounds[1] { 
            let neigh_val = v_indices[get_idx(neighbors[1])].state;
            ill_neigh += u32((neigh_val > 0) && (neigh_val < max_state));      
            uill_neigh += u32(neigh_val == max_state);
            sum_neigh += neigh_val;
        }
        if in_bounds[2] { 
            let neigh_val = v_indices[get_idx(neighbors[2])].state;
            ill_neigh += u32((neigh_val > 0) && (neigh_val < max_state));  
            uill_neigh += u32(neigh_val == max_state); 
            sum_neigh += neigh_val;
        }
        if in_bounds[3] { 
            let neigh_val = v_indices[get_idx(neighbors[3])].state;
            ill_neigh += u32((neigh_val > 0) && (neigh_val < max_state)); 
            uill_neigh += u32(neigh_val == max_state); 
            sum_neigh += neigh_val;
        }
        if in_bounds[4] { 
            let neigh_val = v_indices[get_idx(neighbors[4])].state;
            ill_neigh += u32((neigh_val > 0) && (neigh_val < max_state)); 
            uill_neigh += u32(neigh_val == max_state); 
            sum_neigh += neigh_val;
        }
        if in_bounds[5] { 
            let neigh_val = v_indices[get_idx(neighbors[5])].state;
            ill_neigh += u32((neigh_val > 0) && (neigh_val < max_state)); 
            uill_neigh += u32(neigh_val == max_state); 
            sum_neigh += neigh_val;
        }
        if in_bounds[6] { 
            let neigh_val = v_indices[get_idx(neighbors[6])].state;
            ill_neigh += u32((neigh_val > 0) && (neigh_val < max_state)); 
            uill_neigh += u32(neigh_val == max_state); 
            sum_neigh += neigh_val;
        }
        if in_bounds[7] { 
            let neigh_val = v_indices[get_idx(neighbors[7])].state;
            ill_neigh += u32((neigh_val > 0) && (neigh_val < max_state)); 
            uill_neigh += u32(neigh_val == max_state); 
            sum_neigh += neigh_val;
        }

        var new_state: u32 = u32(0);
        if cell.state == 0 {
            new_state = u32(f32(ill_neigh) / f32(grid[2])) + u32(f32(uill_neigh) / f32(grid[3]));
        } else {
            let s = cell.state + sum_neigh;
            new_state = u32(f32(s) / f32(ill_neigh + uill_neigh + 1)) + grid[4];
        }

        if new_state > max_state {
            new_state = max_state;
        }
        
        v_indices[idx].copy = new_state;
        v_indices[idx]._pad.x = ill_neigh;

    }
}