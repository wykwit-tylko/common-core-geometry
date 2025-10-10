pub mod error;
pub mod operations;
pub mod primitives;
pub mod utils;

pub use error::{GeometryError, Result};
pub use operations::{chebyshev_distance, manhattan_distance};
pub use primitives::{Point3D, Vector3D, LineSegment, Plane, Sphere, AABB, Triangle, Ray};
