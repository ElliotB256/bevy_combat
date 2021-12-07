use bevy::{core::FixedTimestep, prelude::*};
use crate::constants::FIXED_TIME_STEP;

pub struct Velocity(Vec3);
pub struct Speed(f32);
pub struct TurnSpeed {
    pub radians_per_second: f32,
}
pub struct MaxTurnSpeed {
    pub radians_per_second: f32,
}
pub struct Mass(f32);
pub struct Thrust(f32);
pub struct MaxSpeed(f32);

/// Direction the entity is facing. Readonly.
pub struct Heading {
     pub radians: f32,
}

fn update_velocity(mut query: Query<(&Speed, &Transform, &mut Velocity)>) {
    for (speed, transform, mut velocity) in query.iter_mut() {
        velocity.0 = speed.0 * transform.local_x();
    }
}

fn update_translation(mut query: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut trans) in query.iter_mut() {
        trans.translation = trans.translation + vel.0 * FIXED_TIME_STEP;
    }
}

fn update_rotation(mut query: Query<(&TurnSpeed, &mut Transform)>) {
    for (rps, mut trans) in query.iter_mut() {
        trans.rotation = trans.rotation * (Quat::from_rotation_z(rps.radians_per_second * FIXED_TIME_STEP));
    }
}

fn calculate_max_speed(mut query: Query<(&Mass, &Thrust, &mut MaxSpeed)>) {
    for (mass, thrust, mut max_speed) in query.iter_mut() {
        max_speed.0 = thrust.0 / mass.0;
    }
}

fn calculate_speed(mut query: Query<(&MaxSpeed, &mut Speed)>) {
    for (max_speed, mut speed) in query.iter_mut() {
        speed.0 = max_speed.0;
    }
}

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum MovementSystems {
    CalculateSpeed,
    CalculateMaxSpeed,
    UpdateRotation,
    UpdateVelocity,
}

#[derive(Default)]
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // app.register_type::<Velocity>()
        // .register_type::<Speed>()
        // .register_type::<TurnSpeed>()
        // .register_type::<Mass>()
        // .register_type::<Thrust>()
        // .register_type::<MaxSpeed>();

        app.add_startup_system_to_stage(
            CoreStage::Update,
            calculate_max_speed
                .system()
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .label(MovementSystems::CalculateMaxSpeed),
        )
        .add_startup_system_to_stage(
            CoreStage::Update,
            calculate_speed
                .system()
                .label(MovementSystems::CalculateSpeed)
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .after(MovementSystems::CalculateMaxSpeed),
        )
        .add_startup_system_to_stage(
            CoreStage::Update,
            update_rotation
                .system()
                .label(MovementSystems::UpdateRotation)
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .after(MovementSystems::CalculateSpeed),
        )
        .add_startup_system_to_stage(
            CoreStage::Update,
            update_velocity
                .system()
                .label(MovementSystems::UpdateVelocity)
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .after(MovementSystems::UpdateRotation),
        )
        .add_startup_system_to_stage(
            CoreStage::Update,
            update_translation
                .system()
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .after(MovementSystems::UpdateVelocity),
        );
    }
}
