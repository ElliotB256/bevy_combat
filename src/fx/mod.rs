//! Special effects and particle systems.

pub mod animated;
pub mod death;
pub mod beams;

use bevy::prelude::*;
use rand::{Rng};

use crate::{combat::{damage::Damage, effects::{EffectLocation, Instigator}}, game::game_loop_run_criteria};

use self::animated::AnimatedEffects;

/// An effect that spawns when an effect hits a target.
pub struct HitEffect {
    pub effect: AnimatedEffects
}

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(create_hit_effects.system());
        //app.add_system(create_muzzle_flares.system());
        app.add_system(death::do_death_effects.system().after(crate::combat::mortal::MortalSystems::UpdateDieing).with_run_criteria(game_loop_run_criteria()));
    }
}

fn create_hit_effects(
    mut commands: Commands,
    query: Query<(&HitEffect, &EffectLocation)>,
) {
    let mut rng = rand::thread_rng();

    for (effect, location) in query.iter() {
        let x_offset : f32 = rng.gen_range(-6.0..6.0);
        let y_offset : f32 = rng.gen_range(-6.0..6.0);
        commands.spawn().insert(animated::CreateAnimatedEffect {
            effect: effect.effect,
            transform: Transform::from_translation(location.0 + Vec3::new(x_offset, y_offset, 0.1)),
            parent: None
        });
    }
}

fn create_muzzle_flares(
    mut commands: Commands,
    query: Query<(&Damage, &Instigator)>,
    in_world_query: Query<&GlobalTransform>
) {
    for (_damage, instigator) in query.iter() {
        let parent = match in_world_query.get(instigator.0) {
            Ok(_) => Some(instigator.0),
            Err(_) => None,
        };
        commands.spawn().insert(animated::CreateAnimatedEffect {
            effect: animated::AnimatedEffects::MuzzleFlare,
            transform: Transform::identity(),
            parent
        }).id();
    }
}