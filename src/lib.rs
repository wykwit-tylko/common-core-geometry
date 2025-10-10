pub mod error;
pub mod primitives;
pub mod utils;

pub use error::{GeometryError, Result};
pub use primitives::{Point3D, Vector3D, LineSegment, Plane, Sphere, AABB, Triangle, Ray};
