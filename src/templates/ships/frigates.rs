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
pub struct FrigateResources {
    medium_ship_1_color: Handle<Image>,
    medium_ship_1_mask: Handle<Image>,
    medium_ship_1_mesh: Mesh2dHandle,
}

#[derive(Component)]
pub struct RocketFrigateSpawner;
impl SpawnShipTemplate for RocketFrigateSpawner {
    type Resources<'a> = FrigateResources;

    fn spawn<'a>(
        &self,
        commands: &mut Commands,
        resources: &Res<FrigateResources>,
        materials: &mut ResMut<Assets<ShipMaterial>>,
    ) -> Entity {
        let launcher_left = commands
            .spawn((
                Transform {
                    translation: Vec3::new(-8.0, 0.0, 0.0),
                    rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                    scale: Vec3::splat(1.0),
                },
                GlobalTransform::default(),
            ))
            .insert((Target::default(), InheritTargetFromParent))
            .insert((
                crate::combat::tools::Cooldown::new(0.6),
                crate::combat::tools::TargettedTool {
                    range: 500.0,
                    cone: 2.0*std::f32::consts::PI,
                    armed: true,
                    firing: false,
                },
                crate::combat::effects::Effector::new(
                    super::rockets::small_rocket_launcher,
                ),
            ))
            .id();
        let launcher_right = commands
            .spawn((
                Transform {
                    translation: Vec3::new(8.0, 0.0, 0.0),
                    rotation: Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
                    scale: Vec3::splat(1.0),
                },
                GlobalTransform::default(),
            ))
            .insert((Target::default(), InheritTargetFromParent))
            .insert((
                crate::combat::tools::Cooldown::new(0.6),
                crate::combat::tools::TargettedTool {
                    range: 500.0,
                    cone: 2.0*std::f32::consts::PI,
                    armed: true,
                    firing: false,
                },
                crate::combat::effects::Effector::new(
                    super::rockets::small_rocket_launcher,
                    //crate::templates::weapons::pulse_laser_attack
                ),
            ))
            .id();

        commands
            .spawn({
                MaterialMesh2dBundle {
                    mesh: resources.medium_ship_1_mesh.clone(),
                    material: materials.add(ShipMaterial {
                        color: Color::rgba(0.0, 0.0, 1.0, 1.0),
                        last_damaged_time: 1.0,
                        base_texture: resources.medium_ship_1_color.clone(),
                        color_mask: resources.medium_ship_1_mask.clone(),
                    }),
                    ..default()
                }
            })
            .insert(MovementBundle {
                max_turn_speed: MaxTurnSpeed::new(3.0),
                mass: Mass(2.0),
                thrust: Thrust(200.0),
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
                health: 200.0,
                radius: 32.0,
            })
            .insert(CircularHitBox { radius: 28.0 })
            .insert(Evasion::new(0.0))
            .push_children(&[launcher_left, launcher_right])
            .id()
    }
}

pub struct FrigateTemplatePlugin;
impl FrigateTemplatePlugin {
    fn setup(mut commands: Commands, assets: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>) {
        let resources = FrigateResources {
            medium_ship_1_color: assets.load("art/crab.png"),
            medium_ship_1_mask: assets.load("art/crab_mask.png"),
            medium_ship_1_mesh: meshes
            .add(Mesh::from(Rectangle {
                half_size: Vec2::new(32.0, 32.0),
            }))
            .into(),
        };
        commands.insert_resource(resources);
    }
}
impl Plugin for FrigateTemplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, FrigateTemplatePlugin::setup);
        app.add_systems(
            FixedUpdate,
            (
                spawn_ships_and_despawn_spawn_commands::<RocketFrigateSpawner>,
            ),
        );
    }
}
