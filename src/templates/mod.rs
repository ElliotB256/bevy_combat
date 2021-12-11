//! Bevy does not yet store prefabs.
//! 
//! As a workaround, we create template scripts which will produce different entities in our game.

pub mod weapons;
use bevy::{ecs::system::Commands, prelude::Entity};

pub trait EntityTemplate {
    fn spawn(commands: &mut Commands) -> Entity;
}