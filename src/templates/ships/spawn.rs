//! Commands to spawn npcs.
//! 
//! To spawn an entity, insert a SpawnBundle of the required type into the world.

use bevy::prelude::*;

use crate::{combat::Team, materials::ShipMaterial};

#[derive(Bundle)]
pub struct SpawnBundle<T> where T : SpawnShipTemplate + Send + Sync + Component {
    pub spawn: T,
    pub transform: Transform,
    pub team: Team,
}

/// Spawns new entities according to a template.
pub trait SpawnShipTemplate {
    /// Resources used to spawn the entity.
    type Resources<'a> : Resource;

    /// Spawns a new entity.
    fn spawn(&self, commands: &mut Commands, input: &Res<Self::Resources<'_>>, materials: &mut ResMut<Assets<ShipMaterial>>) -> Entity;
}

pub fn spawn_ships_and_despawn_spawn_commands<T>(
    mut commands: Commands,
    resources: Res<T::Resources<'_>>,
    query: Query<(Entity, &T, &Transform, &Team)>,
    mut materials: ResMut<Assets<ShipMaterial>>
) where T : Component + Send + Sync + SpawnShipTemplate {
    for (spawner_entity, spawn, transform, team) in query.iter() {
        let transform = Transform {
            translation: transform.translation,
            scale: Vec3::splat(0.5),
            ..default()
        };
        let created = spawn.spawn(&mut commands, &resources, &mut materials);
        commands
            .entity(created)
            .insert(*team)
            .insert(transform)
            .insert(Into::<GlobalTransform>::into(transform));
        commands.entity(spawner_entity).despawn()
    }
}