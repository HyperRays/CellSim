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

fn value_to_rgb(value: f32) -> vec3<f32> {
    // Ensure value is within the range [0, 1]
    let clamped_value = clamp(value, 0.0, 1.0);

    // Scale value to [0, 3] range
    let scaled_value = clamped_value * 3.0;
    
    // Determine the section and fractional part
    let section = floor(scaled_value);
    let fraction = fract(scaled_value);
    
    // Define the color transitions using vec3
    let color1 = vec3<f32>(255.0 / 255.0, 32.0 / 255.0, 78.0 / 255.0);   // rgb(255, 32, 78)
    let color2 = vec3<f32>(160.0 / 255.0, 21.0 / 255.0, 62.0 / 255.0); // rgb(160, 21, 62)
    let color3 = vec3<f32>(93.0 / 255.0, 14.0 / 255.0, 65.0 / 255.0); // rgb(93, 14, 65)
    let color4 = vec3<f32>(0.0 / 255.0, 34.0 / 255.0, 77.0 / 255.0); // rgb(0, 34, 77)
    
    // Interpolate colors based on the section
    let colorA = mix(color1, color2, fraction * step(0.0, section));
    let colorB = mix(color2, color3, fraction * step(1.0, section));
    let colorC = mix(color3, color4, fraction * step(2.0, section));
    
    let finalColor = mix(mix(colorA, colorB, step(1.0, section)), colorC, step(2.0, section));
    
    return finalColor;
}






@compute
@workgroup_size(1,1,1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>){
    let idx = get_idx(global_id.x,global_id.y);
    instances[idx].color = value_to_rgb( 1.0/200.0 * f32(v_indices[idx].copy) );
    v_indices[idx].state = v_indices[idx].copy;
}