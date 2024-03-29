//! Weapons that can deal damage.
//!

use bevy::prelude::*;

use crate::{
    combat::{attack::Attack, damage::Damage},
    fx::{beams::BeamStyle, HitEffect},
};

/// The attack from a small pulsed laser.
pub fn pulse_laser_attack(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Attack::new(3.0),
            Damage::new(20.0),
            BeamStyle {
                effect: crate::fx::animated::AnimatedEffects::BlueLaserBeam,
                width: 1.0,
            },
            HitEffect {
                effect: crate::fx::animated::AnimatedEffects::SmallExplosion,
            },
        ))
        .id()
}

pub fn small_pulse_laser_attack(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Attack::new(2.0),
            Damage::new(2.0),
            BeamStyle {
                effect: crate::fx::animated::AnimatedEffects::GreenLaserBeam,
                width: 0.5,
            },
            HitEffect {
                effect: crate::fx::animated::AnimatedEffects::TinyPlusExplosion,
            },
        ))
        .id()
}

pub fn small_rocket_attack(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Attack::new(10.0),
            Damage::new(15.0),
            HitEffect {
                effect: crate::fx::animated::AnimatedEffects::FlashExplosion,
            },
        ))
        .id()
}