//! Implementation of shields.

use bevy::prelude::*;

use crate::fx::animated::{AnimatedEffects, CreateAnimatedEffect};

use super::{
    attack::{Attack, AttackResult},
    damage::Damage,
    effects::{EffectLocation, SourceTransform},
    Target,
};

pub struct MaxShieldHP(pub f32);
pub struct Shield {
    pub health: f32,
    pub radius: f32,
}

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum ShieldSystems {
    AbsorbDamage,
}

/// Flag component that indicates an attack bypasses shields.
pub struct BypassShield;

pub fn shield_absorb_damage(
    mut commands: Commands,
    mut attacks_query: Query<(
        &mut Damage,
        &Target,
        &SourceTransform,
        &mut EffectLocation,
        &mut Attack,
    )>,
    mut shields_query: Query<(&mut Shield, &GlobalTransform)>,
) {
    for (mut damage, target, source_t, mut hit_loc, mut attack) in attacks_query.iter_mut() {
        // does attack target have a shield?
        if target.0.is_none() {
            continue;
        }

        // non hit attacks cannot be shielded
        if attack.result != AttackResult::Hit {
            continue;
        }

        if let Ok((mut shield, shield_transform)) =
            shields_query.get_mut(target.0.expect("target is none"))
        {
            // if attack from within shield radius, no protection given:
            let delta = source_t.0.translation - hit_loc.0;
            if delta.length_squared() < shield.radius.powi(2) {
                continue;
            }

            // shield blocks incoming damage
            let absorbed = shield.health.min(damage.0);

            if absorbed > 0.0 {
                shield.health -= absorbed;
                damage.0 -= absorbed;
                hit_loc.0 += delta.normalize() * shield.radius;
                attack.result = AttackResult::Blocked;

                // spawn a 'hit shield' effect
                commands.spawn().insert(CreateAnimatedEffect {
                    transform: Transform::from_translation(shield_transform.translation)
                        * Transform::from_rotation(Quat::from_rotation_z(
                            delta.y.atan2(delta.x) - std::f32::consts::FRAC_PI_2,
                        ))
                        * Transform::from_scale(Vec3::splat(shield.radius / 32.0)),
                    parent: None,
                    effect: AnimatedEffects::Shield,
                });
            }
        }
    }
}
