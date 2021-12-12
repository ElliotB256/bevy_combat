//! Death effects for entities.

use bevy::prelude::*;
use rand::Rng;

use crate::{game::{GameTimeDelta}, combat::mortal::Dieing};

use super::animated::{AnimatedEffects, CreateAnimatedEffect};

//pub struct DeathEffectsPlugin;
// impl Plugin for DeathEffectsPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.add_system(do_death_effects.system().with_run_criteria(game_loop_run_criteria()));
//     }
// }

/// Generates effects while the entity is dieing.
pub struct DeathEffect {
    /// Remaining time to wait until launching another explosion.
    pub time_to_explosion: f32,
    pub dying_explosion: AnimatedEffects,
    pub death_explosion: AnimatedEffects
}

pub fn do_death_effects(
    mut commands: Commands,
    dt: Res<GameTimeDelta>,
    mut query: Query<(&mut DeathEffect, &GlobalTransform, &Dieing)>
) {
    let mut rng = rand::thread_rng();
    for (mut death_effect, transform, dieing) in query.iter_mut() {
        death_effect.time_to_explosion -= dt.0;
        
        if death_effect.time_to_explosion < 0.0 {
            let x_offset : f32 = rng.gen_range(-6.0..6.0);
            let y_offset : f32 = rng.gen_range(-6.0..6.0);
            death_effect.time_to_explosion = rng.gen_range(0.05..0.2);
            commands.spawn().insert(CreateAnimatedEffect {
                effect: death_effect.dying_explosion.clone(),
                transform: Transform::from_translation(transform.translation + Vec3::new(x_offset, y_offset, 0.1))
            });
        }

        if dieing.dead {
            commands.spawn().insert(CreateAnimatedEffect {
                effect: death_effect.death_explosion.clone(),
                transform: Transform::from_translation(transform.translation)
            });
        }
    }
}
