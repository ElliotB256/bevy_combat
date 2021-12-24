//! Illustrate laser beam attacks.

use bevy::prelude::*;

use crate::combat::{effects::{SourceTransform, EffectLocation}};

use super::animated::{CreateAnimatedEffect, AnimatedEffects};

pub struct BeamEffectPlugin;

impl Plugin for BeamEffectPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(spawn_beams.system());
    }
}

fn get_transform(start: Vec3, end: Vec3, width: f32) -> Transform {
    let delta = end-start;
    let angle = delta.y.atan2(delta.x) + std::f32::consts::FRAC_PI_2;
    let scale = Vec3::new(width, delta.length()/4.0, 1.0);
    
    Transform::from_translation((end+start)/2.0)
    * Transform::from_rotation(Quat::from_rotation_z(angle))
    * Transform::from_scale(scale)
}

fn spawn_beams(
    mut commands: Commands,
    query: Query<(&BeamStyle, &SourceTransform, &EffectLocation)>,
) {
    for (style, source, effect) in query.iter() {
        let transform = get_transform(source.0.translation, effect.0, style.width);
        commands.spawn().insert(
            CreateAnimatedEffect {
                transform: transform,
                effect: style.effect
            }
        );
    }
}

pub struct BeamStyle {
    pub effect: AnimatedEffects,
    pub width: f32
}