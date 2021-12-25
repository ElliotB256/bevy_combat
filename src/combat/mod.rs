use bevy::prelude::*;

use crate::game::{game_loop_run_criteria, DESPAWN_STAGE};

pub mod tools;
pub mod effects;
pub mod attack;
pub mod damage;
pub mod mortal;

pub struct Target(pub Option<Entity>);

impl Default for Target {
    fn default() -> Self {
        Target { 0: None }
    }
}

/// The team an entity is assigned to.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Team(pub i32);

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum CombatSystems {
    Set
}

#[derive(Default)]
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set_to_stage(CoreStage::Update,
            SystemSet::new().label(CombatSystems::Set)
            .with_run_criteria(game_loop_run_criteria())
            .with_system(
                tools::update_cooldowns.system().label(tools::ToolSystems::UpdateCooldowns)
            )
            .with_system(
                tools::fire_targetted_tools.system().label(tools::ToolSystems::FireTargettedTools)
            )
            .with_system(
                effects::apply_effects.system().label(effects::EffectSystems::ApplyEffects)
            )
            .with_system(
                damage::apply_damage.system().label(damage::DamageSystems::ApplyDamage)
            )
            .with_system(
                mortal::update_dieing.system().label(mortal::MortalSystems::UpdateDieing)
            )
            .with_system(
                mortal::check_for_dieing_entities.system().label(mortal::MortalSystems::CheckForDieingEntities)
            )
            .with_system(
                effects::remove_old_effects.system().label(effects::EffectSystems::RemoveOldEffects)
            )
            
        );
        app.add_system_to_stage(DESPAWN_STAGE,
            mortal::dispose_dieing.system()
        );
    }
}
