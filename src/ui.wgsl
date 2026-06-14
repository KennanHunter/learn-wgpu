struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct InstanceInput {
    @location(1) character: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>
}

@vertex
fn vs_main(
    vertices: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    // For now we just draw on top of everything 
    out.clip_position = vec4(vertices.position, 1.0);

    return out;
}

@group(0) @binding(0)
var t_atlas: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let object_color: vec4<f32> = textureSample(t_atlas, s_diffuse, in.tex_coords);

    return object_color;
}
 