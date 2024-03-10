use bevy::prelude::*;

use crate::{ai::movement::TurnToDestinationBehavior, movement::TurnSpeed};

use super::{
    effects::{Effector, Instigator},
    Target, Team,
};

#[derive(Component, Copy, Clone)]
pub struct Projectile {
    pub reached_target: bool,
}
impl Default for Projectile {
    fn default() -> Self {
        Self::new()
    }
}

impl Projectile {
    pub fn new() -> Self {
        Projectile {
            reached_target: false,
        }
    }
}

#[derive(Component, Copy, Clone)]
pub struct CircularHitBox {
    pub radius: f32,
}

#[derive(Component, Default, Copy, Clone)]
pub struct Homing;

// A homing missle:
// - Projectile
// - Target
// - Effector
// - Effectiveness
// - Instigator
// - MovementBundle
// - TurnToDestinationBehavior
// - Homing

pub fn update_homing_projectile_position_target(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut TurnToDestinationBehavior,
            &Target,
            &mut TurnSpeed,
        ),
        (With<Homing>, With<Projectile>),
    >,
    transforms: Query<&GlobalTransform>,
) {
    for (entity, mut turn_to_destination, target, mut turn_speed) in query.iter_mut() {
        if let Some(target_entity) = target.0 {
            if let Ok(target_transform) = transforms.get(target_entity) {
                turn_to_destination.destination = target_transform.translation();
            } else {
                commands
                    .entity(entity)
                    .remove::<TurnToDestinationBehavior>();
                turn_speed.radians_per_second = 0.0;
            }
        } else {
            commands
                .entity(entity)
                .remove::<TurnToDestinationBehavior>();
            turn_speed.radians_per_second = 0.0;
        }
    }
}

pub fn check_projectiles_reached_target(
    mut query: Query<(&Target, &GlobalTransform, &mut Projectile)>,
    transforms: Query<(&GlobalTransform, &CircularHitBox)>,
) {
    for (target, transform, mut projectile) in query.iter_mut() {
        if projectile.reached_target {
            continue;
        }
        let Some(target_entity) = target.0 else {
            continue;
        };
        let Ok((target_transform, hit_box)) = transforms.get(target_entity) else {
            continue;
        };
        let r2 = (target_transform.translation() - transform.translation()).length_squared();
        if r2 < hit_box.radius.powi(2) {
            projectile.reached_target = true;
        }
    }
}

pub fn projectiles_apply_effects(mut query: Query<(&mut Effector, &Projectile)>) {
    for (mut effector, projectile) in query.iter_mut() {
        if projectile.reached_target {
            effector.number_to_apply = 1;
        }
    }
}

pub fn despawn_projectiles(mut commands: Commands, mut query: Query<(Entity, &Projectile)>) {
    for (entity, projectile) in query.iter_mut() {
        if projectile.reached_target {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn initialise_projectiles(
    mut commands: Commands,
    query: Query<(Entity, &Instigator), Added<Projectile>>,
    launcher_query: Query<(Option<&Team>, &Target)>,
) {
    for (entity, instigator) in query.iter() {
        let Ok((team_opt, target)) = launcher_query.get(instigator.0) else {
            continue;
        };
        if let Some(team) = team_opt {
            commands.entity(entity).insert((*team, *target));
        } else {
            commands.entity(entity).insert(*target);
        }
    }
}
