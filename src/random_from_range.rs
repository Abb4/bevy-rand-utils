use glam::{Vec2, Vec3};

pub trait RandomFromRange {
    /// Creates a random instance, setting values to random `f32` between `min` (inclusive) and `max` (exclusive).
    fn new_random(min: &f32, max: &f32) -> Self;

    /// Creates a random instance, setting values to random `f32` between `min` (inclusive) and `max` (exclusive).
    /// Negates the produced values at random.
    fn new_random_signed(min: &f32, max: &f32) -> Self;
}

impl RandomFromRange for f32 {
    fn new_random(min: &f32, max: &f32) -> Self {
        assert!(min < max);
        min + fastrand::f32() * (max - min)
    }

    fn new_random_signed(min: &f32, max: &f32) -> Self {
        assert!(min < max);
        f32::new_random(min, max) * if fastrand::bool() { 1.0 } else { -1.0 }
    }
}

impl RandomFromRange for Vec3 {
    fn new_random(min: &f32, max: &f32) -> Self {
        Vec3::new(
            f32::new_random(&min, &max),
            f32::new_random(&min, &max),
            f32::new_random(&min, &max),
        )
    }

    fn new_random_signed(min: &f32, max: &f32) -> Self {
        Vec3::new(
            f32::new_random_signed(&min, &max),
            f32::new_random_signed(&min, &max),
            f32::new_random_signed(&min, &max),
        )
    }
}

impl RandomFromRange for Vec2 {
    fn new_random(min: &f32, max: &f32) -> Self {
        Vec2::new(f32::new_random(&min, &max), f32::new_random(&min, &max))
    }

    fn new_random_signed(min: &f32, max: &f32) -> Self {
        Vec2::new(
            f32::new_random_signed(&min, &max),
            f32::new_random_signed(&min, &max),
        )
    }
}
