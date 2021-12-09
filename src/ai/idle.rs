use bevy::prelude::*;
use super::movement::TurnToDestinationBehavior;
pub struct IdleBehavior;
use rand::Rng;
use crate::combat::Target;
use crate::ai::movement::PursueBehavior;

pub struct RoamBehavior {
    pub centre: Vec3,
    pub radius: f32
}

pub const ARRIVAL_TOLERANCE : f32 = 10.0;

pub fn do_roaming(mut query: Query<(
    &GlobalTransform,
    &RoamBehavior,
    &mut TurnToDestinationBehavior
), With<IdleBehavior>>) {
    let mut rng = rand::thread_rng();

    for (transform, roam, mut turn_to_destination) in query.iter_mut() {

        let delta = turn_to_destination.destination - transform.translation;
        
        if delta.length_squared() > ARRIVAL_TOLERANCE * ARRIVAL_TOLERANCE {
            let radius = rng.gen_range(0.0..roam.radius);
            let angle = 2.0 * rng.gen_range(0.0..std::f32::consts::PI);
            let new_target = roam.centre + radius * angle.cos() * Vec3::X + radius * angle.sin() * Vec3::Y;
            turn_to_destination.destination = new_target;
        }
    }
}

pub fn idle_to_combat(
    mut commands: Commands,
    query: Query<
        (Entity, &Target),
        (With<IdleBehavior>, Without<PursueBehavior>, Changed<Target>)
        > 
) {
    for (entity, target) in query.iter() {
        if target.0.is_some() {
            commands.entity(entity).insert(PursueBehavior::default());
            commands.entity(entity).remove::<IdleBehavior>();
        }
    }
}