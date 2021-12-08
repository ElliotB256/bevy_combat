use bevy::prelude::*;

pub struct Target(pub Option<Entity>);

impl Default for Target {
    fn default() -> Self {
        Target { 0: None }
    }
}

pub struct Health(pub f32);
pub struct MaxHealth(pub f32);
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Team(pub i32);