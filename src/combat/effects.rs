//! Effects that can be applied to entities, e.g. attacks or healing.

use bevy::{prelude::*};

use super::{Target, tools::TargettedTool};

pub struct SourceLocation(pub Vec3);

/// The entity responsible for causing an effect.
pub struct Instigator(pub Entity);

/// The effectiveness of an effect. Effects start with an effectiveness of 1.0
pub struct Effectiveness(pub f32);

impl Default for Effectiveness {
    fn default() -> Self {
        Effectiveness { 0: 1.0 }
    }
}

type Spawner = fn(&Commands) -> Entity;

pub struct Effect {
    pub spawn_effect: Spawner
}

pub fn apply_effects (
    mut commands: Commands,
    query: Query<(
        Entity,
        &Target,
        &GlobalTransform,
        &TargettedTool,
        &Effect,
    )>,
    pos_query: Query<&GlobalTransform>,
) {

    for (entity, target, transform, tool, effect) in query.iter() {
        
        if !tool.firing || target.0.is_none() {
            continue;
        }

        // Spawn the effect
        let spawned = (effect.spawn_effect)(&commands);
        commands.entity(spawned).insert_bundle(
            (
                Target { 0: target.0 }, 
                Instigator { 0: entity },
                SourceLocation { 0: transform.translation },
                Effectiveness::default()
            )
        );

        // moved Attack::new to ammo creation.
        //             if (transforms.HasComponent(target.Value))
        //                 buffer.AddComponent(entityInQueryIndex, attack, new HitLocation { Position = transforms[target.Value].Position });
    }
}

/// Deletes old effect entities.
pub fn remove_old_effects (
    mut commands: Commands,
    query: Query<(
        Entity,
        &Effect,
    )>
) {
    for (entity, _effect) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}