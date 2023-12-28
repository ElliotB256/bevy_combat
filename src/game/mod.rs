use bevy::input::{keyboard::KeyCode, Input};
use bevy::prelude::*;

use crate::constants::FIXED_TIME_STEP;

#[derive(Resource)]
pub struct GameTimeDelta(pub f32);

#[derive(Resource)]
pub struct GameSpeed(pub i32);

pub static DESPAWN_STAGE: &str = "despawn_stage";

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum DespawnSet {
    Parallel,
    CommandFlush,
}

// #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
// pub struct GameUpdateLogic;

// #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
// struct FixedUpdateCommandFlush;

#[derive(Default)]
pub struct BaseGamePlugin;

impl Plugin for BaseGamePlugin {
    fn build(&self, app: &mut App) {
        // app.configure_sets(
        //     FixedUpdate,
        //     (
        //         GameUpdateLogic,
        //         FixedUpdateCommandFlush,
        //         DespawnSet::Parallel,
        //         DespawnSet::CommandFlush,
        //     )
        //         .chain(),
        // );
        // app.add_system(FixedUpdateCommandFlush, apply_deferred);
        // app.add_system(DespawnSet::CommandFlush, apply_deferred);
        // app.add_startup_system(startup);
        // app.add_system(control_game_speed);
        app.add_systems(Startup,startup
        );
        app.add_systems(Update, control_game_speed);

    }
}

fn startup(mut commands: Commands) {
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
