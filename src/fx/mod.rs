//! Special effects and particle systems.

pub mod explosion;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::combat::{damage::Damage, effects::EffectLocation};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(create_explosions_for_damage.system());
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
        commands.spawn().insert(explosion::CreateExplosion {
            
            translation: location.0 + Vec3::new(x_offset, y_offset, 0.1)
        });
    }
}