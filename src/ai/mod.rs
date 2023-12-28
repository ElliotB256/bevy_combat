//! Module for implementing NPC artificial intelligence.

use bevy::prelude::*;
pub mod aggression;
pub mod idle;
pub mod movement;

#[derive(Default)]
pub struct AIPlugin;

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemSet)]
pub struct AISystems;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                movement::peel_manoeuvre,
                movement::pursue,
                movement::turn_to_destination
                    .after(crate::movement::update_heading)
                    .before(crate::movement::update_rotation),
                idle::do_roaming,
                (
                    aggression::update_aggression_source,
                    aggression::do_retargetting,
                    aggression::find_targets,
                )
                    .chain(),
                idle::idle_to_combat,
            ),
        );
    }
}
