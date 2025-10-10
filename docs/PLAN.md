# Implementation Plan

## Overview

This document outlines the step-by-step implementation plan for the Common Core 3D geometry engine, organized by phases and dependencies.

## Phase 1: Foundation (DONE)

- 1.1 Project Setup
- 1.2 Error Handling
- 1.3 Utilities

## Phase 2: Core Primitives (DONE)

- 2.1 Point3D
- 2.2 Vector3D
- 2.3 LineSegment
- 2.4 Plane
- 2.5 Sphere
- 2.6 AABB
- 2.7 Triangle
- 2.8 Ray
- 2.9 Primitives Module

## Phase 3: Distance Operations (DONE)

- 3.1 `src/operations/distance.rs`
- 3.2 `src/operations/mod.rs`

## Phase 4: Intersection Operations (DONE)

- 4.1 Ray Intersections
- 4.2 Other Intersections

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

## Phase 10: Polish

- [ ] Run `cargo fmt`
- [ ] Run `cargo clippy` and fix warnings
- [ ] Verify all tests pass
- [ ] Verify documentation builds correctly

## Implementation Notes

1. Start with the foundation (error handling and utilities) as everything depends on them
2. Implement primitives in order of dependency (Point3D and Vector3D first)
3. Write tests alongside implementation, not after
4. Keep documentation up-to-date as you code
5. Commit frequently with meaningful messages
6. Use type-driven development - let the compiler guide you
7. Profile performance after basic implementation is complete
8. Consider using `#[inline]` for small, frequently-called methods
