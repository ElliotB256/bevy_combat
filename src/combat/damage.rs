

/// Entity will deal a specified amount of damage.
pub struct Damage(pub f32);

impl Damage {
    pub fn new(damage: f32) -> Self {
        Damage { 0: damage }
    }
}