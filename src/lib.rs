//! # Common Core Geometry
//!
//! A lightweight 3D geometry engine providing fundamental geometric primitives and operations
//! for computational geometry, graphics, and spatial computing applications.
//!
//! ## Features
//!
//! - **Core Primitives**: Point3D, Vector3D, LineSegment, Ray, Plane, Triangle, Sphere, AABB
//! - **Distance Metrics**: Euclidean, Manhattan, and Chebyshev distances
//! - **Intersection Testing**: Ray-primitive intersections for ray casting and collision detection
//! - **Transformations**: Translation and scaling operations via the `Transformable` trait
//! - **SVG Rendering**: Project 3D scenes to 2D SVG with perspective and orthographic cameras
//!
//! ## Quick Start
//!
//! ```
//! use common_core_geometry::{Point3D, Vector3D, Ray, Sphere, AABB};
//! use common_core_geometry::operations::ray_sphere_intersection;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create geometric primitives
//! let point = Point3D::new(1.0, 2.0, 3.0);
//! let direction = Vector3D::new(0.0, 0.0, -1.0);
//! let ray = Ray::new(point, direction)?;
//!
//! // Test ray-sphere intersection
//! let sphere = Sphere::new(Point3D::new(1.0, 2.0, 0.0), 1.0)?;
//! if let Some((t1, t2)) = ray_sphere_intersection(&ray, &sphere) {
//!     println!("Ray hits sphere at distances: {} and {}", t1, t2);
//! }
//!
//! // Create and test axis-aligned bounding box
//! let aabb = AABB::new(
//!     Point3D::new(0.0, 0.0, 0.0),
//!     Point3D::new(5.0, 5.0, 5.0)
//! )?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Distance Operations
//!
//! ```
//! use common_core_geometry::{Point3D, manhattan_distance, chebyshev_distance};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let p1 = Point3D::new(0.0, 0.0, 0.0);
//! let p2 = Point3D::new(3.0, 4.0, 0.0);
//!
//! // Euclidean distance (built into Point3D)
//! let euclidean = p1.distance_to(&p2);
//! assert_eq!(euclidean, 5.0);
//!
//! // Manhattan distance (taxicab metric)
//! let manhattan = manhattan_distance(&p1, &p2);
//! assert_eq!(manhattan, 7.0);
//!
//! // Chebyshev distance (chessboard metric)
//! let chebyshev = chebyshev_distance(&p1, &p2);
//! assert_eq!(chebyshev, 4.0);
//! # Ok(())
//! # }
//! ```
//!
//! ## Transformations
//!
//! ```
//! use common_core_geometry::{Point3D, Sphere, Vector3D, Transformable};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0)?;
//!
//! // Translate the sphere
//! let offset = Vector3D::new(5.0, 0.0, 0.0);
//! let translated = sphere.translate(&offset);
//! assert_eq!(translated.center, Point3D::new(5.0, 0.0, 0.0));
//!
//! // Scale the sphere around origin
//! let center = Point3D::new(0.0, 0.0, 0.0);
//! let scaled = sphere.scale(&center, 2.0);
//! assert_eq!(scaled.radius, 2.0);
//! # Ok(())
//! # }
//! ```
//!
//! ## SVG Rendering
//!
//! ```
//! use common_core_geometry::{Point3D, Sphere, Vector3D, Camera, SVGRenderer};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a perspective camera
//! let camera = Camera::perspective(
//!     Point3D::new(10.0, 10.0, 10.0),   // position
//!     Point3D::new(0.0, 0.0, 0.0),      // target
//!     Vector3D::new(0.0, 1.0, 0.0),     // up vector
//!     60.0,                              // field of view
//!     800.0 / 600.0,                     // aspect ratio
//!     0.1,                               // near plane
//!     100.0,                             // far plane
//! );
//!
//! // Create SVG renderer
//! let mut renderer = SVGRenderer::new(800, 600, camera);
//! renderer.set_background("white");
//!
//! // Add geometric objects
//! let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 2.0)?;
//! renderer.add_sphere(&sphere, "blue", 2.0);
//!
//! // Render to string or file
//! let svg_string = renderer.to_svg_string();
//! // renderer.to_file("output.svg")?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Module Organization
//!
//! - [`primitives`] - Core geometric shapes and structures
//! - [`operations`] - Geometric operations (distance, intersection, transformation)
//! - [`svg`] - SVG rendering with camera projection
//! - [`error`] - Error types and result aliases
//! - [`utils`] - Utility functions and constants

pub mod error;
pub mod operations;
pub mod primitives;
pub mod svg;
pub mod utils;

pub use error::{GeometryError, Result};
pub use operations::{chebyshev_distance, manhattan_distance, Transformable};
pub use primitives::{LineSegment, Plane, Point3D, Ray, Sphere, Triangle, Vector3D, AABB};
pub use svg::{Camera, SVGRenderer};
