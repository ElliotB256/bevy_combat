use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::Quat,
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};

use bevy_combat::combat::{
    mortal::{Health, MaxHealth},
    Target, Team,
};
use bevy_combat::{ai::aggression::*, combat::shields::Shield};
use bevy_combat::{
    ai::{idle::IdleBehavior, movement::TurnToDestinationBehavior, AIPlugin},
    combat::{damage::LastDamageTimer, evasion::Evasion},
    materials::ShipMaterial,
};
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
    app.add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
        level: bevy::log::Level::INFO,
        ..default()
    }))
    .add_plugins((
        LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin,
    ));

    app.add_plugins((
        BaseGamePlugin,
        AIPlugin,
        MovementPlugin,
        bevy_combat::combat::CombatPlugin,
        bevy_combat::fx::animated::AnimatedEffectsPlugin,
        bevy_combat::fx::EffectsPlugin,
        bevy_combat::fx::beams::BeamEffectPlugin,
        Material2dPlugin::<ShipMaterial>::default(),
    ));

    app.add_systems(Startup, setup);
    app.add_systems(Update, tick);
    app.run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ShipMaterial>>,
    assets: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();

    let tile_size = Vec2::splat(16.0);
    let smallship_base = assets.load("art/smallship.png");
    let smallship_mask = assets.load("art/smallship_mask.png");
    let drone_base = assets.load("art/drone.png");
    let drone_mask = assets.load("art/drone_mask.png");

    commands
        .spawn(Camera2dBundle::default())
        .insert(PrintTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
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
            .spawn({
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad {
                            size: Vec2::new(32.0, 32.0),
                            flip: false,
                        }))
                        .into(),
                    material: materials.add(ShipMaterial {
                        color: Color::rgba(0.0, 0.0, 1.0, 1.0),
                        last_damaged_time: 1.0,
                        base_texture: smallship_base.clone(),
                        color_mask: smallship_mask.clone(),
                    }),
                    transform: Transform {
                        translation,
                        rotation,
                        scale,
                    },
                    ..default()
                }
            })
            .insert(MovementBundle {
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
            .insert((
                AggroRadius(1000.0),
                AggroLocation::default(),
                TargetingOrders {
                    preferred: AgentCategory::FIGHTER,
                    discouraged: AgentCategory::CRUISER,
                    target_same_team: false,
                },
                Target::default(),
                Team(1),
                Health(100.0),
                LastDamageTimer(0.0),
                MaxHealth(100.0),
                AgentCategory::FIGHTER,
                Mortal,
                RetargetBehavior {
                    interval: 4.0,
                    remaining_time: 4.0,
                },
            ))
            .insert((
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
            .spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad {
                        size: Vec2::new(16.0, 16.0),
                        flip: false,
                    }))
                    .into(),
                material: materials.add(ShipMaterial {
                    color: Color::rgba(1.0, 0.0, 0.0, 1.0),
                    last_damaged_time: 1.0,
                    base_texture: drone_base.clone(),
                    color_mask: drone_mask.clone(),
                }),
                transform: Transform {
                    translation,
                    rotation,
                    scale,
                },
                ..default()
            })
            .insert(MovementBundle {
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
            .insert((
                AggroRadius(1000.0),
                AggroLocation::default(),
                TargetingOrders {
                    preferred: AgentCategory::FIGHTER,
                    discouraged: AgentCategory::CRUISER,
                    target_same_team: false,
                },
                Target::default(),
                Team(2),
                Health(50.0),
                LastDamageTimer(0.0),
                MaxHealth(50.0),
                AgentCategory::FIGHTER,
                Mortal,
                RetargetBehavior {
                    interval: 4.0,
                    remaining_time: 4.0,
                },
            ))
            .insert((
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
