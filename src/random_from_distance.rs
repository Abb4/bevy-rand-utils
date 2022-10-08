use glam::Vec2;

use crate::random_from_range::RandomFromRange;

pub trait RandomFromDistance {
    fn new_random_from_distance(distance: &f32) -> Self;
}

impl RandomFromDistance for f32 {
    fn new_random_from_distance(distance: &f32) -> Self {
        f32::new_random_signed(&0.0, distance)
    }
}

impl RandomFromDistance for Vec2 {
    fn new_random_from_distance(distance: &f32) -> Self {
        // see https://stackoverflow.com/a/50746409

        let radius = f32::new_random(&0.0, distance);

        let angle = f32::new_random(&0.0, distance) * 2.0 * std::f32::consts::PI;

        Vec2::new(radius * angle.cos(), radius * angle.sin())
    }
}
