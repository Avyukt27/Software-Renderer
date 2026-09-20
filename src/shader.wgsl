struct CameraUniform {
    matrix: mat4x4<f32>,
    position: vec3<f32>,
}

struct LightUniform {
    position: vec3<f32>,
    _pad1: f32,
    diffuse: vec3<f32>,
    _pad2: f32,
    specular: vec3<f32>,
    _pad3: f32,
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
    let roughness = textureSample(t_specular, s_specular, in.uv).r;

    let smoothness = 1.0 - roughness;
    let specular_strength = smoothness * smoothness;
    let shininess = max(2.0, smoothness * 128.0);

    let normal = normalize(in.normals);
    let view_dir = normalize(camera.position - in.world_position);

    var total_ambient = vec3<f32>(0.1, 0.1, 0.1);
    var total_diffuse = vec3<f32>(0.0);
    var total_specular = vec3<f32>(0.0);

    let light_count = arrayLength(&lights);
    for (var i = 0u; i < light_count; i++) {
        let light = lights[i];
        let light_vector = light.position - in.world_position;
        let light_dir = normalize(light_vector);
        let dist = length(light_vector);

        let attenuation = 1.0 / (1.0 + 0.1 * dist + 0.01 * (dist * dist));

        let diffuse = max(dot(normal, light_dir), 0.0) * light.diffuse * attenuation;
        let halfway_dir = normalize(light_dir + view_dir);
        let spec = pow(max(dot(normal, halfway_dir), 0.0), shininess);
        let specular = light.specular * spec * specular_strength * attenuation;

        total_diffuse += diffuse;
        total_specular += specular;
    }

    let result = (total_ambient + total_diffuse) * texture_colour.rgb + total_specular;
    return vec4<f32>(result, texture_colour.a);
}
