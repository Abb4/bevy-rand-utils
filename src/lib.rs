#![warn(missing_docs)]
//! Implements some randomness extension traits for common [glam](https://docs.rs/glam/latest/glam/index.html) types, which are currently used by [bevy](https://bevyengine.org/)
//!
//! Impementation uses the the [fastrand](https://docs.rs/fastrand/latest/fastrand/index.html) crate.
//!
//! # Usage
//! ```ignore
//! use glam::{Vec2, Vec3};
//! use bevy_rand_utils::prelude::*;
//!
//! var f = f32::new_random(&50.0, &80.0); // generates a random f32 between 50 (inclusive) and 80 (exclusive)
//!
//! var vec1 = Vec2::new_random(&50.0, &80.0); // generates a Vec2 with x, y randomly between 50 (inclusive) and 80 (exclusive)
//! var vec2 = Vec3::new_random(&50.0, &80.0); // generates a Vec3 with random x, y, z
//! var vec3 = Vec2::new_random_from_distance(&80.0); // generates a vector at most 80 units (exclusive) away from Vec2::ZERO.
//! ```

#[doc(hidden)]
pub mod prelude;
pub use crate::prelude::*;

mod random_from_distance;
mod random_from_range;
mod tests;
mod vec2_ext;
