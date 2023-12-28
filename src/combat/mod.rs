use bevy::prelude::*;



pub mod attack;
pub mod damage;
pub mod effects;
pub mod evasion;
pub mod mortal;
pub mod shields;
pub mod tools;

#[derive(Clone, Copy, Component)]
#[derive(Default)]
pub struct Target(pub Option<Entity>);



/// The team an entity is assigned to.
#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub struct Team(pub i32);

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemSet)]
pub struct CombatSystems;

#[derive(Default, Component)]
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                tools::update_cooldowns,
                tools::fire_targetted_tools,
                (
                    (effects::apply_effects, evasion::calculate_evasion_ratings),
                    (
                        evasion::determine_missed_attacks,
                        shields::shield_absorb_damage,
                        damage::apply_damage,
                    )
                        .chain(),
                )
                    .chain(),
                mortal::update_dieing,
                mortal::check_for_dieing_entities,
                effects::remove_old_effects,
            )
                .in_set(CombatSystems),
        );
        app.add_systems(
            PostUpdate,
            (apply_deferred, mortal::dispose_dieing, apply_deferred).chain(),
        );
    }
}
