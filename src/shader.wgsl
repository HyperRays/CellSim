// Vertex shader

var<push_constant> scale: vec2<f32>;


struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct InstanceInput {
    @location(1) inst_pos: vec3<f32>,
    @location(2) scale: f32,
    @location(3) color: vec3<f32>,
    @location(4) _pad: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    inst: InstanceInput
) -> VertexOutput {
    var out: VertexOutput;
    out.color = inst.color;

    // scale the points to have physical window size
    var scaled_points = (model.position)*inst.scale;
    //shift the points according to instance postion
    var shifted = (scaled_points + inst.inst_pos);
    //scale the points back to 1.0/-1.0 to fit in viewport
    var normalised = shifted/vec3<f32>(scale, 1.0);
    // shift entire screen, so that the top left corner is 0,0
    var view = normalised + vec3<f32>(-1.0,1.0,0.0);

    out.clip_position = vec4<f32>(view, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}