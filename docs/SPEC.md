# Common Core Geometry Engine Specification

## Overview

Common Core is a basic 3D geometry engine written in Rust, providing fundamental geometric primitives and operations for computational geometry applications in three-dimensional space.

## Core Principles

- **Type Safety**: Leverage Rust's type system for compile-time guarantees
- **Performance**: Zero-cost abstractions with efficient algorithms
- **Simplicity**: Clean, minimal API focused on essential operations
- **Correctness**: Robust handling of edge cases and numerical precision

## Geometric Primitives

### Point3D
Represents a point in 3D Euclidean space.

**Properties:**
- `x: f64` - X coordinate
- `y: f64` - Y coordinate
- `z: f64` - Z coordinate

**Operations:**
- Distance to another point
- Midpoint calculation
- Translation by vector
- Equality comparison (with epsilon for floating-point)

### Vector3D
Represents a 3D vector for direction and magnitude.

**Properties:**
- `x: f64` - X component
- `y: f64` - Y component
- `z: f64` - Z component

**Operations:**
- Magnitude/length calculation
- Normalization
- Dot product
- Cross product (returns Vector3D)
- Addition, subtraction, scalar multiplication
- Angle between vectors
- Projection onto another vector
- Orthogonal basis generation

### Line
Represents an infinite line in 3D space.

**Representations:**
- Point-direction form: point + t * direction
- Two-point form

**Operations:**
- Point-line distance
- Closest point on line to a given point
- Line-line distance (skew lines)
- Parallel/perpendicular checking
- Line-plane intersection

### LineSegment
Represents a finite line segment between two points.

**Properties:**
- `start: Point3D` - Starting point
- `end: Point3D` - Ending point

**Operations:**
- Length calculation
- Midpoint
- Contains point check
- Segment-segment closest points
- Point-segment distance
- Segment-plane intersection

### Plane
Represents an infinite plane in 3D space.

**Representations:**
- Normal form: `ax + by + cz + d = 0`
- Point-normal form
- Three-point form

**Properties:**
- `normal: Vector3D` - Normal vector (normalized)
- `d: f64` - Distance from origin

**Operations:**
- Point-plane distance (signed)
- Point on plane check
- Plane-plane intersection (returns Line)
- Line-plane intersection
- Ray-plane intersection
- Parallel/perpendicular checking
- Flip normal

### Sphere
Represents a sphere in 3D space.

**Properties:**
- `center: Point3D` - Center point
- `radius: f64` - Radius (must be positive)

**Operations:**
- Volume calculation
- Surface area calculation
- Point containment check
- Sphere-sphere intersection
- Line-sphere intersection
- Ray-sphere intersection
- Plane-sphere intersection

### AABB (Axis-Aligned Bounding Box)
Axis-aligned box in 3D space.

**Properties:**
- `min: Point3D` - Minimum corner
- `max: Point3D` - Maximum corner

**Operations:**
- Volume calculation
- Surface area calculation
- Point containment check
- AABB-AABB intersection
- Bounding box union
- Center point
- Diagonal length
- Expand by point/scalar
- Ray-AABB intersection

### Triangle
Represents a triangle defined by three points.

**Properties:**
- `a: Point3D` - First vertex
- `b: Point3D` - Second vertex
- `c: Point3D` - Third vertex

**Operations:**
- Area calculation
- Normal vector (cross product)
- Centroid calculation
- Point containment check (barycentric coordinates)
- Ray-triangle intersection
- Plane from triangle
- Bounding box

### Ray
Represents a half-infinite ray starting from an origin.

**Properties:**
- `origin: Point3D` - Starting point
- `direction: Vector3D` - Direction (normalized)

**Operations:**
- Point at parameter t
- Ray-sphere intersection
- Ray-plane intersection
- Ray-triangle intersection
- Ray-AABB intersection

### Mesh
Represents a triangular mesh.

**Properties:**
- `vertices: Vec<Point3D>` - Vertex positions
- `indices: Vec<[usize; 3]>` - Triangle indices

**Operations:**
- Bounding box calculation
- Surface area calculation
- Triangle iteration
- Normal calculation per face
- Ray-mesh intersection

## Common Operations

### Distance Metrics
- Euclidean distance
- Manhattan distance
- Chebyshev distance
- Point-to-line distance
- Point-to-plane distance
- Line-to-line distance (skew lines)

### Intersection Tests
- Point-in-shape queries
- Shape-shape intersection detection
- Intersection point/line calculation
- Ray casting for collision detection

### Transformations
- Translation
- Rotation (axis-angle, quaternion, Euler angles)
- Scaling (uniform and non-uniform)
- Reflection (across plane)
- Matrix transformations (4x4 homogeneous)

### Utility Functions
- Epsilon-based floating-point comparison
- Angle normalization
- Degree/radian conversion
- Barycentric coordinate calculation
- Normal vector computation

## Error Handling

Use `Result<T, GeometryError>` for operations that can fail:
- Invalid geometric constructions (e.g., zero-radius sphere)
- Degenerate cases (e.g., zero-length line segment, collinear triangle points)
- No intersection found
- Invalid parameters
- Division by zero in normalization

## Numerical Precision

- Use `f64` for all coordinates and measurements
- Define a global epsilon constant (e.g., `1e-10`) for floating-point comparisons
- Provide utilities for robust geometric predicates
- Handle near-zero vectors in normalization
- Careful handling of parallel/coincident cases

## API Design Goals

1. **Ergonomic**: Intuitive method names and parameter orders
2. **Consistent**: Uniform patterns across all types
3. **Composable**: Easy to combine primitives
4. **Documented**: Comprehensive doc comments with examples
5. **Tested**: Unit tests for all operations including edge cases

## SVG Rendering Module

Provides functionality for rendering 3D geometry to 2D SVG format through projection.

### Camera and Projection

**Perspective Camera:**
- `position: Point3D` - Camera position in world space
- `target: Point3D` - Look-at target
- `up: Vector3D` - Up vector
- `fov: f64` - Field of view in degrees
- `aspect: f64` - Aspect ratio (width/height)
- `near: f64` - Near clipping plane
- `far: f64` - Far clipping plane

**Orthographic Camera:**
- `position: Point3D` - Camera position
- `target: Point3D` - Look-at target
- `up: Vector3D` - Up vector
- `width: f64` - View width
- `height: f64` - View height

### SVG Renderer

**SVGRenderer:**
Converts 3D geometry to 2D SVG output.

**Configuration:**
- `width: usize` - SVG canvas width in pixels
- `height: usize` - SVG canvas height in pixels
- `camera: Camera` - Camera for projection
- `background_color: Option<String>` - Background color (CSS format)

**Supported Primitives:**
- Point3D → SVG circle
- LineSegment → SVG line
- Triangle → SVG polygon
- Sphere → SVG circle (projected)
- AABB → SVG wireframe box
- Mesh → SVG mesh wireframe/filled

**Styling Options:**
- Stroke color and width
- Fill color and opacity
- Visibility culling (back-face removal)
- Depth sorting (painter's algorithm)
- Wireframe vs filled rendering

**Output Methods:**
- `to_string()` - Returns SVG as String
- `to_file(path)` - Writes SVG to file
- `to_writer(writer)` - Writes to any `Write` trait

### Rendering Pipeline

1. **Transform**: Apply world transformations to geometry
2. **View Transform**: Convert world space to camera space
3. **Projection**: Project 3D points to 2D screen space
4. **Clipping**: Clip geometry to view frustum
5. **Depth Sort**: Sort primitives by depth (optional)
6. **Rasterize**: Convert to SVG elements with styling

### Example Usage

```rust
let camera = PerspectiveCamera::new(
    Point3D::new(0.0, 0.0, 5.0),
    Point3D::new(0.0, 0.0, 0.0),
    Vector3D::new(0.0, 1.0, 0.0),
    60.0,
    16.0 / 9.0,
);

let mut renderer = SVGRenderer::new(800, 600, camera);
renderer.set_background("#ffffff");

renderer.add_sphere(&sphere, "#ff0000", 2.0);
renderer.add_triangle(&triangle, "#00ff00", 1.0);

renderer.to_file("output.svg")?;
```

## Future Considerations

- Quaternion implementation for rotations
- Boolean operations on meshes (CSG)
- Curve primitives (Bezier, B-splines, NURBS)
- Convex hull algorithms (3D)
- Mesh simplification and subdivision
- Spatial indexing (octree, BVH, k-d tree)
- SIMD optimizations
- No-std support for embedded systems
- GPU acceleration integration points
- Advanced SVG rendering (lighting, shadows, gradients)
- Animation support (SMIL or CSS animations)
- Interactive SVG output with JavaScript

## Dependencies

Minimal external dependencies:
- Standard library only for core functionality
- Optional: `approx` crate for testing floating-point comparisons
- Optional: `serde` for serialization support

## Module Structure

```
common-core/
├── src/
│   ├── lib.rs
│   ├── primitives/
│   │   ├── mod.rs
│   │   ├── point.rs
│   │   ├── vector.rs
│   │   ├── line.rs
│   │   ├── line_segment.rs
│   │   ├── plane.rs
│   │   ├── sphere.rs
│   │   ├── aabb.rs
│   │   ├── triangle.rs
│   │   ├── ray.rs
│   │   └── mesh.rs
│   ├── operations/
│   │   ├── mod.rs
│   │   ├── distance.rs
│   │   ├── intersection.rs
│   │   └── transform.rs
│   ├── svg/
│   │   ├── mod.rs
│   │   ├── camera.rs
│   │   ├── projection.rs
│   │   └── renderer.rs
│   ├── utils/
│   │   ├── mod.rs
│   │   └── float.rs
│   └── error.rs
└── tests/
    └── integration_tests.rs
```

## Version 0.1.0 Scope

Initial release will include:
- Point3D, Vector3D
- LineSegment
- Plane
- Sphere
- AABB
- Triangle
- Ray
- Basic distance calculations
- Basic intersection tests (ray-sphere, ray-plane, ray-triangle, ray-AABB)
- Utility functions for float comparison
- SVG rendering module with perspective/orthographic projection

Deferred to later versions:
- Infinite Line primitive
- Mesh support
- Advanced transformations (quaternions, matrices)
- Convex hull and other algorithms
- Spatial acceleration structures
- Advanced SVG features (lighting, shadows, animations)
