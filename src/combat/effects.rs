//! Effects that can be applied to entities, e.g. attacks or healing.

use bevy::{prelude::*};

use super::{Target, tools::TargettedTool};

/// Transform of the effect source.
#[derive(Component)]
pub struct SourceTransform(pub GlobalTransform);

/// The location where an effect is applied.
#[derive(Component)]
pub struct EffectLocation(pub Vec3);

/// The entity responsible for causing an effect.
#[derive(Component)]
pub struct Instigator(pub Entity);

/// The effectiveness of an effect. Effects start with an effectiveness of 1.0
#[derive(Component)]
pub struct Effectiveness(pub f32);

impl Default for Effectiveness {
    fn default() -> Self {
        Effectiveness { 0: 1.0 }
    }
}

type Spawner = fn(&mut Commands) -> Entity;

#[derive(Component)]
pub struct Effector {
    pub spawn_effect: Spawner
}

#[derive(Component)]
pub struct Effect;

pub fn apply_effects (
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Target,
        &GlobalTransform,
        &mut TargettedTool,
        &Effector,
    )>,
    pos_query: Query<&GlobalTransform>,
) {

    for (entity, target, transform, mut tool, effect) in query.iter_mut() {
        
        if !tool.firing || target.0.is_none() {
            continue;
        }

        tool.firing = false;

        // Spawn the effect
        let spawned = (effect.spawn_effect)(&mut commands);
        commands.entity(spawned).insert_bundle(
            (
                Target { 0: target.0 }, 
                Instigator { 0: entity },
                SourceTransform { 0: *transform },
                Effectiveness::default(),
                Effect
            )
        );

        if let Ok(target_transform) = pos_query.get_component::<GlobalTransform>(target.0.expect("target is none")) {
            commands.entity(spawned).insert(
                EffectLocation { 0: target_transform.translation() }
            );
        }
        
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

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum EffectSystems {
    RemoveOldEffects,
    ApplyEffects
}