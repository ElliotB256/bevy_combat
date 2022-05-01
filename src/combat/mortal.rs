//! All things relating to mortality, life and death.

use bevy::prelude::*;
use rand::Rng;

use crate::game::GameTimeDelta;

#[derive(Component)]
pub struct Health(pub f32);
#[derive(Component)]
pub struct MaxHealth(pub f32);

/// Marks that an entity can die.
#[derive(Component)]
pub struct Mortal;

/// Indicates that an entity is in the process of dying.
/// 
/// This entity is doomed - there is no saving it. Call this the 'death throes' if you will.
#[derive(Component)]
pub struct Dieing {
    pub remaining_time: f32,
    pub dead: bool,
    pub dispose: bool
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
            commands.entity(entity).insert(Dieing {remaining_time: time, dead: false, dispose: false});
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
    mut query: Query<&mut Dieing>
) {
    for mut dieing in query.iter_mut() {
        dieing.remaining_time -= dt.0;
        if dieing.remaining_time < 0.0
        {
            if !dieing.dead
            {
                dieing.dead = true;
            } else {
                dieing.dispose = true;
            }
        }
    }
}

pub fn dispose_dieing(
    mut commands: Commands,
    query: Query<(Entity, &Dieing)>
) {
    for (entity, dieing) in query.iter() {
        if dieing.dispose {
            commands.entity(entity).despawn_recursive();
        }
    }
}