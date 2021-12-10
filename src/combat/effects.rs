//! Effects that can be applied to entities, e.g. attacks or healing.

use bevy::prelude::*;

pub enum AttackResult {
    Hit,
    Miss
}

pub struct Attack { 
    pub accuracy: f32,
    pub result: AttackResult,
}

impl Attack {
    pub fn new(accuracy: f32) -> Self {
        Attack {
            accuracy,
            result: AttackResult::Hit
        }
    }
}

pub struct SourceLocation(Vec3);

pub struct InstantEffect {
    pub effect_template: Entity,
    pub accuracy: f32
}