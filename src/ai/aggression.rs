use bevy::prelude::*;

use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct AgentCategory: u32 {
        const FIGHTER = 0b00000001;
        const FRIGATE = 0b00000010;
        const CRUISER = 0b00000100;
        const TURRET  = 0b00001000;
        const MISSILE = 0b00010000;
    }
}

pub struct AggroRadius(f32);
pub struct AggroLocation(Vec3);
pub struct RetargetBehavior;

pub const MAX_AGGRO_RADIUS : f32 = 80.0;