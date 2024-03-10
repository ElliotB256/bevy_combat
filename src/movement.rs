//! Movement and rotation of entities.
use crate::{game::GameTimeDelta, math_util};
use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Velocity(pub Vec3);
#[derive(Default, Component)]
pub struct MaxSpeed(pub f32);
#[derive(Default, Component)]
pub struct Speed(pub f32);
#[derive(Default, Component)]
pub struct MaxTurnSpeed {
    pub radians_per_second: f32,
}
#[derive(Default, Component)]
pub struct TurnSpeed {
    pub radians_per_second: f32,
}
#[derive(Component, Default)]
pub struct Mass(pub f32);
#[derive(Component, Default)]
pub struct Thrust(pub f32);

#[derive(Default, Component)]
/// Direction the entity is facing. Readonly.
pub struct Heading {
    pub radians: f32,
}

impl MaxTurnSpeed {
    pub fn new(rps: f32) -> MaxTurnSpeed {
        MaxTurnSpeed {
            radians_per_second: rps,
        }
    }
}

#[derive(Bundle, Default)]
pub struct MovementBundle {
    pub velocity: Velocity,
    pub speed: Speed,
    pub max_speed: MaxSpeed,
    pub turn_speed: TurnSpeed,
    pub max_turn_speed: MaxTurnSpeed,
    pub mass: Mass,
    pub thrust: Thrust,
    pub heading: Heading,
}

fn update_velocity(mut query: Query<(&Speed, &Transform, &mut Velocity)>) {
    for (speed, transform, mut velocity) in query.iter_mut() {
        velocity.0 = speed.0 * *transform.local_y();
    }
}

fn update_translation(dt: Res<GameTimeDelta>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut trans) in query.iter_mut() {
        trans.translation += vel.0 * dt.0;
    }
}

pub fn set_heading_from_transform_on_heading_added(
    mut query: Query<(&mut Heading, &Transform, Option<&GlobalTransform>), Added<Heading>>,
) {
    for (mut heading, transform, global_transform_opt) in query.iter_mut() {
        let used_transform = 
        if let Some(global_transform) = global_transform_opt {
            global_transform.compute_transform()
        } else {
            *transform
        };
        // A heading of zero corresponds to facing along the local y direction.
        heading.radians = math_util::get_heading_to_point(*used_transform.local_y());
    }
}

pub fn update_rotation(mut query: Query<(&Heading, &mut Transform)>) {
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

pub fn update_heading(dt: Res<GameTimeDelta>, mut query: Query<(&TurnSpeed, &mut Heading)>) {
    for (turn_speed, mut heading) in query.iter_mut() {
        heading.radians += turn_speed.radians_per_second * dt.0;
        let two_pi = 2.0 * std::f32::consts::PI;
        while heading.radians > two_pi {
            heading.radians -= two_pi;
        }
        while heading.radians < 0.0 {
            heading.radians += two_pi;
        }
    }
}

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemSet)]
pub struct MovementSystems;

#[derive(Default)]
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                set_heading_from_transform_on_heading_added,
                update_heading.after(set_heading_from_transform_on_heading_added),
                calculate_max_speed,
                calculate_speed.after(calculate_max_speed),
                update_rotation.after(update_heading),
                update_velocity.after(update_rotation),
                update_translation.after(update_velocity),
            )
                .in_set(MovementSystems),
        );
    }
}
