# bevy-rand-utils

Implements some randomness extension traits (see `prelude`) for common [glam](https://docs.rs/glam/latest/glam/index.html) types, which are currently used by [bevy](https://bevyengine.org/).

Impementation uses the the [fastrand](https://docs.rs/fastrand/latest/fastrand/index.html) crate.

# Usage
Pull crate in by adding the following to `Cargo.toml`:

```
bevy-rand-utils = { git = "https://github.com/Abb4/bevy-rand-utils" }
```

The main branch contains stable releases. Development occurs on the `development` branch.

Use some randomness functions:

```rust
use glam::{Vec2, Vec3};
use bevy_rand_utils::prelude::*;

var f = f32::new_random(&50.0, &80.0); // generates a random f32 between 50 (inclusive) and 80 (exclusive)

var vec1 = Vec2::new_random(&50.0, &80.0); // generates a Vec2 with x, y randomly between 50 (inclusive) and 80 (exclusive)
var vec2 = Vec3::new_random(&50.0, &80.0); // generates a Vec3 with random x, y, z
var vec3 = Vec2::new_random_from_distance(&80.0); // generates a vector at most 80 units (exclusive) away from Vec2::ZERO.
```