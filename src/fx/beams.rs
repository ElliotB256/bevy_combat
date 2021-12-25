//! Illustrate laser beam attacks.

use bevy::prelude::*;

use crate::combat::{effects::{SourceTransform, EffectLocation, Instigator}, Target};

use super::animated::{CreateAnimatedEffect, AnimatedEffects};

pub struct BeamEffectPlugin;

impl Plugin for BeamEffectPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(spawn_beams.system());
        app.add_system(beams_track_target.system());
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
    query: Query<(&BeamStyle, &SourceTransform, &EffectLocation, &Target, &Instigator)>,
) {
    for (style, source, effect, target, instigator) in query.iter() {
        let transform = get_transform(source.0.translation, effect.0, style.width);
        commands.spawn().insert(
            CreateAnimatedEffect {
                transform: transform,
                effect: style.effect,
                parent: None
            }
        )
        .insert(
            BeamTracking {
                target: target.0.expect("no target"),
                source: instigator.0,
                start: source.0.translation,
                end: effect.0,
                width: style.width
            }
        );
    }
}

fn beams_track_target(
    mut query: Query<(&mut BeamTracking, &mut Transform)>,
    world_query: Query<&GlobalTransform>
) {
    for (mut tracking, mut transform) in query.iter_mut() {
        if let Ok(start_t) = world_query.get_component::<GlobalTransform>(tracking.source) {
            tracking.start = start_t.translation;
        }
        if let Ok(end_t) = world_query.get_component::<GlobalTransform>(tracking.target) {
            tracking.end = end_t.translation;
        }
        *transform = get_transform(tracking.start, tracking.end, tracking.width);
    }
}

pub struct BeamStyle {
    pub effect: AnimatedEffects,
    pub width: f32
}

#[derive(Clone, Copy)]
pub struct BeamTracking {
    pub target: Entity,
    pub source: Entity,
    pub start: Vec3,
    pub end: Vec3,
    pub width: f32
}