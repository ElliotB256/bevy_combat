//! Module for implementing NPC artificial intelligence.

use bevy::{core::FixedTimestep, prelude::*};
use crate::constants::FIXED_TIME_STEP;

pub mod movement;
pub mod idle;
pub mod aggression;

#[derive(Default)]
pub struct AIPlugin;

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum AISystems {
    PeelManoeuvre,
    Pursue,
    TurnToDestination,
    DoRoaming,
    UpdateAggressionSource,
    DoRetargetting,
    FindTargets,
    IdleToCombat
}

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_to_stage(
            CoreStage::Update,
            movement::peel_manoeuvre
                .system()
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .label(AISystems::PeelManoeuvre)
        )
        .add_system_to_stage(
            CoreStage::Update,
            movement::pursue
                .system()
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .label(AISystems::Pursue)
        )
        .add_system_to_stage(
            CoreStage::Update,
            movement::turn_to_destination
                .system()
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .label(AISystems::TurnToDestination)
                .after(crate::movement::MovementSystems::UpdateHeading)
                .before(crate::movement::MovementSystems::UpdateRotation)
        )
        .add_system_to_stage(
            CoreStage::Update,
            idle::do_roaming
                .system()
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .label(AISystems::DoRoaming)
        )
        ;

        app.add_system_to_stage(
            CoreStage::Update,
            aggression::update_aggression_source
                .system()
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .label(AISystems::UpdateAggressionSource)
        )
        .add_system_to_stage(
            CoreStage::Update,
            aggression::do_retargetting
                .system()
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .label(AISystems::DoRetargetting)
        )
        .add_system_to_stage(
            CoreStage::Update,
            aggression::find_targets
                .system()
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .label(AISystems::FindTargets)
                .after(AISystems::UpdateAggressionSource)
        );

        app.add_system_to_stage(
            CoreStage::Update,
            idle::idle_to_combat
                .system()
                .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                .label(AISystems::IdleToCombat)
        );
    }
}
