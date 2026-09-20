struct CameraUniform {
    matrix: mat4x4<f32>,
}

struct LightUniform {
    position: vec3<f32>,
    _pad1: f32,
    diffuse: vec3<f32>,
    _pad2: f32,
    ambient: vec3<f32>,
    ambient_strength: f32,
}

struct ModelUniform {
    matrix: mat4x4<f32>,
}

@group(0) @binding(0) var t_diffuse: texture_2d<f32>;
@group(0) @binding(1) var s_diffuse: sampler;
@group(0) @binding(2) var t_specular: texture_2d<f32>;
@group(0) @binding(3) var s_specular: sampler;

@group(1) @binding(0) var<uniform> camera: CameraUniform;
@group(2) @binding(0) var<storage, read> lights: array<LightUniform>;
@group(3) @binding(0) var<storage, read> models: array<ModelUniform>;

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
fn vs_main(in: VertexInput, @builtin(instance_index) instance_index: u32) -> VertexOutput {
    var out: VertexOutput;

    let model_matrix = models[instance_index].matrix;

    out.clip_position = camera.matrix * model_matrix * vec4<f32>(in.position, 1.0);
    out.colour = in.colour;
    out.uv = in.uv;
    out.normals = (model_matrix * vec4<f32>(in.normals, 0.0)).xyz;
    out.world_position = (model_matrix * vec4<f32>(in.position, 1.0)).xyz;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texture_colour = textureSample(t_diffuse, s_diffuse, in.uv) * in.colour;
    let specular_sample = textureSample(t_specular, s_specular, in.uv).r;

    let normal = normalize(in.normals);

    var total_ambient = vec3<f32>(0.0);
    var total_diffuse = vec3<f32>(0.0);

    let light_count = arrayLength(&lights);
    for (var i = 0u; i < light_count; i++) {
        let light = lights[i];
        let light_dir = normalize(light.position - in.world_position);

        let ambient = light.ambient * light.ambient_strength;
        let diffuse = max(dot(normal, light_dir), 0.0) * light.diffuse;

        total_ambient += ambient;
        total_diffuse += diffuse;
    }

    let result = (total_ambient + total_diffuse) * texture_colour.rgb;
    return vec4<f32>(result, texture_colour.a);
}
