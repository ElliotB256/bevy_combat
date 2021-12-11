//! Weapons that can deal damage.
//! 

use bevy::{prelude::*};

use crate::combat::{attack::Attack, damage::Damage};

/// The attack from a small pulsed laser.
pub fn small_pulse_laser_attack(commands: &mut Commands) -> Entity {
    commands.spawn().insert_bundle(
        (
            Attack::new(3.0),
            Damage::new(10.0)
        )
    ).id()
}