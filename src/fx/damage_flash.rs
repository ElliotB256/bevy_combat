use bevy::prelude::*;

use crate::{combat::damage::LastDamageTimer, materials::ShipMaterial, game::GameTimeDelta};

pub fn update_damage_flashes(
    dt: Res<GameTimeDelta>,
    mut materials : ResMut<Assets<ShipMaterial>>,
    mut query: Query<(&mut LastDamageTimer, &Handle<ShipMaterial>)>,
) {
    for (mut timer, material) in query.iter_mut() {
        match materials.get_mut(material) {
            None => {}
            Some(material_instance) => {
                material_instance.last_damaged_time = timer.0;
            }
        }

        timer.0 += dt.0;
    }
}