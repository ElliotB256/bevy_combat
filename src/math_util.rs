//! Utility maths methods

use bevy::prelude::*;

/// Gets the heading given a delta vector separating two points.
/// 
/// # Arguments
/// 
/// * `delta`: difference vector equal to `target - source`.
pub fn get_heading_to_point(delta: Vec3) -> f32 {
    return delta.y.atan2(delta.x);
}

/// Returns the smallest angle difference between two stated angles.
/// 
/// # Arguments
/// 
/// * `target`: target heading.
///
/// * `initial`: initial heading.
pub fn get_angle_difference(target: f32, initial: f32) -> f32 {
    // custom mod required - mod(a,n) = a - floor(a / n) * n
    let a = (target - initial) + std::f32::consts::PI;
    let n = 2.0 * std::f32::consts::PI;
    return (a - (a / n).floor() * n) - std::f32::consts::PI;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_difference() {
        let pi = std::f32::consts::PI;
        assert!(get_angle_difference(0.0, 1.0) - -1.0 < 1.0e-4);
        assert!(get_angle_difference(1.0, 0.0) - 1.0 < 1.0e-4);
        assert!(get_angle_difference(pi, 1.0) - (pi-1.0) < 1.0e-4);
        assert!(get_angle_difference(1.0, pi) - (1.0 - pi) < 1.0e-4);
        assert!(get_angle_difference(0.5, 2.0*pi-0.5) - 1.0 < 1.0e-4);
        assert!(get_angle_difference(2.0*pi-0.5, 0.5) - -1.0 < 1.0e-4);
    }
}