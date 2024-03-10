//! Rockets and their launchers

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    ai::{
        aggression::
            AgentCategory
        ,
        movement::TurnToDestinationBehavior,
    },
    combat::{
        damage::LastDamageTimer, effects::Effector, evasion::Evasion, lifetime::Lifetime, mortal::{Health, MaxHealth, Mortal}, projectile::{Homing, Projectile}, CombatSystems
    },
    fx::{animated::AnimatedEffects, death::DeathEffect},
    materials::ShipMaterial,
    movement::{Mass, MaxTurnSpeed, MovementBundle, Thrust},
};

use super::spawn::{spawn_ships_and_despawn_spawn_commands, SpawnShipTemplate};


#[derive(Resource)]
pub struct RocketResources {
    rocket_color: Handle<Image>,
    rocket_mask: Handle<Image>,
    rocket_mesh: Mesh2dHandle,
}

#[derive(Component)]
pub struct RocketSpawner;
impl SpawnShipTemplate for RocketSpawner {
    type Resources<'a> = RocketResources;

    fn spawn<'a>(
        &self,
        commands: &mut Commands,
        resources: &Res<RocketResources>,
        materials: &mut ResMut<Assets<ShipMaterial>>,
    ) -> Entity {
        commands
            .spawn({
                MaterialMesh2dBundle {
                    mesh: resources.rocket_mesh.clone(),
                    material: materials.add(ShipMaterial {
                        color: Color::rgba(0.0, 0.0, 1.0, 1.0),
                        last_damaged_time: 1.0,
                        base_texture: resources.rocket_color.clone(),
                        color_mask: resources.rocket_mask.clone(),
                    }),
                    ..default()
                }
            })
            .insert(MovementBundle {
                max_turn_speed: MaxTurnSpeed::new(10.0),
                mass: Mass(0.2),
                thrust: Thrust(100.0),
                ..default()
            })
            .insert(TurnToDestinationBehavior::default())
            .insert((
                Health(5.0),
                LastDamageTimer(10.0),
                MaxHealth(5.0),
                AgentCategory::MISSILE,
                Mortal,
                Projectile::new(),
                Homing,
                Lifetime { seconds_remaining: 8.0 },
                Effector::new(crate::templates::weapons::small_rocket_attack)
            ))
            .insert(DeathEffect {
                time_to_explosion: 0.1,
                time_to_smoke: 0.05,
                dying_explosion: AnimatedEffects::SmallExplosion,
                death_explosion: AnimatedEffects::MediumExplosion,
            })
            .insert(Evasion::new(2.0))
            .id()
    }
}

pub struct RocketTemplatePlugin;
impl RocketTemplatePlugin {
    fn setup(mut commands: Commands, assets: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>) {
        let resources = RocketResources {
            rocket_color: assets.load("art/rocket.png"),
            rocket_mask: assets.load("art/rocket_mask.png"),
            rocket_mesh: meshes
            .add(Mesh::from(Rectangle {
                half_size: Vec2::new(4.0, 8.0),
            }))
            .into(),
        };
        commands.insert_resource(resources);
    }
}
impl Plugin for RocketTemplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, RocketTemplatePlugin::setup);
        app.add_systems(
            FixedUpdate,
            (
                (spawn_ships_and_despawn_spawn_commands::<RocketSpawner>).after(CombatSystems),
            ),
        );
    }
}


pub fn small_rocket_launcher(commands: &mut Commands) -> Entity {
    commands
        .spawn(
            RocketSpawner
        )
        .id()
}