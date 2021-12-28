use bevy::prelude::*;

use super::{effects::Effect, mortal::Health, Target, attack::{Attack, AttackResult}};

/// Entity will deal a specified amount of damage.
pub struct Damage(pub f32);

impl Damage {
    pub fn new(damage: f32) -> Self {
        Damage { 0: damage }
    }
}

/// Applies damage effects to entities.
pub fn apply_damage(
    query: Query<(&Target, &Damage, &Attack), With<Effect>>,
    mut health_query: Query<&mut Health>,
) {
    for (target, damage, attack) in query.iter() {

        if attack.result != AttackResult::Hit {
            continue;
        }

        if let Some(target_entity) = target.0 {
            if let Ok(mut health) = health_query.get_mut(target_entity) {
                health.0 -= damage.0;
                //println!("damage dealt to {:?}! health now {:?}.", target_entity, health.0);
            }
        }
    }
}

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum DamageSystems {
    ApplyDamage,
}
