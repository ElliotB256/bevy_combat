use bevy::core::FixedTimestep;
use bevy::input::{keyboard::KeyCode, Input};
use bevy::prelude::*;

use crate::constants::FIXED_TIME_STEP;

pub struct GameTimeDelta(pub f32);
pub struct GameSpeed(pub i32);

pub fn game_loop_run_criteria() -> FixedTimestep {
    FixedTimestep::step(FIXED_TIME_STEP as f64).with_label("fixed_update_run_criteria")
}

pub static DESPAWN_STAGE: &str = "despawn_stage";

#[derive(Default)]
pub struct BaseGamePlugin;

impl Plugin for BaseGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::Update,
            DESPAWN_STAGE,
            SystemStage::single_threaded(),
        );
        app.add_startup_system(startup);
        app.add_system(control_game_speed);
    }
}

fn startup(
    mut commands: Commands
) {
    commands.insert_resource(GameTimeDelta { 0: 1.0 / 60.0 });
    commands.insert_resource(GameSpeed { 0: 2 });
}

fn control_game_speed(
    keyboard_input: Res<Input<KeyCode>>,
    mut dt: ResMut<GameTimeDelta>,
    mut speed: ResMut<GameSpeed>,
) {
    if keyboard_input.just_pressed(KeyCode::Equals) {
        speed.0 += 1;
    }
    if keyboard_input.just_pressed(KeyCode::Minus) {
        speed.0 -= 1;
    }
    speed.0 = speed.0.max(0).min(3);
    dt.0 = FIXED_TIME_STEP * speed.0 as f32 / 2.0;
}
