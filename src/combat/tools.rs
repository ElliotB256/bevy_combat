//! Functionality for devices that can be used to create some effect - be it spawn a projectile, damage or heal a target, etc.

use bevy::prelude::*;
use super::Target;
use crate::game::GameTimeDelta;

/// A tool that applies an effect to a target
pub struct TargettedTool {
    /// The targetting angular cone of the tool in radians.
    pub cone: f32,
    /// The range at which the tools effect can be applied.
    pub range: f32,
    /// True if the tool is prepared to fire.
    pub armed: bool,
    /// True if the tool is currently being fired this instant.
    pub firing: bool
}

/// Cooldown timer for a tool.
pub struct Cooldown {
    /// Time remaining on the cooldown.
    pub remaining: f32,
    /// Total duration of the cooldown.
    pub duration: f32,
}

impl Cooldown {
    pub fn reset(&mut self) {
        self.remaining = self.duration;
    }
    /// Is the cooldown ready?
    pub fn is_ready(&self) -> bool { self.remaining <= 0.0 }
}

/// Updates all cooldowns, decreasing remaining time by dt.
pub fn update_cooldowns(
    dt: Res<GameTimeDelta>,
    mut query: Query<&mut Cooldown>
) {
    for mut cooldown in query.iter_mut() {
        cooldown.remaining -= dt.0;
    }
}

pub fn fire_targetted_tools(
        mut query: Query<(
            &mut Cooldown,
            &mut TargettedTool,
            &Target,
            &GlobalTransform
        )>,
        pos_query: Query<&GlobalTransform>
) {

    for (mut cooldown, mut tool, target, transform) in query.iter_mut() {
        if target.0.is_none() {
            continue;
        }

        if !tool.armed {
            continue;
        }

        if !cooldown.is_ready() {
            continue;
        }

        match pos_query.get_component::<GlobalTransform>(target.0.expect("target is None")) {
            Err(_) => { continue },
            Ok(target_transform) => {
                let delta = target_transform.translation - transform.translation;

                // Cannot fire if out of weapon range
                if delta.length_squared() > tool.range * tool.range {
                    continue;
                }

                // Only fire when target is within weapon cone.
                let projection = delta.normalize().dot(transform.local_y().normalize());
                if projection < (tool.cone / 2.0).cos() {
                    continue;
                }

                // Success: Fire the tool
                tool.firing = true;
                cooldown.reset();
            }
        }
        
    }

}