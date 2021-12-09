use bevy::{core::FixedTimestep, prelude::*};
use crate::constants::FIXED_TIME_STEP;
use crate::math_util::get_heading_to_point;

#[derive(Default)]
pub struct Velocity(pub Vec3);
#[derive(Default)]
pub struct MaxSpeed(pub f32);
#[derive(Default)]
pub struct Speed(pub f32);
#[derive(Default)]
pub struct MaxTurnSpeed {
    pub radians_per_second: f32,
}
#[derive(Default)]
pub struct TurnSpeed {
    pub radians_per_second: f32,
}
pub struct Mass(pub f32);
pub struct Thrust(pub f32);

#[derive(Default)]
/// Direction the entity is facing. Readonly.
pub struct Heading {
     pub radians: f32,
}

impl MaxTurnSpeed {
    pub fn new(rps: f32) -> MaxTurnSpeed {
        MaxTurnSpeed { radians_per_second: rps }
    }
}

#[derive(Bundle)]
pub struct MovementBundle {
    pub velocity: Velocity,
    pub speed: Speed,
    pub max_speed: MaxSpeed,
    pub turn_speed: TurnSpeed,
    pub max_turn_speed: MaxTurnSpeed,
    pub mass: Mass,
    pub thrust: Thrust,
    pub heading: Heading
}

fn update_velocity(mut query: Query<(&Speed, &Transform, &mut Velocity)>) {
    for (speed, transform, mut velocity) in query.iter_mut() {
        velocity.0 = speed.0 * transform.local_y();
    }
}

fn update_translation(mut query: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut trans) in query.iter_mut() {
        trans.translation = trans.translation + vel.0 * FIXED_TIME_STEP;
    }
}

fn update_rotation(mut query: Query<(&Heading, &mut Transform)>) {
    for (heading, mut trans) in query.iter_mut() {
        trans.rotation = Quat::from_rotation_z(heading.radians - std::f32::consts::FRAC_PI_2);
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

fn update_heading(mut query: Query<(&TurnSpeed, &mut Heading)>) {
    for (turn_speed, mut heading) in query.iter_mut() {
        heading.radians += turn_speed.radians_per_second * FIXED_TIME_STEP;
        let two_pi = 2.0 * std::f32::consts::PI;
        while heading.radians > two_pi {
            heading.radians -= two_pi;
        }
        while heading.radians < 0.0 {
            heading.radians += two_pi;
        }
    }
}

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum MovementSystems {
    CalculateSpeed,
    CalculateMaxSpeed,
    UpdateRotation,
    UpdateVelocity,
    UpdateHeading
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

        app.add_system_to_stage(
            CoreStage::Update,
            update_heading
                .system()
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .label(MovementSystems::UpdateHeading));
        app.add_system_to_stage(
            CoreStage::Update,
            calculate_max_speed
                .system()
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
        )
        .add_system_to_stage(
            CoreStage::Update,
            calculate_speed
                .system()
                .label(MovementSystems::CalculateSpeed)
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .after(MovementSystems::CalculateMaxSpeed),
        )
        .add_system_to_stage(
            CoreStage::Update,
            update_rotation
                .system()
                .label(MovementSystems::UpdateRotation)
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .after(MovementSystems::UpdateHeading),
        )
        .add_system_to_stage(
            CoreStage::Update,
            update_velocity
                .system()
                .label(MovementSystems::UpdateVelocity)
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .after(MovementSystems::UpdateRotation),
        )
        .add_system_to_stage(
            CoreStage::Update,
            update_translation
                .system()
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .after(MovementSystems::UpdateVelocity),
        );
    }
}
