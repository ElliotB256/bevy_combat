use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::Quat,
    prelude::*,
};

use bevy_combat::{ai::{idle::IdleBehavior, movement::TurnToDestinationBehavior, AIPlugin}, combat::evasion::Evasion};
use bevy_combat::combat::{
    mortal::{Health, MaxHealth},
    Target, Team,
};
use bevy_combat::{ai::aggression::*, combat::shields::Shield};
use bevy_combat::{
    combat::mortal::Mortal,
    fx::{animated::AnimatedEffects, death::DeathEffect},
    game::BaseGamePlugin,
    movement::*,
};
use rand::Rng;

#[derive(Component)]
pub struct PrintTimer(Timer);
#[derive(Component)]
pub struct Position(Transform);

fn main() {
    let mut app = App::new();
        app.add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(BaseGamePlugin)
        .add_plugin(AIPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(bevy_combat::combat::CombatPlugin)
        .add_plugin(bevy_combat::fx::animated::AnimatedEffectsPlugin)
        .add_plugin(bevy_combat::fx::EffectsPlugin)
        .add_plugin(bevy_combat::fx::beams::BeamEffectPlugin)
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(tick.label("Tick"));
        app.run()
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();

    let tile_size = Vec2::splat(16.0);
    let sprite_handle = assets.load("art/smallship.png");
    let drones = assets.load("art/drone.png");

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(PrintTimer(Timer::from_seconds(1.0, true)))
        .insert(Position(Transform::from_translation(Vec3::new(
            0.0, 0.0, 1000.0,
        ))));

    commands.insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.8)));

    // Team 1
    for _i in 0..20 {
        let position =
            Vec2::new(-20.0, 0.0) + Vec2::new(rng.gen_range(-5.0..5.0), rng.gen_range(-20.0..20.0));
        let translation = (position * tile_size).extend(0.0);
        let rotation = Quat::from_rotation_z(rng.gen::<f32>());
        let scale = Vec3::splat(0.5);

        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                texture: sprite_handle.clone(),
                transform: Transform {
                    translation,
                    rotation,
                    scale,
                },
                ..Default::default()
            })
            .insert_bundle(MovementBundle {
                velocity: Velocity::default(),
                speed: Speed::default(),
                max_speed: MaxSpeed::default(),
                turn_speed: TurnSpeed::default(),
                max_turn_speed: MaxTurnSpeed::new(3.0),
                mass: Mass(1.0),
                thrust: Thrust(150.0),
                heading: Heading::default(),
            })
            .insert(IdleBehavior)
            .insert(TurnToDestinationBehavior {
                destination: Vec3::default(),
            })
            .insert(bevy_combat::ai::idle::RoamBehavior {
                centre: Vec3::default(),
                radius: 10.0,
            })
            .insert_bundle((
                AggroRadius { 0: 1000.0 },
                AggroLocation::default(),
                TargetingOrders {
                    preferred: AgentCategory::FIGHTER,
                    discouraged: AgentCategory::CRUISER,
                    target_same_team: false,
                },
                Target::default(),
                Team { 0: 1 },
                Health { 0: 100.0 },
                MaxHealth { 0: 100.0 },
                AgentCategory::FIGHTER,
                Mortal,
                RetargetBehavior {
                    interval: 4.0,
                    remaining_time: 4.0,
                },
            ))
            .insert_bundle((
                bevy_combat::combat::tools::Cooldown::new(1.0),
                bevy_combat::combat::tools::TargettedTool {
                    range: 100.0,
                    cone: 0.15,
                    armed: true,
                    firing: false,
                },
                bevy_combat::combat::effects::Effector {
                    spawn_effect: bevy_combat::templates::weapons::pulse_laser_attack,
                },
            ))
            .insert(DeathEffect {
                time_to_explosion: 0.1,
                time_to_smoke: 0.05,
                dying_explosion: AnimatedEffects::SmallExplosion,
                death_explosion: AnimatedEffects::MediumExplosion,
            })
            .insert(Shield {
                health: 100.0,
                radius: 32.0,
            })
            .insert(Evasion::new(0.0));
    }

    // Team 2
    for _i in 0..60 {
        let drone_size = Vec2::splat(8.0);
        let position =
            Vec2::new(60.0, 0.0) + Vec2::new(rng.gen_range(-5.0..5.0), rng.gen_range(-20.0..20.0));
        let translation = (position * drone_size).extend(0.0);
        let rotation = Quat::from_rotation_z(rng.gen::<f32>());
        let scale = Vec3::splat(0.5);

        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                texture: drones.clone(),
                transform: Transform {
                    translation,
                    rotation,
                    scale,
                },
                ..Default::default()
            })
            .insert_bundle(MovementBundle {
                velocity: Velocity::default(),
                speed: Speed::default(),
                max_speed: MaxSpeed::default(),
                turn_speed: TurnSpeed::default(),
                max_turn_speed: MaxTurnSpeed::new(4.0),
                mass: Mass(1.0),
                thrust: Thrust(250.0),
                heading: Heading::default(),
            })
            .insert(IdleBehavior)
            .insert(TurnToDestinationBehavior {
                destination: Vec3::default(),
            })
            .insert(bevy_combat::ai::idle::RoamBehavior {
                centre: Vec3::default(),
                radius: 10.0,
            })
            .insert_bundle((
                AggroRadius { 0: 1000.0 },
                AggroLocation::default(),
                TargetingOrders {
                    preferred: AgentCategory::FIGHTER,
                    discouraged: AgentCategory::CRUISER,
                    target_same_team: false,
                },
                Target::default(),
                Team { 0: 2 },
                Health { 0: 50.0 },
                MaxHealth { 0: 50.0 },
                AgentCategory::FIGHTER,
                Mortal,
                RetargetBehavior {
                    interval: 4.0,
                    remaining_time: 4.0,
                },
            ))
            .insert_bundle((
                bevy_combat::combat::tools::Cooldown::new(0.2),
                bevy_combat::combat::tools::TargettedTool {
                    range: 80.0,
                    cone: 0.3,
                    armed: true,
                    firing: false,
                },
                bevy_combat::combat::effects::Effector {
                    spawn_effect: bevy_combat::templates::weapons::small_pulse_laser_attack,
                },
            ))
            .insert(DeathEffect {
                time_to_explosion: 0.1,
                time_to_smoke: 0.05,
                dying_explosion: AnimatedEffects::SmallExplosion,
                death_explosion: AnimatedEffects::MediumExplosion,
            })
            .insert(Evasion::new(0.0));
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
