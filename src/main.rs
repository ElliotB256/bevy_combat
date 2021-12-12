use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::Quat,
    prelude::*,
};

use rand::Rng;
use bevy_combat::movement::*;
use bevy_combat::ai::{AIPlugin, movement::TurnToDestinationBehavior, idle::IdleBehavior};
use bevy_combat::ai::aggression::*;
use bevy_combat::combat::{Health, MaxHealth, Target, Team};

pub struct PrintTimer(Timer);
pub struct Position(Transform);

fn main() {
    App::build()
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(AIPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(bevy_combat::combat::CombatPlugin)
        .add_plugin(bevy_combat::fx::animated::AnimatedEffectsPlugin)
        .add_plugin(bevy_combat::fx::EffectsPlugin)
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(tick.system().label("Tick"))
        .run()
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let tile_size = Vec2::splat(16.0);
    let map_size = Vec2::splat(10.0);

    let half_x = (map_size.x / 2.0) as i32;
    let half_y = (map_size.y / 2.0) as i32;

    let sprite_handle = materials.add(assets.load("art/smallship.png").into());

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(PrintTimer(Timer::from_seconds(1.0, true)))
        .insert(Position(Transform::from_translation(Vec3::new(
            0.0, 0.0, 1000.0,
        ))));

    for y in -half_y..=half_y {
        for x in -half_x..=half_x {
            let position = Vec2::new(x as f32, y as f32);
            let translation = (position * tile_size).extend(0.0);
            let rotation = Quat::from_rotation_z(rng.gen::<f32>());
            let scale = Vec3::splat(1.0);

            commands.spawn().insert_bundle(SpriteBundle {
                material: sprite_handle.clone(),
                transform: Transform {
                    translation,
                    rotation,
                    scale,
                },
                sprite: Sprite::new(tile_size),
                ..Default::default()
            }
            ).insert_bundle(
                MovementBundle {
                    velocity: Velocity::default(),
                    speed: Speed::default(),
                    max_speed: MaxSpeed::default(),
                    turn_speed: TurnSpeed::default(),
                    max_turn_speed: MaxTurnSpeed::new(3.0),
                    mass: Mass(1.0),
                    thrust: Thrust(150.0),
                    heading: Heading::default()
                }
            )
            .insert(IdleBehavior)
            .insert(TurnToDestinationBehavior { destination: Vec3::default() })
            .insert(bevy_combat::ai::idle::RoamBehavior { centre: Vec3::default(), radius: 10.0 })
            .insert_bundle(
                (
                    AggroRadius { 0: 1000.0 },
                    AggroLocation::default(),
                    TargetingOrders { 
                        preferred: AgentCategory::FIGHTER,
                        discouraged: AgentCategory::CRUISER,
                        target_same_team: false
                    },
                    Target::default(),
                    Team { 0: 2 },
                    Health { 0: 100.0 },
                    MaxHealth { 0: 100.0 },
                    AgentCategory::FIGHTER
                )
            )
            .insert_bundle(
                (
                    bevy_combat::combat::tools::Cooldown::new(1.0),
                    bevy_combat::combat::tools::TargettedTool {
                        range: 100.0,
                        cone: 0.4,
                        armed: true,
                        firing: false
                    },
                    bevy_combat::combat::effects::Effector { 
                        spawn_effect: bevy_combat::templates::weapons::small_pulse_laser_attack
                    }
                )
            );

        }
    }

    let rotation = Quat::from_rotation_z(rng.gen::<f32>());
    let scale = Vec3::splat(3.0);
    commands.spawn().insert_bundle(SpriteBundle {
        material: sprite_handle.clone(),
        transform: Transform {
            translation: Vec3::new(200.0,200.0,0.0),
            rotation,
            scale,
        },
        sprite: Sprite::new(tile_size),
        ..Default::default()
    }
    ).insert_bundle(
        MovementBundle {
            velocity: Velocity::default(),
            speed: Speed::default(),
            max_speed: MaxSpeed::default(),
            turn_speed: TurnSpeed::default(),
            max_turn_speed: MaxTurnSpeed::new(3.0),
            mass: Mass(1.0),
            thrust: Thrust(50.0),
            heading: Heading::default()
        }
    )
    .insert(IdleBehavior)
    .insert(TurnToDestinationBehavior { destination: Vec3::default() })
    .insert(bevy_combat::ai::idle::RoamBehavior { centre: Vec3::default(), radius: 300.0 })
    .insert_bundle(
        (
            AggroRadius { 0: 300.0 },
            AggroLocation::default(),
            TargetingOrders { 
                preferred: AgentCategory::FIGHTER,
                discouraged: AgentCategory::CRUISER,
                target_same_team: false
            },
            Target::default(),
            Team { 0: 1 },
            Health { 0: 100.0 },
            MaxHealth { 0: 100.0 },
            AgentCategory::FIGHTER
        )
    );

    commands.insert_resource(bevy_combat::game::GameTimeDelta { 0: 1.0/60.0 });
}

fn tick(time: Res<Time>, sprites: Query<&Sprite>, mut query: Query<&mut PrintTimer>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            println!("Sprites: {}", sprites.iter().count(),);
        }
    }
}