use bevy::core::FixedTimestep;
use bevy::prelude::*;

use crate::constants::FIXED_TIME_STEP;

pub struct GameTimeDelta(pub f32);

pub fn game_loop_run_criteria() -> FixedTimestep {
    FixedTimestep::step(FIXED_TIME_STEP as f64).with_label("fixed_update_run_criteria")
}

pub static DESPAWN_STAGE: &str = "despawn_stage";

#[derive(Default)]
pub struct BaseGamePlugin;

impl Plugin for BaseGamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_after(
            CoreStage::Update,
            DESPAWN_STAGE,
            SystemStage::single_threaded(),
        );
    }
}
