//! Implements AI for moving and steering entities.

use super::IdleBehavior;
use crate::combat::Target;
use crate::constants::FIXED_TIME_STEP;
use crate::math_util::*;
use crate::movement::{Heading, MaxTurnSpeed, TurnSpeed};
use bevy::prelude::*;

/// Indicates that an entity should turn towards a destination.
pub struct TurnToDestinationBehavior {
    pub destination: Vec3,
}

pub struct PursueBehavior;
pub const PROXIMITY_RADIUS: f32 = 4.0;

/// Turns entities with a [TurnToDestinationBehavior](TurnToDestinationBehavior.struct.html) towards their destination.
fn turn_to_destination(
    mut query: Query<(
        &TurnToDestinationBehavior,
        &GlobalTransform,
        &MaxTurnSpeed,
        &Heading,
        &mut TurnSpeed,
    )>,
) {
    for (behavior, transform, max_turn_speed, heading, mut turn_speed) in query.iter_mut() {
        // // Determine desired heading to target
        let delta = behavior.destination - transform.translation;
        let desired_heading = get_heading_to_point(delta);

        // Adjust rotation speed to aim for desired heading.
        let diff = get_angle_difference(desired_heading, heading.radians);
        turn_speed.radians_per_second = diff.signum()
            * max_turn_speed
                .radians_per_second
                .min(diff.abs() / FIXED_TIME_STEP);
    }
}

/// Entity pursues their target.
fn pursue(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &PursueBehavior,
        &Target,
        &GlobalTransform,
        &mut TurnToDestinationBehavior,
    )>,
    pos_query: Query<&GlobalTransform>,
) {
    for (entity, _pursue, target, transform, mut turn_to) in query.iter_mut() {
        let result = pos_query.get_component::<GlobalTransform>(target.0);
        match result {
            Err(_) => {
                // target does not have position. Go to idle state
                commands.entity(entity).remove::<PursueBehavior>();
                commands.entity(entity).insert(IdleBehavior);
                continue;
            }
            Ok(target_transform) => {
                turn_to.destination = target_transform.translation;
                // if too close to target, evasive manoeuvre
                let delta = target_transform.translation - transform.translation;
                if delta.length_squared() < PROXIMITY_RADIUS * PROXIMITY_RADIUS {
                    commands
                        .entity(entity)
                        .remove_bundle::<(TurnToDestinationBehavior, PursueBehavior)>();
                    commands.entity(entity).insert(PeelManoeuvreBehavior);
                }
            }
        }
    }
}

/// A 'peel' manoeuvre causes an entity to move away from its target.
/// 
/// It is usually triggered when the entity gets too close.
pub struct PeelManoeuvreBehavior;