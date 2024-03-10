use bevy::prelude::*;

use crate::game::GameTimeDelta;

/// An entity with a finite lifetime before expiration.
#[derive(Copy, Clone, Component)]
pub struct Lifetime { 
    pub seconds_remaining: f32
}

/// Indicates that an entity with a `Lifetime` has expired.
#[derive(Copy, Clone, Component)]
pub struct Expired;

pub fn update_lifetimes(
    mut query: Query<(Entity, &mut Lifetime)>,
    expired: Query<Entity, With<Expired>>,
    delta_time: Res<GameTimeDelta>,
    mut commands: Commands
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.seconds_remaining -= delta_time.0;
        if lifetime.seconds_remaining < 0.0 {
            commands.entity(entity).insert(Expired).remove::<Lifetime>();
        }
    }

    for entity in expired.iter() {
        commands.entity(entity).despawn_recursive()
    }
}