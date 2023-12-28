//! Special effects and particle systems.

pub mod animated;
pub mod beams;
pub mod damage_flash;
pub mod death;

use bevy::prelude::*;
use rand::Rng;

use crate::combat::{
    attack::{Attack, AttackResult},
    effects::EffectLocation,
    CombatSystems,
};

use self::animated::AnimatedEffects;

/// An effect that spawns when an effect hits a target.
#[derive(Component)]
pub struct HitEffect {
    pub effect: AnimatedEffects,
}

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                create_hit_effects.after(CombatSystems),
                death::do_death_effects.after(crate::combat::mortal::update_dieing),
                damage_flash::update_damage_flashes,
            ),
        );
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

        let x_offset: f32 = rng.gen_range(-6.0..6.0);
        let y_offset: f32 = rng.gen_range(-6.0..6.0);
        commands.spawn(animated::CreateAnimatedEffect {
            effect: effect.effect,
            transform: Transform::from_translation(location.0 + Vec3::new(x_offset, y_offset, 0.1)),
            parent: None,
        });
    }
}
