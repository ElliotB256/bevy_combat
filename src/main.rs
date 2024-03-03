use bevy::{
    asset::AssetMetaCheck,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::Material2dPlugin,
};

use bevy_combat::{ai::AIPlugin, materials::ShipMaterial};
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
    ));

    app.add_systems(Startup, setup);
    app.add_systems(Update, tick);
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

fn tick(time: Res<Time>, sprites: Query<&Sprite>, mut query: Query<&mut PrintTimer>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            println!("Sprites: {}", sprites.iter().count(),);
        }
    }
}
