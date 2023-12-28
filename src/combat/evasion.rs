//! Attacks can be evaded if the target is sufficiently fast moving

use bevy::prelude::*;
use rand::Rng;

use super::{
    attack::{Attack, AttackResult},
    Target,
};
use crate::movement::{MaxTurnSpeed, Speed, TurnSpeed};

/// An entity can evade incoming attacks.
#[derive(Component)]
pub struct Evasion {
    /// The base evasion rating for an entity.
    pub base: f32,
    /// The bonus evasion rating due to movement.
    pub movement_bonus: f32,
    /// The total evasion rating
    pub total: f32,
}

impl Evasion {
    pub fn new(base: f32) -> Self {
        Evasion {
            base,
            movement_bonus: 0.0,
            total: 0.0,
        }
    }
}

pub const TURN_EVASION_FACTOR: f32 = 2.0;
pub const SPEED_EVASION_FACTOR: f32 = 200.0;

/// Calculate evasion ratings based on entity linear and turn speed.
pub fn calculate_evasion_ratings(
    mut query: Query<(&mut Evasion, &TurnSpeed, &MaxTurnSpeed, &Speed)>,
) {
    for (mut evasion, turn_speed, max_turn_speed, speed) in query.iter_mut() {
        evasion.movement_bonus =
            (turn_speed.radians_per_second.abs() + max_turn_speed.radians_per_second.abs()) / 2.0
                * speed.0
                / (TURN_EVASION_FACTOR * SPEED_EVASION_FACTOR);
        evasion.total = evasion.base + evasion.movement_bonus;
    }
}

/// Calculate whether attacks are hit or miss.
pub fn determine_missed_attacks(
    mut attack_query: Query<(&mut Attack, &Target)>,
    target_query: Query<&Evasion>,
) {
    let mut rng = rand::thread_rng();

    for (mut attack, target) in attack_query.iter_mut() {
        if target.0.is_none() {
            continue;
        }

        if let Ok(evasion) = target_query.get(target.0.expect("target is none")) {
            let hit_chance = (-evasion.total / attack.accuracy).exp();
            if rng.gen_range(0.0..1.0) > hit_chance {
                attack.result = AttackResult::Miss;
            }
        }
    }
}
