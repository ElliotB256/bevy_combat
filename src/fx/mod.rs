//! Special effects and particle systems.

pub mod animated;
pub mod death;
pub mod beams;
pub mod damage_flash;

use bevy::prelude::*;
use rand::{Rng};

use crate::{combat::{effects::{EffectLocation}, CombatSystems, attack::{Attack, AttackResult}}, game::game_loop_run_criteria};

use self::animated::AnimatedEffects;

/// An effect that spawns when an effect hits a target.
#[derive(Component)]
pub struct HitEffect {
    pub effect: AnimatedEffects
}

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_hit_effects.after(CombatSystems::Set));
        //app.add_system(create_muzzle_flares);
        app.add_system(death::do_death_effects.after(crate::combat::mortal::MortalSystems::UpdateDieing).with_run_criteria(game_loop_run_criteria()));
        app.add_system(damage_flash::update_damage_flashes.with_run_criteria(game_loop_run_criteria()));
    }
}

fn create_hit_effects(
    mut commands: Commands,
    query: Query<(&HitEffect, &EffectLocation, &Attack)>,
) {
    let mut rng = rand::thread_rng();

    for (effect, location, attack) in query.iter() {

        if attack.result == AttackResult::Miss {
            continue;
        }

        let x_offset : f32 = rng.gen_range(-6.0..6.0);
        let y_offset : f32 = rng.gen_range(-6.0..6.0);
        commands.spawn().insert(animated::CreateAnimatedEffect {
            effect: effect.effect,
            transform: Transform::from_translation(location.0 + Vec3::new(x_offset, y_offset, 0.1)),
            parent: None
        });
    }
}