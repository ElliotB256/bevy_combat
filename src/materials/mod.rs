use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d
};

#[derive(AsBindGroup, TypeUuid, Clone, Asset, TypePath)]
#[uuid = "0e307091-2db9-40e7-ae85-cd890941101d"]
pub struct ShipMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub last_damaged_time: f32,
    #[texture(1)]
    #[sampler(2)]
    pub base_texture: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    pub color_mask: Handle<Image>,
}

impl Material2d for ShipMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ship.wgsl".into()
    }
}