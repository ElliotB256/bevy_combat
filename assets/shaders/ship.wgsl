struct ShipMaterial {
    color: vec4<f32>,
    last_damaged_time: f32
};

@group(2) @binding(0)
var<uniform> material: ShipMaterial;
@group(2) @binding(1)
var base_texture: texture_2d<f32>;
@group(2) @binding(2)
var base_texture_sampler: sampler;
@group(2) @binding(3)
var color_mask: texture_2d<f32>;
@group(2) @binding(4)
var color_mask_sampler: sampler;

@fragment
fn fragment(
    //#import bevy_pbr::mesh_vertex_output
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    let white : vec4<f32> = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    var mask : vec4<f32> = textureSample(color_mask, color_mask_sampler, uv);
    var color : vec4<f32> = mix(white, material.color, mask[0]);
    var painted_ship : vec4<f32> = color * textureSample(base_texture, base_texture_sampler, uv);

    return mix(white, painted_ship, clamp(material.last_damaged_time * 3f, 0.0, 1.0)) * painted_ship.a;
}