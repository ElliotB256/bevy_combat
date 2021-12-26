

#[derive(PartialEq, Clone, Copy)]
pub enum AttackResult {
    Hit,
    Miss,
    Blocked
}

pub struct Attack { 
    pub accuracy: f32,
    pub result: AttackResult,
}

impl Attack {
    pub fn new(accuracy: f32) -> Self {
        Attack {
            accuracy,
            result: AttackResult::Hit
        }
    }
}

