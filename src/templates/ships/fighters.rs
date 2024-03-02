//! Fighter templates

// use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

// use crate::movement::MovementBundle;

// pub fn light_fighter(commands: &mut Commands) -> Entity {

// }

// pub fn attack_drone(commands: &mut Commands, spawn_transform: Transform) -> Entity {
//     let drone_size = Vec2::splat(8.0);
//     let scale = Vec3::splat(0.5);

//     commands
//         .spawn(MaterialMesh2dBundle {
//             mesh: meshes
//                 .add(Mesh::from(shape::Quad {
//                     size: Vec2::new(16.0, 16.0),
//                     flip: false,
//                 }))
//                 .into(),
//             material: materials.add(ShipMaterial {
//                 color: Color::rgba(1.0, 0.0, 0.0, 1.0),
//                 last_damaged_time: 1.0,
//                 base_texture: drone_base.clone(),
//                 color_mask: drone_mask.clone(),
//             }),
//             transform: Transform {
//                 spawn_transform,
//                 rotation,
//                 scale,
//             },
//             ..default()
//         })
//         .insert(MovementBundle {
//             velocity: Velocity::default(),
//             speed: Speed::default(),
//             max_speed: MaxSpeed::default(),
//             turn_speed: TurnSpeed::default(),
//             max_turn_speed: MaxTurnSpeed::new(4.0),
//             mass: Mass(1.0),
//             thrust: Thrust(250.0),
//             heading: Heading::default(),
//         })
//         .insert(IdleBehavior)
//         .insert(TurnToDestinationBehavior {
//             destination: Vec3::default(),
//         })
//         .insert(bevy_combat::ai::idle::RoamBehavior {
//             centre: Vec3::default(),
//             radius: 10.0,
//         })
//         .insert((
//             AggroRadius(1000.0),
//             AggroLocation::default(),
//             TargetingOrders {
//                 preferred: AgentCategory::FIGHTER,
//                 discouraged: AgentCategory::CRUISER,
//                 target_same_team: false,
//             },
//             Target::default(),
//             Team(2),
//             Health(50.0),
//             LastDamageTimer(0.0),
//             MaxHealth(50.0),
//             AgentCategory::FIGHTER,
//             Mortal,
//             RetargetBehavior {
//                 interval: 4.0,
//                 remaining_time: 4.0,
//             },
//         ))
//         .insert((
//             bevy_combat::combat::tools::Cooldown::new(0.2),
//             bevy_combat::combat::tools::TargettedTool {
//                 range: 80.0,
//                 cone: 0.3,
//                 armed: true,
//                 firing: false,
//             },
//             bevy_combat::combat::effects::Effector {
//                 spawn_effect: bevy_combat::templates::weapons::small_pulse_laser_attack,
//             },
//         ))
//         .insert(DeathEffect {
//             time_to_explosion: 0.1,
//             time_to_smoke: 0.05,
//             dying_explosion: AnimatedEffects::SmallExplosion,
//             death_explosion: AnimatedEffects::MediumExplosion,
//         })
//         .insert(Evasion::new(0.0))
//         .id()
// }