//! Effects that can be applied to entities, e.g. attacks or healing.

use bevy::prelude::*;

use super::Target;

/// Transform of the effect source.
#[derive(Component)]
pub struct SourceTransform(pub GlobalTransform);

/// The location where an effect is applied.
#[derive(Component)]
pub struct EffectLocation(pub Vec3);

/// The entity responsible for causing an effect.
#[derive(Component, Clone, Copy)]
pub struct Instigator(pub Entity);

/// The effectiveness of an effect. Effects start with an effectiveness of 1.0
#[derive(Component)]
pub struct Effectiveness(pub f32);

impl Default for Effectiveness {
    fn default() -> Self {
        Effectiveness(1.0)
    }
}

type Spawner = fn(&mut Commands) -> Entity;

#[derive(Component)]
pub struct Effector {
    pub spawn_effect: Spawner,
    /// Number of applications to apply this simulation step.
    pub number_to_apply: u16,
}
impl Effector {
    pub fn new(spawn_effect: Spawner) -> Self {
        Effector {
            spawn_effect,
            number_to_apply: 0,
        }
    }
}

/// Marker component that indicates an entity is an effect spawned by an [Effector].
#[derive(Component)]
pub struct Effect;

pub fn apply_effects(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Target,
        &GlobalTransform,
        &mut Effector,
        Option<&Instigator>,
    )>,
    pos_query: Query<&GlobalTransform>,
) {
    for (entity, target, transform, mut effect, instigator_opt) in query.iter_mut() {
        let Some(target_entity) = target.0 else {
            continue;
        };
        if effect.number_to_apply == 0 {
            continue;
        };

        // Instigator chaining - any entity with an instigator component was caused by something else -
        // any of the effects produced by it will also be back propagated to the root source. If this entity
        // was not instigated by anything, then it is a new root.
        let instigator: Instigator = match instigator_opt {
            None => Instigator(entity),
            Some(source_instigator) => source_instigator.clone(),
        };

        while effect.number_to_apply > 0 {
            // Spawn the effect
            let spawned = (effect.spawn_effect)(&mut commands);
            commands.entity(spawned).insert((
                Target(Some(target_entity)),
                instigator,
                SourceTransform(*transform),
                transform.compute_transform(),
                *transform, 
                Effectiveness::default(),
                Effect,
            ));

            if let Ok(target_transform) = pos_query.get(target_entity) {
                commands
                    .entity(spawned)
                    .insert(EffectLocation(target_transform.translation()));
            }

            println!("transform = {:?}", transform);
            effect.number_to_apply -= 1;
        }
    }
}

/// Deletes old effect entities.
pub fn remove_old_effects(mut commands: Commands, query: Query<(Entity, &Effect)>) {
    for (entity, _effect) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
