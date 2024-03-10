//! Fighter templates

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    ai::{
        aggression::{
            AgentCategory, AggroLocation, AggroRadius, RetargetBehavior, TargetingOrders,
        },
        idle::IdleBehavior,
        movement::TurnToDestinationBehavior,
    },
    combat::{
        damage::LastDamageTimer, evasion::Evasion, mortal::{Health, MaxHealth, Mortal}, projectile::CircularHitBox, shields::Shield, targets::InheritTargetFromParent, Target, Team
    },
    fx::{animated::AnimatedEffects, death::DeathEffect},
    materials::ShipMaterial,
    movement::{Mass, MaxTurnSpeed, MovementBundle, Thrust},
};

use super::spawn::{spawn_ships_and_despawn_spawn_commands, SpawnShipTemplate};

/// Resources used to spawn fighters.
#[derive(Resource)]
pub struct FighterResources {
    small_ship_color: Handle<Image>,
    small_ship_mask: Handle<Image>,
    small_ship_mesh: Mesh2dHandle,

    drone_color: Handle<Image>,
    drone_mask: Handle<Image>,
    drone_mesh: Mesh2dHandle,
}

#[derive(Component)]
pub struct DroneSpawner;
impl SpawnShipTemplate for DroneSpawner {
    type Resources<'a> = FighterResources;

    fn spawn<'a>(
        &self,
        commands: &mut Commands,
        resources: &Res<FighterResources>,
        materials: &mut ResMut<Assets<ShipMaterial>>,
    ) -> Entity {
        commands
            .spawn({
                MaterialMesh2dBundle {
                    mesh: resources.drone_mesh.clone(),
                    material: materials.add(ShipMaterial {
                        color: Color::rgba(1.0, 0.0, 0.0, 1.0),
                        last_damaged_time: 1.0,
                        base_texture: resources.drone_color.clone(),
                        color_mask: resources.drone_mask.clone(),
                    }),
                    ..default()
                }
            })
            .insert(MovementBundle {
                max_turn_speed: MaxTurnSpeed::new(4.0),
                mass: Mass(1.0),
                thrust: Thrust(250.0),
                ..Default::default()
            })
            .insert(IdleBehavior)
            .insert(TurnToDestinationBehavior::default())
            .insert(crate::ai::idle::RoamBehavior {
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
                crate::combat::tools::Cooldown::new(0.12),
                crate::combat::tools::TargettedTool {
                    range: 80.0,
                    cone: 0.3,
                    armed: true,
                    firing: false,
                },
                crate::combat::effects::Effector::new(
                    crate::templates::weapons::small_pulse_laser_attack,
                ),
            ))
            .insert(DeathEffect {
                time_to_explosion: 0.1,
                time_to_smoke: 0.05,
                dying_explosion: AnimatedEffects::SmallExplosion,
                death_explosion: AnimatedEffects::MediumExplosion,
            })
            .insert(CircularHitBox { radius: 8.0 })
            .insert(Evasion::new(0.0))
            .id()
    }
}

#[derive(Component)]
pub struct SmallShipSpawner;
impl SpawnShipTemplate for SmallShipSpawner {
    type Resources<'a> = FighterResources;

    fn spawn<'a>(
        &self,
        commands: &mut Commands,
        resources: &Res<FighterResources>,
        materials: &mut ResMut<Assets<ShipMaterial>>,
    ) -> Entity {
        let laser_gun_left = commands
            .spawn((
                Transform {
                    translation: Vec3::new(-8.0, 0.0, 0.0),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::splat(1.0),
                },
                GlobalTransform::default(),
            ))
            .insert((Target::default(), InheritTargetFromParent))
            .insert((
                crate::combat::tools::Cooldown::new(1.0),
                crate::combat::tools::TargettedTool {
                    range: 100.0,
                    cone: 0.15,
                    armed: true,
                    firing: false,
                },
                crate::combat::effects::Effector::new(
                    crate::templates::weapons::pulse_laser_attack,
                ),
            ))
            .id();
        let laser_gun_right = commands
            .spawn((
                Transform {
                    translation: Vec3::new(8.0, 0.0, 0.0),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::splat(1.0),
                },
                GlobalTransform::default(),
            ))
            .insert((Target::default(), InheritTargetFromParent))
            .insert((
                crate::combat::tools::Cooldown::new(1.0),
                crate::combat::tools::TargettedTool {
                    range: 100.0,
                    cone: 0.15,
                    armed: true,
                    firing: false,
                },
                crate::combat::effects::Effector::new(
                    crate::templates::weapons::pulse_laser_attack,
                ),
            ))
            .id();

        commands
            .spawn({
                MaterialMesh2dBundle {
                    mesh: resources.small_ship_mesh.clone(),
                    material: materials.add(ShipMaterial {
                        color: Color::rgba(0.0, 0.0, 1.0, 1.0),
                        last_damaged_time: 1.0,
                        base_texture: resources.small_ship_color.clone(),
                        color_mask: resources.small_ship_mask.clone(),
                    }),
                    ..default()
                }
            })
            .insert(MovementBundle {
                max_turn_speed: MaxTurnSpeed::new(3.0),
                mass: Mass(1.0),
                thrust: Thrust(150.0),
                ..default()
            })
            .insert(IdleBehavior)
            .insert(TurnToDestinationBehavior::default())
            .insert(crate::ai::idle::RoamBehavior {
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
            .insert(DeathEffect {
                time_to_explosion: 0.1,
                time_to_smoke: 0.05,
                dying_explosion: AnimatedEffects::SmallExplosion,
                death_explosion: AnimatedEffects::MediumExplosion,
            })
            .insert(Shield {
                health: 100.0,
                radius: 22.0,
            })
            .insert(CircularHitBox { radius: 15.0 })
            .insert(Evasion::new(0.0))
            .push_children(&[laser_gun_left, laser_gun_right])
            .id()
    }
}

pub struct FighterTemplatePlugin;
impl FighterTemplatePlugin {
    fn setup(mut commands: Commands, assets: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>) {
        let resources = FighterResources {
            small_ship_color: assets.load("art/smallship.png"),
            small_ship_mask: assets.load("art/smallship_mask.png"),
            small_ship_mesh: meshes
                .add(Mesh::from(Rectangle {
                    half_size: Vec2::new(16.0, 16.0),
                }))
                .into(),
            drone_color: assets.load("art/drone.png"),
            drone_mask: assets.load("art/drone_mask.png"),
            drone_mesh: meshes
                .add(Mesh::from(Rectangle {
                    half_size: Vec2::new(8.0, 8.0),
                }))
                .into(),
        };
        commands.insert_resource(resources);
    }
}
impl Plugin for FighterTemplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, FighterTemplatePlugin::setup);
        app.add_systems(
            FixedUpdate,
            (
                spawn_ships_and_despawn_spawn_commands::<DroneSpawner>,
                spawn_ships_and_despawn_spawn_commands::<SmallShipSpawner>,
            ),
        );
    }
}
