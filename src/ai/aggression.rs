use bevy::prelude::*;
use bitflags::bitflags;
use crate::{game::GameTimeDelta, combat::Target};

bitflags! {
    #[derive(Default)]
    pub struct AgentCategory: u32 {
        const FIGHTER = 0b00000001;
        const FRIGATE = 0b00000010;
        const CRUISER = 0b00000100;
        const TURRET  = 0b00001000;
        const MISSILE = 0b00010000;
    }
}

pub struct AggroRadius(pub f32);
#[derive(Default)]
pub struct AggroLocation(pub Vec3);
pub struct RetargetBehavior {
    pub interval: f32,
    pub remaining_time: f32
}

pub const MAX_AGGRO_RADIUS : f32 = 1000.0;

#[derive(Copy, Clone)]
pub struct TargetingOrders {
    pub preferred: AgentCategory,
    pub discouraged: AgentCategory,
    pub target_same_team: bool
}

pub struct GuardBehavior {
    pub protected: Entity
}

pub fn update_aggression_source(
    mut guards_query: Query<(
        &GuardBehavior,
        &mut AggroLocation,
    )>,
    mut solo_query: Query<(
        &GlobalTransform, 
        &mut AggroLocation,
    ), Without<GuardBehavior>>,
    pos_query: Query<&GlobalTransform>
) {
    for (guard, mut aggro_loc) in guards_query.iter_mut() {
        match pos_query.get(guard.protected) {
            Err(_) => { continue },
            Ok(other_transform) => { aggro_loc.0 = other_transform.translation }
        }
    }

    for (transform, mut aggro_loc) in solo_query.iter_mut() {
        aggro_loc.0 = transform.translation
    }
}

pub fn do_retargetting(
    dt: Res<GameTimeDelta>,
    mut query: Query<(
        &mut Target,
        &mut RetargetBehavior,
    )>
) {
    for (mut target, mut retarget) in query.iter_mut() {
        retarget.remaining_time = retarget.remaining_time - dt.0;
        if retarget.remaining_time < 0.0
        {
            retarget.remaining_time = retarget.interval;
            target.0 = None;
        }
    }
}

use crate::combat::{Health, MaxHealth, Team};
use multimap::MultiMap;

pub const HASH_CELL_SIZE : f32 = 50.0;

struct TargetInformation {
    pub entity: Entity,
    pub category: AgentCategory,
    pub health_fraction: f32,
    pub position: Vec3,
    pub team: Team
}

struct Targetter {
    pub team: Team,
    pub position: Vec3,
    pub orders: TargetingOrders,
    pub radius: f32,
    pub score: f32,
    pub current_target: Target
}

impl Targetter {
    fn consider(&mut self, candidate: &TargetInformation) {

        if (self.team == candidate.team) != self.orders.target_same_team {
            return;
        }

        // Cannot target if outside aggro radius.
        let delta = (candidate.position - self.position).length_squared();
        if delta > self.radius.powi(2)
        {
            return;
        }

        let mut score = delta;
        if self.orders.target_same_team
        {
            score = candidate.health_fraction;
        }

        if self.orders.preferred.contains(candidate.category) 
        {
            score = score / 5.0;
        }
        if self.orders.discouraged.contains(candidate.category){
            score = score * 5.0;
        }

        if score > self.score{
            return;
        }

        self.score = score;
        self.current_target.0 = Some(candidate.entity);
    }
}

/// Convert a position to cell coordinates
fn get_cell_coordinates(position: Vec3) -> (i32, i32) {
    ((position.x / HASH_CELL_SIZE).floor() as i32, (position.y / HASH_CELL_SIZE).floor() as i32)
}

pub fn find_targets(
    target_query: Query<(
    Entity,
    &GlobalTransform,
    &Team,
    &AgentCategory,
    &Health,
    &MaxHealth
    )>,
    mut targetter_query: Query<(
        &AggroLocation,
        &AggroRadius,
        &Team,
        &TargetingOrders,
        &mut Target,
        )>
) {
    let mut sorted_targets = MultiMap::new();

    // Sort valid targets by position into a hashmap.
    for (entity, transform, team, category, health, max_health) in target_query.iter() {
        let health_fraction = health.0 / max_health.0;
        let position = transform.translation;
        sorted_targets.insert(
            get_cell_coordinates(position),
            TargetInformation {
                entity: entity,
                category: *category,
                health_fraction: health_fraction, 
                position: position,
                team: *team
            }
        );
        //println!("Insert {:?} at {:?}", entity, get_cell_coordinates(position));
    }

    // Pick best target for each targetter.
    for (aggro_loc, aggro_radius, team, orders, mut target) in targetter_query.iter_mut() {
        
        if target.0.is_some()
        {
            continue;
        }

        let mut targetter = Targetter {
            team: *team,
            position: aggro_loc.0,
            orders: *orders,
            radius: aggro_radius.0,
            score: std::f32::INFINITY,
            current_target: Target::default()
        };

        let min_coords = get_cell_coordinates(targetter.position - Vec3::splat(targetter.radius));
        let max_coords = get_cell_coordinates(targetter.position + Vec3::splat(targetter.radius));
        
        for x in min_coords.0..=max_coords.0 {
            for y in min_coords.1..=max_coords.1 {
                    // Identify bucket to search
                    let current_bucket = (x,y);

                    // Consider all candidate targets within each bucket.
                    match sorted_targets.get_vec(&current_bucket) {
                        None => {continue;}
                        Some(candidates) => {
                            for candidate in candidates {
                                targetter.consider(candidate);
                            }
                        }
                    }
            }
        }

        target.0 = targetter.current_target.0;
        println!("Assigned target: {:?}", target.0);
    }    

    //println!("Number of bins: {:?}", sorted_targets.keys().len());
}