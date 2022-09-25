use bevy::prelude::{Vec2, Vec3};

/// Produces a `Vec2` at least `min_distance` and at most `max_distance` away from `origin`.
pub fn rand_vec2(origin: Vec2, min_distance: f32, max_distance: f32) -> Vec2 {
    assert!(
        min_distance <= max_distance,
        "Min distance is greater than max distance when generating a vector with offset."
    );

    let vec: Vec<f32> = Vec::from([
        random_f32_signed(min_distance, max_distance),
        random_f32_signed(min_distance, max_distance),
    ]);

    origin + Vec2::from_slice(&vec)
}

/// Produces a `Vec3` at least `min_distance` and at most `max_distance` away from `origin`.
pub fn rand_vec3(origin: Vec3, min_distance: f32, max_distance: f32) -> Vec3 {
    assert!(
        min_distance <= max_distance,
        "Min distance is greater than max distance when generating a vector with offset."
    );

    let vec: Vec<f32> = Vec::from([
        random_f32_signed(min_distance, max_distance),
        random_f32_signed(min_distance, max_distance),
        random_f32_signed(min_distance, max_distance),
    ]);

    origin + Vec3::from_slice(&vec)
}

/// Produces a `f32` between `lower_bound` and `upper_bound`.
/// Also randomly negates the result.
fn random_f32_signed(lower_bound: f32, upper_bound: f32) -> f32 {
    let offset = upper_bound - lower_bound;

    lower_bound + fastrand::f32() * offset * if fastrand::bool() { 1.0 } else { -1.0 }
}
