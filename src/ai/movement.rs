//! Implements AI for moving and steering entities.

use crate::ai::idle::IdleBehavior;
use crate::combat::Target;
use crate::constants::FIXED_TIME_STEP;
use crate::math_util::*;
use crate::movement::{Heading, MaxTurnSpeed, TurnSpeed};
use bevy::prelude::*;

/// Indicates that an entity should turn towards a destination.
#[derive(Default, Component)]
pub struct TurnToDestinationBehavior {
    pub destination: Vec3,
}

#[derive(Default, Component)]
pub struct PursueBehavior;
pub const PROXIMITY_RADIUS: f32 = 64.0;

/// Turns entities with a [TurnToDestinationBehavior](TurnToDestinationBehavior.struct.html) towards their destination.
pub fn turn_to_destination(
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
        let delta = behavior.destination - transform.translation();
        let desired_heading = get_heading_to_point(delta);

        // Adjust rotation speed to aim for desired heading.
        let diff = get_angle_difference(desired_heading, heading.radians);
        turn_speed.radians_per_second = diff.signum()
            * max_turn_speed
                .radians_per_second
                .min(diff.abs() / FIXED_TIME_STEP);
        //println!("destination: {:?}, delta: {:?}.", behavior.destination, delta);
    }
    //println!("turn_to_destination: {:?} entities.", query.iter_mut().len());
}

/// Entity pursues their target.
pub fn pursue(
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
        
        if target.0.is_none() {
            continue;
        }

        let result = pos_query.get(target.0.expect("target is none"));
        
        match result {
            Err(_) => {
                // target does not have position. Go to idle state
                commands.entity(entity).remove::<PursueBehavior>();
                commands.entity(entity).insert(IdleBehavior);
                continue;
            }
            Ok(target_transform) => {
                turn_to.destination = target_transform.translation();
                
                // if too close to target, evasive manoeuvre
                let delta = target_transform.translation() - transform.translation();
                //println!("entity: {:?}, destination: {:?}, delta: {:?}.", target.0.expect("target"), turn_to.destination, delta);
                if delta.length_squared() < PROXIMITY_RADIUS * PROXIMITY_RADIUS {
                    commands
                        .entity(entity)
                        .remove::<(TurnToDestinationBehavior, PursueBehavior)>();
                    commands.entity(entity).insert(PeelManoeuvreBehavior);
                }
            }
        }
    }
    //println!("pursue: {:?} entities, {:?} err, {:?} ok.", query.iter_mut().len(), err_count, ok_count);
}

/// A 'peel' manoeuvre causes an entity to move away from its target.
/// 
/// It is usually triggered when the entity gets too close.
#[derive(Component)]
pub struct PeelManoeuvreBehavior;
const ENGAGEMENT_RADIUS: f32 = 128.0;

pub fn peel_manoeuvre(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &PeelManoeuvreBehavior,
        &Target,
        &GlobalTransform,
        &Heading,
        &MaxTurnSpeed,
        &mut TurnSpeed,
    )>,
    pos_query: Query<&GlobalTransform>
) {
    for (entity, _peel, target, transform, heading, max_turn_speed, mut turn_speed) in query.iter_mut() {
        
        if target.0.is_none() {
            continue;
        }

        let result = pos_query.get(target.0.expect("target is none"));
        
        match result {
            Err(_) => {
                // target does not have position. Disengage.
                commands.entity(entity).remove::<PeelManoeuvreBehavior>();
                commands.entity(entity).insert(IdleBehavior);
                commands.entity(entity).insert(TurnToDestinationBehavior::default());
                continue;
            }
            Ok(target_transform) => {
                // Turn away from the enemy.
                let mut delta = target_transform.translation() - transform.translation();
                delta.z = 0.0;
                let angle_diff = get_angle_difference(get_heading_to_point(delta), heading.radians);

                if angle_diff.abs() < 0.3 * std::f32::consts::PI {
                    turn_speed.radians_per_second = -max_turn_speed.radians_per_second * angle_diff.signum();
                }
                else {
                    turn_speed.radians_per_second = 0.0;
                }
                
                // Remain in evasive manoeuvre until a certain distance to target is reached.               
                if delta.length_squared() > ENGAGEMENT_RADIUS * ENGAGEMENT_RADIUS {
                    commands
                        .entity(entity)
                        .remove::<PeelManoeuvreBehavior>();
                    commands.entity(entity).insert(PursueBehavior);
                    commands.entity(entity).insert(TurnToDestinationBehavior::default());
                }
            }
        }
    }
    //println!("peel: {:?} entities, {:?} err, {:?} ok.", query.iter_mut().len(), err_count, ok_count);
}
