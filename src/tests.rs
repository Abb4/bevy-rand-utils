#[cfg(test)]
pub mod tests {

    use glam::{Vec2, Vec3};

    use crate::random_from_distance::RandomFromDistance;
    use crate::random_from_range::RandomFromRange;

    #[test]
    pub fn should_create_f32_in_range() {
        let lower_bound = 5.0;
        let upper_bound = 10.0;

        for _ in 0..100 {
            let rand = f32::new_random(&lower_bound, &upper_bound);

            println!("{}", rand);

            assert!(rand < upper_bound);
            assert!(rand >= lower_bound);
        }
    }

    #[test]
    pub fn should_create_f32_in_range_signed() {
        let lower_bound = 5.0;
        let upper_bound = 10.0;

        for _ in 0..100 {
            let rand = f32::new_random_signed(&lower_bound, &upper_bound);

            println!("{}", rand);

            assert!(rand < upper_bound || rand > (-1.0) * upper_bound);
            assert!(rand >= lower_bound || rand <= (-1.0) * lower_bound);
        }
    }

    #[test]
    pub fn should_create_vec2_in_range() {
        let lower_bound = 5.0;
        let upper_bound = 10.0;

        for _ in 0..100 {
            let rand = Vec2::new_random(&lower_bound, &upper_bound);

            println!("{}", rand);

            assert!(rand.x < upper_bound);
            assert!(rand.y < upper_bound);

            assert!(rand.x >= lower_bound);
            assert!(rand.y >= lower_bound);
        }
    }

    #[test]
    pub fn should_create_vec2_in_range_signed() {
        let lower_bound = 5.0;
        let upper_bound = 10.0;

        for _ in 0..100 {
            let rand = Vec2::new_random_signed(&lower_bound, &upper_bound);

            println!("{}", rand);

            assert!(rand.x < upper_bound || rand.x > (-1.0) * upper_bound);
            assert!(rand.y < upper_bound || rand.y > (-1.0) * upper_bound);

            assert!(rand.x >= lower_bound || rand.x <= (-1.0) * lower_bound);
            assert!(rand.y >= lower_bound || rand.y <= (-1.0) * lower_bound);
        }
    }

    #[test]
    pub fn should_create_vec3_in_range() {
        let lower_bound = 5.0;
        let upper_bound = 10.0;

        for _ in 0..100 {
            let rand = Vec3::new_random(&lower_bound, &upper_bound);

            println!("{}", rand);

            assert!(rand.x < upper_bound);
            assert!(rand.y < upper_bound);
            assert!(rand.z < upper_bound);

            assert!(rand.x >= lower_bound);
            assert!(rand.y >= lower_bound);
            assert!(rand.z >= lower_bound);
        }
    }

    #[test]
    pub fn should_create_vec3_in_range_signed() {
        let lower_bound = 5.0;
        let upper_bound = 10.0;

        for _ in 0..100 {
            let rand = Vec3::new_random_signed(&lower_bound, &upper_bound);

            println!("{}", rand);

            assert!(rand.x < upper_bound || rand.x > (-1.0) * upper_bound);
            assert!(rand.y < upper_bound || rand.y > (-1.0) * upper_bound);
            assert!(rand.z < upper_bound || rand.z > (-1.0) * upper_bound);

            assert!(rand.x >= lower_bound || rand.x <= (-1.0) * lower_bound);
            assert!(rand.y >= lower_bound || rand.y <= (-1.0) * lower_bound);
            assert!(rand.z >= lower_bound || rand.z <= (-1.0) * lower_bound);
        }
    }

    #[test]
    pub fn should_create_vec2_within_distance() {
        let distance = 10.0;

        for _ in 0..100 {
            let rand = Vec2::new_random_from_distance(&distance);

            println!("{}, distance: {}", rand, rand.length());

            assert!(rand.length() < distance);
        }
    }
}
