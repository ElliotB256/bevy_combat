//! All things relating to mortality, life and death.

use bevy::prelude::*;
use rand::Rng;

use crate::game::GameTimeDelta;

pub struct Health(pub f32);
pub struct MaxHealth(pub f32);

/// Marks that an entity can die.
pub struct Mortal;

/// Indicates that an entity is in the process of dying.
/// 
/// This entity is doomed - there is no saving it. Call this the 'death throes' if you will.
pub struct Dieing {
    pub remaining_time: f32,
    pub dead: bool
}


#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum MortalSystems {
    CheckForDieingEntities,
    UpdateDieing
}

pub fn check_for_dieing_entities(
    mut commands: Commands,
    query: Query<(Entity, &Health), (With<Mortal>, Without<Dieing>)>
) {
    let mut rng = rand::thread_rng();
    for (entity, health) in query.iter() {
        if health.0 <= 0.0 {
            // There's a chance things die instantly.
            let time = if rng.gen_range(0.0..1.0) < 0.3 {
                0.0
            } else {
                // There's a small chance things have a few seconds of death throes.
                rng.gen_range(1.0..4.0)
            };
            commands.entity(entity).insert(Dieing {remaining_time: time, dead: false});
        }
    }
}

/// Updates `Dieing` components.
/// 
/// Decreases the remaining time until it reaches zero.
/// Then, 'dead' is set to true.
/// The next update, the entity is despawned.
pub fn update_dieing(
    dt: Res<GameTimeDelta>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Dieing)>
) {
    for (entity, mut dieing) in query.iter_mut() {
        dieing.remaining_time -= dt.0;
        if dieing.remaining_time < 0.0
        {
            if !dieing.dead
            {
                dieing.dead = true;
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}