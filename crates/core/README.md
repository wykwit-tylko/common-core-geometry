# Common Core Geometry

A lightweight 3D geometry engine for Rust, providing fundamental geometric primitives and operations for computational geometry, graphics, and spatial computing applications.

## Features

- **Core Primitives**: Point3D, Vector3D, LineSegment, Ray, Plane, Triangle, Sphere, AABB
- **Distance Metrics**: Euclidean, Manhattan, and Chebyshev distances
- **Intersection Testing**: Comprehensive ray-primitive intersections for ray casting and collision detection
- **Transformations**: Translation and scaling operations via the `Transformable` trait
- **SVG Rendering**: Project 3D scenes to 2D SVG with perspective and orthographic cameras
- **Zero Dependencies**: Pure Rust implementation with no external dependencies
- **Well Tested**: 100+ unit tests and integration tests with >95% code coverage

## Usage Examples

### Basics

```rust
use common_core_geometry::{Point3D, Vector3D, Ray, Sphere};
use common_core_geometry::operations::ray_sphere_intersection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create geometric primitives
    let origin = Point3D::new(0.0, 0.0, 5.0);
    let direction = Vector3D::new(0.0, 0.0, -1.0);
    let ray = Ray::new(origin, direction)?;

    // Create a sphere at the origin
    let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 2.0)?;

    // Test intersection
    if let Some((t1, t2)) = ray_sphere_intersection(&ray, &sphere) {
        println!("Ray intersects sphere at t={} and t={}", t1, t2);
        println!("Hit point 1: {:?}", ray.point_at(t1));
        println!("Hit point 2: {:?}", ray.point_at(t2));
    }

    Ok(())
}
```

### Working with Primitives

```rust
use common_core_geometry::{Point3D, Vector3D, Triangle, AABB};

// Create points
let p1 = Point3D::new(0.0, 0.0, 0.0);
let p2 = Point3D::new(1.0, 0.0, 0.0);
let p3 = Point3D::new(0.0, 1.0, 0.0);

// Create a triangle
let triangle = Triangle::new(p1, p2, p3)?;
assert_eq!(triangle.area(), 0.5);

// Create an axis-aligned bounding box
let aabb = AABB::new(
    Point3D::new(-1.0, -1.0, -1.0),
    Point3D::new(1.0, 1.0, 1.0)
)?;
```

### Distance Calculations

```rust
use common_core_geometry::{Point3D, manhattan_distance, chebyshev_distance};

let p1 = Point3D::new(0.0, 0.0, 0.0);
let p2 = Point3D::new(3.0, 4.0, 0.0);

// Euclidean distance
let euclidean = p1.distance_to(&p2);
assert_eq!(euclidean, 5.0);

// Manhattan (taxicab) distance
let manhattan = manhattan_distance(&p1, &p2);
assert_eq!(manhattan, 7.0);

// Chebyshev (chessboard) distance
let chebyshev = chebyshev_distance(&p1, &p2);
assert_eq!(chebyshev, 4.0);
```

### Ray Casting

```rust
use common_core_geometry::{Point3D, Vector3D, Ray, Sphere, AABB, Triangle};
use common_core_geometry::operations::{
    ray_sphere_intersection,
    ray_aabb_intersection,
    ray_triangle_intersection
};

let ray = Ray::new(
    Point3D::new(0.0, 0.0, 10.0),
    Vector3D::new(0.0, 0.0, -1.0)
)?;

// Test sphere intersection
let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 2.0)?;
if let Some((t1, t2)) = ray_sphere_intersection(&ray, &sphere) {
    println!("Hit sphere at {} and {}", t1, t2);
}

// Test AABB intersection
let aabb = AABB::new(
    Point3D::new(-1.0, -1.0, -1.0),
    Point3D::new(1.0, 1.0, 1.0)
)?;
if let Some((t_min, t_max)) = ray_aabb_intersection(&ray, &aabb) {
    println!("Ray enters AABB at {} and exits at {}", t_min, t_max);
}

// Test triangle intersection
let triangle = Triangle::new(
    Point3D::new(-2.0, -2.0, 0.0),
    Point3D::new(2.0, -2.0, 0.0),
    Point3D::new(0.0, 2.0, 0.0)
)?;
if let Some(t) = ray_triangle_intersection(&ray, &triangle) {
    println!("Hit triangle at {}", t);
}
```

### Transformations

```rust
use common_core_geometry::{Point3D, Sphere, Vector3D, Transformable};

let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0)?;

// Translate
let offset = Vector3D::new(5.0, 0.0, 0.0);
let translated = sphere.translate(&offset);
assert_eq!(translated.center, Point3D::new(5.0, 0.0, 0.0));

// Scale around a point
let center = Point3D::new(0.0, 0.0, 0.0);
let scaled = sphere.scale(&center, 2.0);
assert_eq!(scaled.radius, 2.0);
```

### SVG Rendering

```rust
use common_core_geometry::{Point3D, Vector3D, Sphere, AABB, Camera, SVGRenderer};

// Create a perspective camera
let camera = Camera::perspective(
    Point3D::new(10.0, 10.0, 10.0),   // position
    Point3D::new(0.0, 0.0, 0.0),      // look at
    Vector3D::new(0.0, 1.0, 0.0),     // up vector
    60.0,                              // field of view
    16.0 / 9.0,                        // aspect ratio
    0.1,                               // near plane
    100.0,                             // far plane
);

// Create renderer
let mut renderer = SVGRenderer::new(1920, 1080, camera);
renderer.set_background("#1a1a1a");

// Add primitives
let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 2.0)?;
renderer.add_sphere(&sphere, "#ff6b6b", 2.0);

let aabb = AABB::new(
    Point3D::new(-5.0, -1.0, -5.0),
    Point3D::new(5.0, -0.5, 5.0)
)?;
renderer.add_aabb(&aabb, "#4ecdc4", 1.5);

// Save to file
renderer.to_file("scene.svg")?;
```

## Architecture

The library is organized into several modules:

- **primitives**: Core geometric shapes (Point3D, Vector3D, Ray, Sphere, Triangle, AABB, etc.)
- **operations**: Geometric operations
  - `distance`: Manhattan and Chebyshev distance metrics
  - `intersection`: Ray-primitive intersection tests
  - `transform`: Transformable trait for translations and scaling
- **svg**: SVG rendering system
  - `camera`: Perspective and orthographic cameras
  - `projection`: 3D to 2D projection utilities
  - `renderer`: SVG scene renderer
- **error**: Error types and Result aliases
- **utils**: Utility functions (floating-point comparisons, angle conversion, etc.)

## Performance

Common Core Geometry is designed to be lightweight and efficient:

- Zero-cost abstractions using Rust's ownership system
- Inline hints for critical hot-path functions
- No heap allocations in core geometric operations
- Stack-allocated primitives for cache-friendly access

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_ray_sphere_intersection

# Run integration tests
cargo test --test integration_tests

# Run doc tests
cargo test --doc
```

## Documentation

Generate and view the documentation:

```bash
cargo doc --open
```

## Examples

See the `examples/` directory for complete working examples:

- `ray_casting.rs`: Ray casting through a scene
- `svg_scene.rs`: Rendering a 3D scene to SVG

Run an example:

```bash
cargo run --example ray_casting
```

## License

MIT license: <http://opensource.org/licenses/MIT>

## Roadmap

Planned features for future releases:

- [ ] Additional primitives (Cone, Cylinder, Capsule)
- [ ] Polygon clipping operations
- [ ] Convex hull algorithms
- [ ] Spatial data structures (BVH, Octree)

