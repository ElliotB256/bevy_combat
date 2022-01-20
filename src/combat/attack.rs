
use bevy::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum AttackResult {
    Hit,
    Miss,
    Blocked
}

#[derive(Component)]
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

