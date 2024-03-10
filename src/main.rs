use std::time::Duration;

use bevy::{
    asset::AssetMetaCheck,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::Material2dPlugin,
};

use bevy_combat::{ai::AIPlugin, game::GameTimeDelta, materials::ShipMaterial, templates::ships::frigates::RocketFrigateSpawner};
use bevy_combat::{
    combat::Team,
    templates::ships::{
        fighters::{DroneSpawner, SmallShipSpawner},
        spawn::SpawnBundle,
    },
};
use bevy_combat::{game::BaseGamePlugin, movement::*};
use rand::Rng;

#[derive(Component)]
pub struct PrintTimer(Timer);

fn main() {
    let mut app = App::new();
    app.insert_resource(AssetMetaCheck::Never);
    app.add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
        level: bevy::log::Level::INFO,
        ..default()
    }))
    .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin));

    app.add_plugins((
        BaseGamePlugin,
        AIPlugin,
        MovementPlugin,
        bevy_combat::combat::CombatPlugin,
        bevy_combat::fx::animated::AnimatedEffectsPlugin,
        bevy_combat::fx::EffectsPlugin,
        bevy_combat::fx::beams::BeamEffectPlugin,
        Material2dPlugin::<ShipMaterial>::default(),
        bevy_combat::templates::ships::fighters::FighterTemplatePlugin,
        bevy_combat::templates::ships::frigates::FrigateTemplatePlugin,
        bevy_combat::templates::ships::rockets::RocketTemplatePlugin,
    ));

    app.add_systems(Startup, setup);
    app.add_systems(Update, tick);
    app.add_systems(FixedUpdate, spawn_reinforcements);
    app.insert_resource(WaveTimer(Timer::from_seconds(15.0, TimerMode::Repeating)));
    app.run()
}

fn setup(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let tile_size = Vec2::splat(16.0);

    commands
        .spawn(Camera2dBundle::default())
        .insert(PrintTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));

    commands.insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.8)));

    // Team 1
    for _i in 0..20 {
        let position =
            Vec2::new(-20.0, 0.0) + Vec2::new(rng.gen_range(-5.0..5.0), rng.gen_range(-20.0..20.0));
        let translation = (position * tile_size).extend(0.0);
        let rotation = Quat::from_rotation_z(rng.gen::<f32>());
        let scale = Vec3::splat(0.5);

        commands.spawn(SpawnBundle {
            spawn: SmallShipSpawner,
            transform: Transform {
                translation,
                rotation,
                scale,
            },
            team: Team(1),
        });
    }

    // Team 2
    for _i in 0..60 {
        let drone_size = Vec2::splat(8.0);
        let position =
            Vec2::new(60.0, 0.0) + Vec2::new(rng.gen_range(-5.0..5.0), rng.gen_range(-20.0..20.0));
        let translation = (position * drone_size).extend(0.0);
        let rotation = Quat::from_rotation_z(rng.gen::<f32>());
        let scale = Vec3::splat(0.5);

        commands.spawn(SpawnBundle {
            spawn: DroneSpawner,
            transform: Transform {
                translation,
                rotation,
                scale,
            },
            team: Team(2),
        });
    }
}

#[derive(Resource)]
struct WaveTimer(Timer);

fn spawn_reinforcements(
    mut wave_timer: ResMut<WaveTimer>,
    team_members: Query<&Team>,
    dt: Res<GameTimeDelta>,
    mut commands: Commands,
) {
    wave_timer.0.tick(Duration::from_secs_f32(dt.0));

    if wave_timer.0.finished() {
        let mut team_1_count = 0;
        let mut team_2_count = 0;

        for team_member in team_members.iter() {
            match team_member.0 {
                1 => team_1_count += 1,
                2 => team_2_count += 2,
                _ => {}
            }
        }

        let reinforced_team = if team_1_count > team_2_count {
            Team(2)
        } else {
            Team(1)
        };

        let mut rng = rand::thread_rng();
        let number = rng.gen_range(10..=14);
        let drones = rng.gen_bool(0.5);
        for _i in 0..number {
            let pos: Vec2 = get_random_spawn_position_for_team(&reinforced_team);
            let translation = (pos).extend(0.0);
            let rotation = Quat::from_rotation_z(rng.gen::<f32>());
            let scale = Vec3::splat(0.5);

            if drones {
                commands.spawn(SpawnBundle {
                    spawn: DroneSpawner,
                    transform: Transform {
                        translation,
                        rotation,
                        scale,
                    },
                    team: reinforced_team,
                });
            } else {
                commands.spawn(SpawnBundle {
                    spawn: SmallShipSpawner,
                    transform: Transform {
                        translation,
                        rotation,
                        scale,
                    },
                    team: reinforced_team,
                });
            }
        }
        for _i in 0..rng.gen_range(0..=2) {
            let pos: Vec2 = get_random_spawn_position_for_team(&reinforced_team);
            let translation = (pos).extend(0.0);
            let rotation = Quat::from_rotation_z(rng.gen::<f32>());
            let scale = Vec3::splat(0.5);

            commands.spawn(SpawnBundle {
                spawn: RocketFrigateSpawner,
                transform: Transform {
                    translation,
                    rotation,
                    scale,
                },
                team: reinforced_team,
            });
        }
    }
}

fn get_random_spawn_position_for_team(team: &Team) -> Vec2 {
    let mut rng = rand::thread_rng();
    if team.0 == 1 {
        8.0 * (Vec2::new(-80.0, 0.0)
            + Vec2::new(rng.gen_range(-5.0..5.0), rng.gen_range(-20.0..20.0)))
    } else {
        8.0 * (Vec2::new(80.0, 0.0)
            + Vec2::new(rng.gen_range(-5.0..5.0), rng.gen_range(-20.0..20.0)))
    }
}

fn tick(time: Res<Time>, sprites: Query<&Sprite>, mut query: Query<&mut PrintTimer>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            println!("Sprites: {}", sprites.iter().count(),);
        }
    }
}
