use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d
};

use crate::combat::Team;

#[derive(AsBindGroup, TypePath, Clone, Asset)]
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

pub fn set_ship_shader_team_color(
    query: Query<(&Handle<ShipMaterial>, &Team), Changed<Team>>,
    mut materials: ResMut<Assets<ShipMaterial>>
) {
    for (handle, team) in query.iter() {
        let color = match team.0 {
            1 => Color::rgb(0.8, 0.2, 0.2),
            2 => Color::rgb(0.2, 0.2, 0.8),
            _ => Color::rgb(0.2, 0.2, 0.2),
        };
        match materials.get_mut(handle) {
            None => {},
            Some(material) => { material.color = color; }
        };
    }
}