//! Special effects and particle systems.

pub mod animated;
pub mod death;

use bevy::prelude::*;
use rand::{Rng};

use crate::{combat::{damage::Damage, effects::{EffectLocation, Instigator}}, game::game_loop_run_criteria};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(create_explosions_for_damage.system());
        app.add_system(create_muzzle_flares.system());
        app.add_system(death::do_death_effects.system().after(crate::combat::mortal::MortalSystems::UpdateDieing).with_run_criteria(game_loop_run_criteria()));
    }
}

fn create_explosions_for_damage(
    mut commands: Commands,
    query: Query<(&Damage, &EffectLocation)>,
) {
    let mut rng = rand::thread_rng();

    for (_damage, location) in query.iter() {
        let x_offset : f32 = rng.gen_range(-6.0..6.0);
        let y_offset : f32 = rng.gen_range(-6.0..6.0);
        commands.spawn().insert(animated::CreateAnimatedEffect {
            effect: animated::AnimatedEffects::SmallExplosion,
            transform: Transform::from_translation(location.0 + Vec3::new(x_offset, y_offset, 0.1))
        });
    }
}

fn create_muzzle_flares(
    mut commands: Commands,
    query: Query<(&Damage, &Instigator)>,
) {
    for (_damage, instigator) in query.iter() {
        commands.spawn().insert(animated::CreateAnimatedEffect {
            effect: animated::AnimatedEffects::MuzzleFlare,
            transform: Transform::identity()
        })
        .insert(Parent { 0: instigator.0 });
    }
}