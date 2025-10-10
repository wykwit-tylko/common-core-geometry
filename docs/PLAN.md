# Implementation Plan

## Overview

This document outlines the step-by-step implementation plan for the Common Core 3D geometry engine, organized by phases and dependencies.

## Phase 1: Foundation

### 1.1 Project Setup
- [x] Initialize Cargo project
- [ ] Configure Cargo.toml with metadata and dependencies
- [ ] Set up module structure
- [ ] Create basic documentation

### 1.2 Error Handling
**File:** `src/error.rs`

- [ ] Define `GeometryError` enum with variants:
  - `InvalidConstruction` - Invalid geometric construction
  - `DegenerateCase` - Degenerate geometry
  - `NoIntersection` - No intersection found
  - `InvalidParameter` - Invalid function parameter
  - `DivisionByZero` - Division by zero in normalization
- [ ] Implement `Display` and `Error` traits
- [ ] Add error context with descriptive messages

### 1.3 Utilities
**File:** `src/utils/float.rs`

- [ ] Define `EPSILON` constant (1e-10)
- [ ] Implement `approx_eq(a, b)` - floating-point equality
- [ ] Implement `approx_zero(a)` - check if value is near zero
- [ ] Implement `clamp(value, min, max)`
- [ ] Add degree/radian conversion functions

**File:** `src/utils/mod.rs`
- [ ] Re-export utility functions

## Phase 2: Core Primitives

### 2.1 Point3D
**File:** `src/primitives/point.rs`

- [ ] Define `Point3D` struct with `x`, `y`, `z` fields
- [ ] Implement constructors:
  - `new(x, y, z)`
  - `origin()` - returns (0, 0, 0)
- [ ] Implement methods:
  - `distance_to(&self, other: &Point3D) -> f64`
  - `distance_squared_to(&self, other: &Point3D) -> f64`
  - `midpoint(&self, other: &Point3D) -> Point3D`
  - `translate(&self, vector: &Vector3D) -> Point3D`
- [ ] Implement traits:
  - `PartialEq` (using epsilon comparison)
  - `Debug`, `Clone`, `Copy`
  - `Add<Vector3D>`, `Sub<Point3D>` (returns Vector3D)

### 2.2 Vector3D
**File:** `src/primitives/vector.rs`

- [ ] Define `Vector3D` struct with `x`, `y`, `z` fields
- [ ] Implement constructors:
  - `new(x, y, z)`
  - `zero()` - returns (0, 0, 0)
  - `unit_x()`, `unit_y()`, `unit_z()` - unit vectors
  - `from_points(from: &Point3D, to: &Point3D)`
- [ ] Implement methods:
  - `magnitude(&self) -> f64`
  - `magnitude_squared(&self) -> f64`
  - `normalize(&self) -> Result<Vector3D, GeometryError>`
  - `dot(&self, other: &Vector3D) -> f64`
  - `cross(&self, other: &Vector3D) -> Vector3D`
  - `angle(&self, other: &Vector3D) -> f64`
  - `project_onto(&self, other: &Vector3D) -> Vector3D`
  - `is_parallel(&self, other: &Vector3D) -> bool`
  - `is_perpendicular(&self, other: &Vector3D) -> bool`
- [ ] Implement traits:
  - `PartialEq` (using epsilon comparison)
  - `Debug`, `Clone`, `Copy`
  - `Add`, `Sub`, `Mul<f64>`, `Div<f64>`, `Neg`

### 2.3 LineSegment
**File:** `src/primitives/line_segment.rs`

- [ ] Define `LineSegment` struct with `start`, `end` fields
- [ ] Implement constructors:
  - `new(start: Point3D, end: Point3D) -> Result<LineSegment, GeometryError>`
  - Validate non-degenerate (start != end)
- [ ] Implement methods:
  - `length(&self) -> f64`
  - `direction(&self) -> Vector3D`
  - `midpoint(&self) -> Point3D`
  - `point_at(&self, t: f64) -> Point3D` - parametric point (t in [0,1])
  - `closest_point(&self, point: &Point3D) -> Point3D`
  - `distance_to_point(&self, point: &Point3D) -> f64`
- [ ] Implement traits:
  - `Debug`, `Clone`, `Copy`, `PartialEq`

### 2.4 Plane
**File:** `src/primitives/plane.rs`

- [ ] Define `Plane` struct with `normal: Vector3D`, `d: f64`
- [ ] Implement constructors:
  - `new(normal: Vector3D, d: f64) -> Result<Plane, GeometryError>`
  - `from_point_normal(point: &Point3D, normal: &Vector3D) -> Result<Plane, GeometryError>`
  - `from_three_points(p1, p2, p3: &Point3D) -> Result<Plane, GeometryError>`
  - Validate non-zero normal
- [ ] Implement methods:
  - `distance_to_point(&self, point: &Point3D) -> f64` - signed distance
  - `closest_point(&self, point: &Point3D) -> Point3D`
  - `contains_point(&self, point: &Point3D) -> bool`
  - `flip_normal(&self) -> Plane`
  - `is_parallel(&self, other: &Plane) -> bool`
- [ ] Implement traits:
  - `Debug`, `Clone`, `Copy`, `PartialEq`

### 2.5 Sphere
**File:** `src/primitives/sphere.rs`

- [ ] Define `Sphere` struct with `center: Point3D`, `radius: f64`
- [ ] Implement constructors:
  - `new(center: Point3D, radius: f64) -> Result<Sphere, GeometryError>`
  - Validate radius > 0
- [ ] Implement methods:
  - `volume(&self) -> f64`
  - `surface_area(&self) -> f64`
  - `contains_point(&self, point: &Point3D) -> bool`
  - `distance_to_point(&self, point: &Point3D) -> f64`
- [ ] Implement traits:
  - `Debug`, `Clone`, `Copy`, `PartialEq`

### 2.6 AABB
**File:** `src/primitives/aabb.rs`

- [ ] Define `AABB` struct with `min: Point3D`, `max: Point3D`
- [ ] Implement constructors:
  - `new(min: Point3D, max: Point3D) -> Result<AABB, GeometryError>`
  - Validate min < max for all components
  - `from_points(points: &[Point3D]) -> Result<AABB, GeometryError>`
- [ ] Implement methods:
  - `center(&self) -> Point3D`
  - `size(&self) -> Vector3D`
  - `volume(&self) -> f64`
  - `surface_area(&self) -> f64`
  - `diagonal(&self) -> f64`
  - `contains_point(&self, point: &Point3D) -> bool`
  - `intersects(&self, other: &AABB) -> bool`
  - `union(&self, other: &AABB) -> AABB`
  - `expand_by_point(&self, point: &Point3D) -> AABB`
  - `expand_by_scalar(&self, amount: f64) -> AABB`
- [ ] Implement traits:
  - `Debug`, `Clone`, `Copy`, `PartialEq`

### 2.7 Triangle
**File:** `src/primitives/triangle.rs`

- [ ] Define `Triangle` struct with `a`, `b`, `c: Point3D`
- [ ] Implement constructors:
  - `new(a, b, c: Point3D) -> Result<Triangle, GeometryError>`
  - Validate non-collinear points
- [ ] Implement methods:
  - `normal(&self) -> Vector3D`
  - `area(&self) -> f64`
  - `centroid(&self) -> Point3D`
  - `to_plane(&self) -> Plane`
  - `bounding_box(&self) -> AABB`
  - `contains_point(&self, point: &Point3D) -> bool`
  - `barycentric_coords(&self, point: &Point3D) -> (f64, f64, f64)`
- [ ] Implement traits:
  - `Debug`, `Clone`, `Copy`, `PartialEq`

### 2.8 Ray
**File:** `src/primitives/ray.rs`

- [ ] Define `Ray` struct with `origin: Point3D`, `direction: Vector3D`
- [ ] Implement constructors:
  - `new(origin: Point3D, direction: Vector3D) -> Result<Ray, GeometryError>`
  - Validate non-zero direction and normalize it
- [ ] Implement methods:
  - `point_at(&self, t: f64) -> Point3D` - ray equation: origin + t * direction
- [ ] Implement traits:
  - `Debug`, `Clone`, `Copy`, `PartialEq`

### 2.9 Primitives Module
**File:** `src/primitives/mod.rs`

- [ ] Declare submodules
- [ ] Re-export all primitive types

## Phase 3: Distance Operations

**File:** `src/operations/distance.rs`

- [ ] Point-to-point distance (already in Point3D)
- [ ] Point-to-line segment distance (already in LineSegment)
- [ ] Point-to-plane distance (already in Plane)
- [ ] Point-to-sphere distance (already in Sphere)
- [ ] Additional distance metrics:
  - `manhattan_distance(p1: &Point3D, p2: &Point3D) -> f64`
  - `chebyshev_distance(p1: &Point3D, p2: &Point3D) -> f64`

**File:** `src/operations/mod.rs`
- [ ] Declare and re-export distance operations

## Phase 4: Intersection Operations

**File:** `src/operations/intersection.rs`

### 4.1 Ray Intersections
- [ ] `ray_plane_intersection(ray: &Ray, plane: &Plane) -> Option<Point3D>`
- [ ] `ray_sphere_intersection(ray: &Ray, sphere: &Sphere) -> Option<(f64, f64)>`
  - Returns t values for entry/exit points
- [ ] `ray_triangle_intersection(ray: &Ray, triangle: &Triangle) -> Option<f64>`
  - Möller-Trumbore algorithm
- [ ] `ray_aabb_intersection(ray: &Ray, aabb: &AABB) -> Option<(f64, f64)>`
  - Slab method

### 4.2 Other Intersections
- [ ] `aabb_aabb_intersection(a: &AABB, b: &AABB) -> bool`
- [ ] `sphere_sphere_intersection(s1: &Sphere, s2: &Sphere) -> bool`
- [ ] `plane_plane_intersection(p1: &Plane, p2: &Plane) -> Option<Line>`
  - Deferred if Line not in v0.1.0

## Phase 5: Transformation Operations

**File:** `src/operations/transform.rs`

### 5.1 Basic Transformations
- [ ] Translation (already in primitives via Vector3D addition)
- [ ] Implement `Transformable` trait:
  - `translate(&self, v: &Vector3D) -> Self`
  - `scale(&self, center: &Point3D, factor: f64) -> Self`

### 5.2 Implement Transformable for primitives
- [ ] Point3D
- [ ] LineSegment
- [ ] Sphere
- [ ] AABB
- [ ] Triangle

Note: Rotation deferred to later versions (requires quaternions/matrices)

## Phase 6: SVG Rendering Module

### 6.1 Camera
**File:** `src/svg/camera.rs`

- [ ] Define `Camera` enum:
  - `Perspective { position, target, up, fov, aspect, near, far }`
  - `Orthographic { position, target, up, width, height }`
- [ ] Implement methods:
  - `view_matrix(&self) -> [[f64; 4]; 4]`
  - `projection_matrix(&self) -> [[f64; 4]; 4]`

### 6.2 Projection
**File:** `src/svg/projection.rs`

- [ ] `project_point(point: &Point3D, camera: &Camera, width: usize, height: usize) -> (f64, f64)`
  - Transforms 3D point to 2D screen coordinates
- [ ] Helper functions for matrix operations:
  - `multiply_matrix_point(matrix: &[[f64; 4]; 4], point: &Point3D) -> Point3D`
  - `perspective_divide(point: &Point3D) -> (f64, f64)`

### 6.3 Renderer
**File:** `src/svg/renderer.rs`

- [ ] Define `SVGRenderer` struct:
  - `width: usize`
  - `height: usize`
  - `camera: Camera`
  - `background: Option<String>`
  - `elements: Vec<SVGElement>`
- [ ] Define `SVGElement` enum for different shapes
- [ ] Implement `SVGRenderer` methods:
  - `new(width, height, camera) -> Self`
  - `set_background(&mut self, color: &str)`
  - `add_point(&mut self, point: &Point3D, color: &str, size: f64)`
  - `add_line_segment(&mut self, segment: &LineSegment, color: &str, width: f64)`
  - `add_triangle(&mut self, triangle: &Triangle, stroke: &str, fill: Option<&str>, width: f64)`
  - `add_sphere(&mut self, sphere: &Sphere, color: &str, width: f64)`
  - `add_aabb(&mut self, aabb: &AABB, color: &str, width: f64)`
  - `to_string(&self) -> String`
  - `to_file(&self, path: &str) -> std::io::Result<()>`

### 6.4 SVG Module
**File:** `src/svg/mod.rs`

- [ ] Declare submodules
- [ ] Re-export public API

## Phase 7: Library Integration

**File:** `src/lib.rs`

- [ ] Declare all modules (primitives, operations, svg, utils, error)
- [ ] Re-export commonly used types
- [ ] Add module-level documentation
- [ ] Add usage examples in doc comments

## Phase 8: Testing

### 8.1 Unit Tests
For each primitive and operation file:
- [ ] Test constructors with valid inputs
- [ ] Test constructors with invalid inputs (should return errors)
- [ ] Test edge cases (zero-length, degenerate cases)
- [ ] Test numerical precision (epsilon comparisons)
- [ ] Test common operations

### 8.2 Integration Tests
**File:** `tests/integration_tests.rs`

- [ ] Test complete workflows:
  - Create scene with multiple primitives
  - Perform ray casting through scene
  - Generate SVG output
- [ ] Test camera projections
- [ ] Test complex intersections

### 8.3 Documentation Tests
- [ ] Add doc examples that compile and run
- [ ] Verify examples in SPEC.md work as code

## Phase 9: Documentation

- [ ] Add comprehensive doc comments to all public APIs
- [ ] Write README.md with:
  - Project overview
  - Installation instructions
  - Quick start examples
  - Link to docs
- [ ] Add CHANGELOG.md
- [ ] Add LICENSE file
- [ ] Add examples/ directory with sample programs

## Phase 10: Polish and Release

- [ ] Run `cargo fmt`
- [ ] Run `cargo clippy` and fix warnings
- [ ] Verify all tests pass
- [ ] Verify documentation builds correctly
- [ ] Add CI/CD configuration (GitHub Actions)
- [ ] Publish to crates.io

## Implementation Notes

1. Start with the foundation (error handling and utilities) as everything depends on them
2. Implement primitives in order of dependency (Point3D and Vector3D first)
3. Write tests alongside implementation, not after
4. Keep documentation up-to-date as you code
5. Commit frequently with meaningful messages
6. Use type-driven development - let the compiler guide you
7. Profile performance after basic implementation is complete
8. Consider using `#[inline]` for small, frequently-called methods
