struct CameraUniform {
    matrix: mat4x4<f32>,
}

struct LightUniform {
    position: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;
@group(1) @binding(0)
var<uniform> light: LightUniform;

@group(2) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(2) @binding(1)
var s_diffuse: sampler;
@group(2) @binding(2)
var t_specular: texture_2d<f32>;
@group(2) @binding(3)
var s_specular: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) colour: vec4<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) normals: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) normals: vec3<f32>,
    @location(3) world_position: vec3<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.matrix * vec4<f32>(in.position, 1.0);
    out.colour = in.colour;
    out.uv = in.uv;
    out.normals = in.normals;
    out.world_position = in.position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texture_colour = textureSample(t_diffuse, s_diffuse, in.uv) * in.colour;
    let specular_sample = textureSample(t_specular, s_specular, in.uv).r;

    let normal = normalize(in.normals);
    let light_dir = normalize(light.position.xyz - in.world_position);

    let ambient_strength = 0.1;
    let ambient = ambient_strength * vec3<f32>(1.0, 1.0, 1.0);

    let diffuse = max(dot(normal, light_dir), 0.0) * vec3<f32>(1.0, 1.0, 1.0);

    let result = (ambient + diffuse) * texture_colour.rgb;
    return vec4<f32>(result, texture_colour.a);
}
