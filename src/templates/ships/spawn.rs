//! Commands to spawn npcs.
//!
//! To spawn an entity, insert a SpawnBundle of the required type into the world.

use bevy::prelude::*;

use crate::{
    combat::{effects::Instigator, Team},
    materials::ShipMaterial,
};

#[derive(Bundle)]
pub struct SpawnBundle<T>
where
    T: SpawnShipTemplate + Send + Sync + Component,
{
    pub spawn: T,
    pub transform: Transform,
    pub team: Team,
}

/// Spawns new entities according to a template.
pub trait SpawnShipTemplate {
    /// Resources used to spawn the entity.
    type Resources<'a>: Resource;

    /// Spawns a new entity.
    fn spawn(
        &self,
        commands: &mut Commands,
        input: &Res<Self::Resources<'_>>,
        materials: &mut ResMut<Assets<ShipMaterial>>,
    ) -> Entity;
}

/// Spawns entities for each entity with template `T`.
///
/// - The entity will be spawned at the given Transform.
/// - If the spawn command entity has a `Team` component, this will be copied to the new entity.
/// - If the spawn command has an `Instigator` component, this will be copied to the new entity.
/// - If the spawn command ha an `Instigator` but no `Team`, it will attempt to copy the instigator's team to the created entity.
pub fn spawn_ships_and_despawn_spawn_commands<T>(
    mut commands: Commands,
    resources: Res<T::Resources<'_>>,
    query: Query<(Entity, &T, &Transform, Option<&Team>, Option<&Instigator>)>,
    team_query: Query<&Team>,
    mut materials: ResMut<Assets<ShipMaterial>>,
) where
    T: Component + Send + Sync + SpawnShipTemplate,
{
    for (spawner_entity, spawn, transform, team_option, instigator_option) in query.iter() {
        let transform = Transform {
            translation: transform.translation,
            rotation: transform.rotation,
            scale: Vec3::splat(0.5),
            ..default()
        };
        println!("spawn: transform = {:?}", transform);
        let created = spawn.spawn(&mut commands, &resources, &mut materials);
        let mut entity_builder = commands.entity(created);
        entity_builder
            .insert(transform)
            .insert(Into::<GlobalTransform>::into(transform));
        if let Some(team) = team_option {
            entity_builder.insert(*team);
        }
        if let Some(instigator) = instigator_option {
            entity_builder.insert(*instigator);
            if let (Ok(alt_team), None) = (team_query.get(instigator.0), team_option) {
                entity_builder.insert(*alt_team);
            }
        }
        commands.entity(spawner_entity).despawn();
    }
}
