use glam::{BVec3, Vec2};

use crate::random_from_range::RandomFromRange;

pub trait Vec2RandExt {
    fn randomize_using_mask(self: &mut Self, mask: BVec3, rand_min: &f32, rand_max: &f32);
}

impl Vec2RandExt for Vec2 {
    fn randomize_using_mask(self: &mut Self, mask: BVec3, rand_min: &f32, rand_max: &f32) {
        if mask.x {
            self.x = f32::new_random(&rand_min, &rand_max);
        }

        if mask.y {
            self.y = f32::new_random(&rand_min, &rand_max);
        }
    }
}
