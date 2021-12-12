use bevy::{core::FixedTimestep};

use crate::constants::FIXED_TIME_STEP;

pub struct GameTimeDelta(pub f32);

pub fn game_loop_run_criteria() -> FixedTimestep {
    FixedTimestep::step(FIXED_TIME_STEP as f64).with_label("fixed_update_run_criteria")
}