# Common Core Geometry - Workspace Migration Plan

## Overview

This plan outlines the migration from a single-crate project to a workspace structure with Python and WebAssembly bindings.

## Goals

1. Migrate existing core library to `crates/core/`
2. Set up workspace configuration
3. Implement Python bindings in `crates/py/`
4. Implement WebAssembly bindings in `crates/wasm/`
5. Maintain 100% test coverage and zero warnings
6. Ensure all existing functionality continues to work

## Phase 1: Workspace Migration

### Status: Pending

### Objectives
- Restructure project into Cargo workspace
- Move existing code to `crates/core/`
- Update all path references
- Verify all tests pass after migration

### Tasks

#### 1.1: Create Workspace Structure
- [ ] Create `crates/` directory
- [ ] Create `crates/core/` directory
- [ ] Move existing `src/` to `crates/core/src/`
- [ ] Move existing `tests/` to `crates/core/tests/`
- [ ] Move existing `examples/` to `crates/core/examples/`
- [ ] Move existing `Cargo.toml` to `crates/core/Cargo.toml`

#### 1.2: Create Workspace Root
- [ ] Create new root `Cargo.toml` with workspace configuration
- [ ] Define workspace members: `crates/core`, `crates/py`, `crates/wasm`
- [ ] Set workspace package metadata (version, edition, license)
- [ ] Define workspace dependencies

**Root `Cargo.toml`:**
```toml
[workspace]
members = [
    "crates/core",
    "crates/py",
    "crates/wasm",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/common-core-geometry"
homepage = "https://github.com/yourusername/common-core-geometry"

[workspace.dependencies]
common-core-geometry = { path = "crates/core" }
approx = "0.5"
```

#### 1.3: Update Core Crate Configuration
- [ ] Update `crates/core/Cargo.toml` to use workspace inheritance
- [ ] Verify package name remains `common-core-geometry`
- [ ] Ensure all dependencies are properly specified

**`crates/core/Cargo.toml`:**
```toml
[package]
name = "common-core-geometry"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
documentation = "https://docs.rs/common-core-geometry"
description = "A basic 3D geometry engine with fundamental primitives and operations"
keywords = ["geometry", "3d", "graphics", "computational-geometry"]
categories = ["mathematics", "graphics", "science"]

[dependencies]

[dev-dependencies]
approx.workspace = true
```

#### 1.4: Verification
- [ ] Run `cargo build` from workspace root
- [ ] Run `cargo test` from workspace root
- [ ] Run `cargo test` from `crates/core/`
- [ ] Run `cargo clippy -- -D warnings`
- [ ] Run `cargo doc --no-deps`
- [ ] Run examples: `cargo run --example ray_casting`
- [ ] Run examples: `cargo run --example svg_scene`
- [ ] Verify all 109 tests pass
- [ ] Verify zero clippy warnings

#### 1.5: Update Documentation
- [ ] Update README.md with new structure
- [ ] Update any hardcoded paths in documentation
- [ ] Add workspace structure diagram to README

### Acceptance Criteria
- ✅ Workspace builds successfully
- ✅ All 109 tests pass
- ✅ Zero clippy warnings
- ✅ Examples run correctly
- ✅ Documentation builds without errors

---

## Phase 2: Python Bindings Setup

### Status: Pending

### Objectives
- Create `crates/py/` with PyO3 bindings
- Expose core primitives to Python
- Implement Pythonic API
- Set up maturin for building
- Create test suite

### Tasks

#### 2.1: Create Python Crate Structure
- [ ] Create `crates/py/` directory
- [ ] Create `crates/py/src/`
- [ ] Create `crates/py/tests/`
- [ ] Create `crates/py/examples/`
- [ ] Create `crates/py/Cargo.toml`
- [ ] Create `crates/py/pyproject.toml`
- [ ] Create `crates/py/README.md`

#### 2.2: Configure Python Crate
- [ ] Set up `Cargo.toml` with PyO3 dependencies
- [ ] Configure `pyproject.toml` for maturin
- [ ] Set package name to `common-core-geometry`
- [ ] Set import name to `common_core_geometry`

**`crates/py/Cargo.toml`:**
```toml
[package]
name = "common-core-geometry-py"
version.workspace = true
edition.workspace = true
license.workspace = true

[lib]
name = "common_core_geometry"
crate-type = ["cdylib"]

[dependencies]
common-core-geometry.workspace = true
pyo3 = { version = "0.20", features = ["extension-module"] }

[build-dependencies]
pyo3-build-config = "0.20"
```

**`crates/py/pyproject.toml`:**
```toml
[build-system]
requires = ["maturin>=1.4,<2.0"]
build-backend = "maturin"

[project]
name = "common-core-geometry"
description = "A basic 3D geometry engine with fundamental primitives and operations"
readme = "README.md"
requires-python = ">=3.8"
license = { text = "MIT OR Apache-2.0" }
keywords = ["geometry", "3d", "graphics", "computational-geometry"]
classifiers = [
    "Development Status :: 4 - Beta",
    "Intended Audience :: Developers",
    "Intended Audience :: Science/Research",
    "Programming Language :: Rust",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3.8",
    "Programming Language :: Python :: 3.9",
    "Programming Language :: Python :: 3.10",
    "Programming Language :: Python :: 3.11",
    "Programming Language :: Python :: 3.12",
    "Topic :: Scientific/Engineering :: Mathematics",
]

[tool.maturin]
features = ["pyo3/extension-module"]
python-source = "python"
module-name = "common_core_geometry"
```

#### 2.3: Implement Core Primitives Bindings

**Point3D:**
- [ ] Create `#[pyclass]` wrapper for Point3D
- [ ] Implement `__new__` constructor
- [ ] Implement `__repr__` and `__str__`
- [ ] Implement properties: `x`, `y`, `z`
- [ ] Implement `distance_to(other: Point3D) -> float`
- [ ] Implement `midpoint(other: Point3D) -> Point3D`
- [ ] Implement `__getitem__` for indexing (0=x, 1=y, 2=z)
- [ ] Implement `__iter__` for unpacking
- [ ] Implement `__eq__` and `__hash__`

**Vector3D:**
- [ ] Create `#[pyclass]` wrapper for Vector3D
- [ ] Implement `__new__` constructor
- [ ] Implement `__repr__` and `__str__`
- [ ] Implement properties: `x`, `y`, `z`
- [ ] Implement `magnitude() -> float`
- [ ] Implement `normalize() -> Vector3D`
- [ ] Implement `dot(other: Vector3D) -> float`
- [ ] Implement `cross(other: Vector3D) -> Vector3D`
- [ ] Implement `__add__`, `__sub__` for vector operations
- [ ] Implement `__mul__` for scalar multiplication
- [ ] Implement `__len__` to return magnitude
- [ ] Implement `__getitem__` and `__iter__`

**Sphere:**
- [ ] Create `#[pyclass]` wrapper for Sphere
- [ ] Implement `__new__(center: Point3D, radius: float)`
- [ ] Implement `__repr__` and `__str__`
- [ ] Implement properties: `center`, `radius`
- [ ] Implement `volume() -> float`
- [ ] Implement `surface_area() -> float`
- [ ] Implement `contains(point: Point3D) -> bool`

**Ray:**
- [ ] Create `#[pyclass]` wrapper for Ray
- [ ] Implement `__new__(origin: Point3D, direction: Vector3D)`
- [ ] Implement `__repr__` and `__str__`
- [ ] Implement properties: `origin`, `direction`
- [ ] Implement `point_at(t: float) -> Point3D`
- [ ] Implement `intersect_sphere(sphere: Sphere) -> Option<(float, Point3D)>`
- [ ] Implement `intersect_plane(plane: Plane) -> Option<Point3D>`
- [ ] Implement `intersect_triangle(triangle: Triangle) -> Option<(float, Point3D)>`

**Triangle:**
- [ ] Create `#[pyclass]` wrapper for Triangle
- [ ] Implement `__new__(a: Point3D, b: Point3D, c: Point3D)`
- [ ] Implement `__repr__` and `__str__`
- [ ] Implement properties: `a`, `b`, `c`
- [ ] Implement `area() -> float`
- [ ] Implement `normal() -> Vector3D`
- [ ] Implement `centroid() -> Point3D`
- [ ] Implement `contains(point: Point3D) -> bool`

**Plane:**
- [ ] Create `#[pyclass]` wrapper for Plane
- [ ] Implement `__new__` with multiple constructors
- [ ] Implement `from_point_normal(point: Point3D, normal: Vector3D) -> Plane`
- [ ] Implement `from_three_points(a: Point3D, b: Point3D, c: Point3D) -> Plane`
- [ ] Implement `__repr__` and `__str__`
- [ ] Implement properties: `normal`, `d`
- [ ] Implement `distance_to(point: Point3D) -> float`
- [ ] Implement `contains(point: Point3D) -> bool`

**AABB:**
- [ ] Create `#[pyclass]` wrapper for AABB
- [ ] Implement `__new__(min: Point3D, max: Point3D)`
- [ ] Implement `__repr__` and `__str__`
- [ ] Implement properties: `min`, `max`
- [ ] Implement `volume() -> float`
- [ ] Implement `surface_area() -> float`
- [ ] Implement `contains(point: Point3D) -> bool`
- [ ] Implement `center() -> Point3D`

**LineSegment:**
- [ ] Create `#[pyclass]` wrapper for LineSegment
- [ ] Implement `__new__(start: Point3D, end: Point3D)`
- [ ] Implement `__repr__` and `__str__`
- [ ] Implement properties: `start`, `end`
- [ ] Implement `length() -> float`
- [ ] Implement `midpoint() -> Point3D`

#### 2.4: Implement SVG Module Bindings

**PerspectiveCamera:**
- [ ] Create `#[pyclass]` wrapper
- [ ] Implement constructor
- [ ] Implement properties

**OrthographicCamera:**
- [ ] Create `#[pyclass]` wrapper
- [ ] Implement constructor
- [ ] Implement properties

**SVGRenderer:**
- [ ] Create `#[pyclass]` wrapper
- [ ] Implement `__new__(width: int, height: int, camera: Camera)`
- [ ] Implement `add_point(point: Point3D, **style)`
- [ ] Implement `add_line_segment(segment: LineSegment, **style)`
- [ ] Implement `add_triangle(triangle: Triangle, **style)`
- [ ] Implement `add_sphere(sphere: Sphere, **style)`
- [ ] Implement `render() -> str`
- [ ] Implement `save(path: str)`
- [ ] Implement `__enter__` and `__exit__` for context manager

#### 2.5: Create Module Structure
- [ ] Create `crates/py/src/lib.rs` with module exports
- [ ] Create submodules: `primitives`, `svg`
- [ ] Implement `#[pymodule]` function
- [ ] Export all classes and functions

**`crates/py/src/lib.rs`:**
```rust
use pyo3::prelude::*;

mod primitives;
mod svg;

#[pymodule]
fn common_core_geometry(py: Python, m: &PyModule) -> PyResult<()> {
    // Register primitives
    m.add_class::<primitives::Point3D>()?;
    m.add_class::<primitives::Vector3D>()?;
    m.add_class::<primitives::Sphere>()?;
    m.add_class::<primitives::Ray>()?;
    m.add_class::<primitives::Triangle>()?;
    m.add_class::<primitives::Plane>()?;
    m.add_class::<primitives::AABB>()?;
    m.add_class::<primitives::LineSegment>()?;
    
    // Register SVG module
    let svg_module = PyModule::new(py, "svg")?;
    svg_module.add_class::<svg::PerspectiveCamera>()?;
    svg_module.add_class::<svg::OrthographicCamera>()?;
    svg_module.add_class::<svg::SVGRenderer>()?;
    m.add_submodule(svg_module)?;
    
    Ok(())
}
```

#### 2.6: Create Python Tests
- [ ] Create `crates/py/tests/test_point3d.py`
- [ ] Create `crates/py/tests/test_vector3d.py`
- [ ] Create `crates/py/tests/test_sphere.py`
- [ ] Create `crates/py/tests/test_ray.py`
- [ ] Create `crates/py/tests/test_triangle.py`
- [ ] Create `crates/py/tests/test_plane.py`
- [ ] Create `crates/py/tests/test_aabb.py`
- [ ] Create `crates/py/tests/test_line_segment.py`
- [ ] Create `crates/py/tests/test_svg.py`
- [ ] Create `crates/py/tests/test_integration.py`

**Example Test (`test_point3d.py`):**
```python
import pytest
from common_core_geometry import Point3D, Vector3D

def test_point3d_creation():
    p = Point3D(1.0, 2.0, 3.0)
    assert p.x == 1.0
    assert p.y == 2.0
    assert p.z == 3.0

def test_point3d_distance():
    p1 = Point3D(0.0, 0.0, 0.0)
    p2 = Point3D(1.0, 0.0, 0.0)
    assert abs(p1.distance_to(p2) - 1.0) < 1e-10

def test_point3d_unpacking():
    p = Point3D(1.0, 2.0, 3.0)
    x, y, z = p
    assert x == 1.0
    assert y == 2.0
    assert z == 3.0

def test_point3d_repr():
    p = Point3D(1.0, 2.0, 3.0)
    assert "Point3D" in repr(p)
    assert "1.0" in repr(p)
```

#### 2.7: Create Python Examples
- [ ] Create `crates/py/examples/basic_usage.py`
- [ ] Create `crates/py/examples/ray_casting.py`
- [ ] Create `crates/py/examples/svg_rendering.py`

**`basic_usage.py`:**
```python
from common_core_geometry import Point3D, Vector3D, Sphere, Ray

# Create a sphere
center = Point3D(0, 0, 0)
sphere = Sphere(center, 1.0)

print(f"Sphere volume: {sphere.volume()}")
print(f"Sphere surface area: {sphere.surface_area()}")

# Test point containment
test_point = Point3D(0.5, 0.5, 0.5)
print(f"Point {test_point} in sphere: {sphere.contains(test_point)}")

# Ray casting
origin = Point3D(0, 0, 5)
direction = Vector3D(0, 0, -1).normalize()
ray = Ray(origin, direction)

intersection = ray.intersect_sphere(sphere)
if intersection:
    t, point = intersection
    print(f"Ray hit sphere at t={t}, point={point}")
```

#### 2.8: Build and Test
- [ ] Install maturin: `pip install maturin`
- [ ] Build Python package: `cd crates/py && maturin develop`
- [ ] Run tests: `pytest crates/py/tests/`
- [ ] Run examples: `python crates/py/examples/basic_usage.py`
- [ ] Verify all tests pass

#### 2.9: Documentation
- [ ] Create `crates/py/README.md` with usage instructions
- [ ] Add docstrings to all Python classes and methods
- [ ] Create installation guide
- [ ] Create API reference

### Acceptance Criteria
- ✅ Python package builds successfully with maturin
- ✅ All primitive types exposed to Python
- ✅ All tests pass
- ✅ Examples run correctly
- ✅ Documentation complete

---

## Phase 3: WebAssembly Bindings Setup

### Status: Pending

### Objectives
- Create `crates/wasm/` with wasm-bindgen bindings
- Expose core primitives to JavaScript/TypeScript
- Implement JavaScript-friendly API
- Set up wasm-pack for building
- Create test suite and examples

### Tasks

#### 3.1: Create WASM Crate Structure
- [ ] Create `crates/wasm/` directory
- [ ] Create `crates/wasm/src/`
- [ ] Create `crates/wasm/tests/`
- [ ] Create `crates/wasm/examples/`
- [ ] Create `crates/wasm/Cargo.toml`
- [ ] Create `crates/wasm/package.json`
- [ ] Create `crates/wasm/README.md`

#### 3.2: Configure WASM Crate
- [ ] Set up `Cargo.toml` with wasm-bindgen dependencies
- [ ] Configure `package.json` for npm
- [ ] Set crate type to `cdylib`

**`crates/wasm/Cargo.toml`:**
```toml
[package]
name = "common-core-geometry-wasm"
version.workspace = true
edition.workspace = true
license.workspace = true

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
common-core-geometry.workspace = true
wasm-bindgen = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6"
js-sys = "0.3"
console_error_panic_hook = "0.1"

[dev-dependencies]
wasm-bindgen-test = "0.3"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
```

**`crates/wasm/package.json`:**
```json
{
  "name": "@common-core/geometry",
  "version": "0.1.0",
  "description": "A basic 3D geometry engine with fundamental primitives and operations",
  "main": "index.js",
  "types": "index.d.ts",
  "files": [
    "index.js",
    "index.d.ts",
    "index_bg.wasm",
    "index_bg.wasm.d.ts"
  ],
  "keywords": [
    "geometry",
    "3d",
    "graphics",
    "computational-geometry",
    "wasm",
    "webassembly"
  ],
  "license": "MIT OR Apache-2.0",
  "repository": {
    "type": "git",
    "url": "https://github.com/yourusername/common-core-geometry"
  }
}
```

#### 3.3: Implement Core Primitives Bindings

**Point3D:**
- [ ] Create `#[wasm_bindgen]` wrapper for Point3D
- [ ] Implement constructor
- [ ] Implement getters: `x()`, `y()`, `z()`
- [ ] Implement `distanceTo(other: &Point3D) -> f64`
- [ ] Implement `midpoint(other: &Point3D) -> Point3D`
- [ ] Implement `toArray() -> Vec<f64>`
- [ ] Implement `toObject() -> JsValue` (returns `{x, y, z}`)
- [ ] Implement `toJSON() -> JsValue`
- [ ] Add static method `fromArray(arr: Vec<f64>) -> Point3D`
- [ ] Add static method `fromJSON(json: JsValue) -> Point3D`

**Vector3D:**
- [ ] Create `#[wasm_bindgen]` wrapper for Vector3D
- [ ] Implement constructor
- [ ] Implement getters: `x()`, `y()`, `z()`
- [ ] Implement `magnitude() -> f64`
- [ ] Implement `normalize() -> Vector3D`
- [ ] Implement `dot(other: &Vector3D) -> f64`
- [ ] Implement `cross(other: &Vector3D) -> Vector3D`
- [ ] Implement `add(other: &Vector3D) -> Vector3D`
- [ ] Implement `sub(other: &Vector3D) -> Vector3D`
- [ ] Implement `scale(scalar: f64) -> Vector3D`
- [ ] Implement `toArray() -> Vec<f64>`
- [ ] Implement `toObject() -> JsValue`

**Sphere:**
- [ ] Create `#[wasm_bindgen]` wrapper for Sphere
- [ ] Implement `new(center: Point3D, radius: f64) -> Sphere`
- [ ] Implement getters: `center()`, `radius()`
- [ ] Implement `volume() -> f64`
- [ ] Implement `surfaceArea() -> f64`
- [ ] Implement `contains(point: &Point3D) -> bool`

**Ray:**
- [ ] Create `#[wasm_bindgen]` wrapper for Ray
- [ ] Implement `new(origin: Point3D, direction: Vector3D) -> Ray`
- [ ] Implement getters: `origin()`, `direction()`
- [ ] Implement `pointAt(t: f64) -> Point3D`
- [ ] Implement `intersectSphere(sphere: &Sphere) -> JsValue` (returns null or `{t, point}`)
- [ ] Implement `intersectPlane(plane: &Plane) -> JsValue`
- [ ] Implement `intersectTriangle(triangle: &Triangle) -> JsValue`

**Triangle:**
- [ ] Create `#[wasm_bindgen]` wrapper for Triangle
- [ ] Implement `new(a: Point3D, b: Point3D, c: Point3D) -> Triangle`
- [ ] Implement getters: `a()`, `b()`, `c()`
- [ ] Implement `area() -> f64`
- [ ] Implement `normal() -> Vector3D`
- [ ] Implement `centroid() -> Point3D`
- [ ] Implement `contains(point: &Point3D) -> bool`

**Plane:**
- [ ] Create `#[wasm_bindgen]` wrapper for Plane
- [ ] Implement static `fromPointNormal(point: Point3D, normal: Vector3D) -> Plane`
- [ ] Implement static `fromThreePoints(a: Point3D, b: Point3D, c: Point3D) -> Plane`
- [ ] Implement getters: `normal()`, `d()`
- [ ] Implement `distanceTo(point: &Point3D) -> f64`
- [ ] Implement `contains(point: &Point3D) -> bool`

**AABB:**
- [ ] Create `#[wasm_bindgen]` wrapper for AABB
- [ ] Implement `new(min: Point3D, max: Point3D) -> AABB`
- [ ] Implement getters: `min()`, `max()`
- [ ] Implement `volume() -> f64`
- [ ] Implement `surfaceArea() -> f64`
- [ ] Implement `contains(point: &Point3D) -> bool`
- [ ] Implement `center() -> Point3D`

**LineSegment:**
- [ ] Create `#[wasm_bindgen]` wrapper for LineSegment
- [ ] Implement `new(start: Point3D, end: Point3D) -> LineSegment`
- [ ] Implement getters: `start()`, `end()`
- [ ] Implement `length() -> f64`
- [ ] Implement `midpoint() -> Point3D`

#### 3.4: Implement SVG Module Bindings

**PerspectiveCamera:**
- [ ] Create `#[wasm_bindgen]` wrapper
- [ ] Implement constructor with all parameters
- [ ] Implement getters for all properties

**OrthographicCamera:**
- [ ] Create `#[wasm_bindgen]` wrapper
- [ ] Implement constructor
- [ ] Implement getters

**SVGRenderer:**
- [ ] Create `#[wasm_bindgen]` wrapper
- [ ] Implement `new(width: u32, height: u32, camera: Camera) -> SVGRenderer`
- [ ] Implement `addPoint(point: &Point3D, style: JsValue)`
- [ ] Implement `addLineSegment(segment: &LineSegment, style: JsValue)`
- [ ] Implement `addTriangle(triangle: &Triangle, style: JsValue)`
- [ ] Implement `addSphere(sphere: &Sphere, style: JsValue)`
- [ ] Implement `render() -> String`
- [ ] Add helper for parsing JS style objects

#### 3.5: Create Module Structure
- [ ] Create `crates/wasm/src/lib.rs` with module exports
- [ ] Add `console_error_panic_hook` for better error messages
- [ ] Create submodules: `primitives`, `svg`, `utils`
- [ ] Export all classes

**`crates/wasm/src/lib.rs`:**
```rust
use wasm_bindgen::prelude::*;

mod primitives;
mod svg;
mod utils;

pub use primitives::*;
pub use svg::*;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}
```

#### 3.6: Create TypeScript Definitions
- [ ] Generate TypeScript definitions with wasm-pack
- [ ] Manually create additional type definitions if needed
- [ ] Create index.d.ts with all exports

#### 3.7: Create JavaScript Tests
- [ ] Create `crates/wasm/tests/point3d.test.js`
- [ ] Create `crates/wasm/tests/vector3d.test.js`
- [ ] Create `crates/wasm/tests/sphere.test.js`
- [ ] Create `crates/wasm/tests/ray.test.js`
- [ ] Create `crates/wasm/tests/triangle.test.js`
- [ ] Create `crates/wasm/tests/plane.test.js`
- [ ] Create `crates/wasm/tests/aabb.test.js`
- [ ] Create `crates/wasm/tests/line_segment.test.js`
- [ ] Create `crates/wasm/tests/svg.test.js`

**Example Test (wasm-bindgen-test):**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_point3d_creation() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(p.x(), 1.0);
        assert_eq!(p.y(), 2.0);
        assert_eq!(p.z(), 3.0);
    }

    #[wasm_bindgen_test]
    fn test_point3d_distance() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(1.0, 0.0, 0.0);
        let dist = p1.distanceTo(&p2);
        assert!((dist - 1.0).abs() < 1e-10);
    }
}
```

#### 3.8: Create Browser Examples
- [ ] Create `crates/wasm/examples/index.html`
- [ ] Create `crates/wasm/examples/basic.html`
- [ ] Create `crates/wasm/examples/ray_casting.html`
- [ ] Create `crates/wasm/examples/svg_rendering.html`
- [ ] Add simple CSS styling

**`index.html`:**
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Common Core Geometry - WASM Demo</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
        }
        #output {
            background: #f5f5f5;
            padding: 10px;
            border-radius: 4px;
            margin: 20px 0;
        }
    </style>
</head>
<body>
    <h1>Common Core Geometry - WASM Demo</h1>
    <div id="output"></div>
    
    <script type="module">
        import init, { Point3D, Vector3D, Sphere, Ray } from '../pkg/common_core_geometry.js';
        
        async function run() {
            await init();
            
            const output = document.getElementById('output');
            
            // Create a sphere
            const center = new Point3D(0, 0, 0);
            const sphere = new Sphere(center, 1.0);
            
            output.innerHTML += `<p>Sphere volume: ${sphere.volume()}</p>`;
            output.innerHTML += `<p>Sphere surface area: ${sphere.surfaceArea()}</p>`;
            
            // Ray casting
            const origin = new Point3D(0, 0, 5);
            const direction = new Vector3D(0, 0, -1).normalize();
            const ray = new Ray(origin, direction);
            
            const intersection = ray.intersectSphere(sphere);
            if (intersection) {
                output.innerHTML += `<p>Ray hit sphere at t=${intersection.t}</p>`;
            }
        }
        
        run();
    </script>
</body>
</html>
```

#### 3.9: Build and Test
- [ ] Install wasm-pack: `cargo install wasm-pack`
- [ ] Build for bundler: `cd crates/wasm && wasm-pack build --target bundler`
- [ ] Build for web: `wasm-pack build --target web`
- [ ] Build for nodejs: `wasm-pack build --target nodejs`
- [ ] Run tests: `wasm-pack test --headless --chrome`
- [ ] Test examples in browser
- [ ] Verify binary size is reasonable (< 100KB gzipped)

#### 3.10: Documentation
- [ ] Create `crates/wasm/README.md` with usage instructions
- [ ] Add JSDoc comments to all exported functions
- [ ] Create installation guide for bundlers (webpack, vite, etc.)
- [ ] Create usage guide for vanilla HTML
- [ ] Create API reference

### Acceptance Criteria
- ✅ WASM package builds successfully with wasm-pack
- ✅ All primitive types exposed to JavaScript
- ✅ TypeScript definitions generated
- ✅ All tests pass
- ✅ Examples run in browser
- ✅ Small binary size
- ✅ Documentation complete

---

## Phase 4: Integration & Documentation

### Status: Pending

### Objectives
- Update root README with workspace information
- Create comprehensive examples
- Set up CI/CD for all crates
- Prepare for release

### Tasks

#### 4.1: Update Root README
- [ ] Add workspace structure diagram
- [ ] Add installation instructions for each package
- [ ] Add quick start for Rust, Python, and JavaScript
- [ ] Add links to crate-specific READMEs
- [ ] Update badges for all packages

**Root README structure:**
```markdown
# Common Core Geometry

A basic 3D geometry engine with fundamental primitives and operations.

## Packages

- **Rust**: `common-core-geometry` on [crates.io](https://crates.io)
- **Python**: `common-core-geometry` on [PyPI](https://pypi.org)
- **JavaScript/WASM**: `@common-core/geometry` on [npm](https://npmjs.com)

## Quick Start

### Rust
[Installation and examples]

### Python
[Installation and examples]

### JavaScript
[Installation and examples]

## Documentation

- [Rust API Docs](https://docs.rs/common-core-geometry)
- [Python API Docs](link)
- [JavaScript API Docs](link)

## License

MIT OR Apache-2.0
```

#### 4.2: Create Cross-Platform Examples
- [ ] Create `examples/` directory at root
- [ ] Create example showcasing same functionality in all three languages
- [ ] Create README for each example

**Example structure:**
```
examples/
├── ray_casting/
│   ├── README.md
│   ├── rust/
│   │   └── main.rs
│   ├── python/
│   │   └── main.py
│   └── javascript/
│       └── index.html
└── svg_rendering/
    ├── README.md
    ├── rust/
    │   └── main.rs
    ├── python/
    │   └── main.py
    └── javascript/
        └── index.html
```

#### 4.3: Update CHANGELOG
- [ ] Document workspace migration in CHANGELOG.md
- [ ] List all new features (Python bindings, WASM bindings)
- [ ] Note any breaking changes

#### 4.4: Final Verification
- [ ] Build entire workspace: `cargo build --workspace`
- [ ] Test entire workspace: `cargo test --workspace`
- [ ] Run clippy on workspace: `cargo clippy --workspace -- -D warnings`
- [ ] Build Python package and run tests
- [ ] Build WASM package and run tests
- [ ] Test all examples
- [ ] Build all documentation
- [ ] Verify README accuracy

### Acceptance Criteria
- ✅ All packages build successfully
- ✅ All tests pass in all packages
- ✅ Documentation is complete and accurate
- ✅ Examples work in all languages
- ✅ Ready for v0.1.0 release

---

## Success Metrics

### Code Quality
- [ ] 100% of existing tests still pass
- [ ] Zero clippy warnings across all crates
- [ ] All code formatted with `cargo fmt`
- [ ] Documentation builds without warnings

### Python Package
- [ ] Builds on Linux
- [ ] All primitive types exposed
- [ ] All tests pass
- [ ] Examples run successfully

### WASM Package
- [ ] Builds for bundler, web, and nodejs targets
- [ ] All primitive types exposed
- [ ] TypeScript definitions generated
- [ ] All tests pass
- [ ] Examples run in browser

### Documentation
- [ ] README updated with workspace info
- [ ] Each crate has its own README
- [ ] API documentation complete for all packages
- [ ] Examples demonstrate cross-platform usage

## Risks & Mitigation

### Risk: Breaking existing functionality
- **Mitigation**: Run all tests after each phase, maintain 100% test pass rate

### Risk: Python/WASM bindings introduce bugs
- **Mitigation**: Comprehensive test coverage in each language, property-based testing

### Risk: Build complexity increases
- **Mitigation**: Clear documentation, automated CI/CD, example configurations

### Risk: Performance overhead in bindings
- **Mitigation**: Benchmark critical paths, optimize hot code, use zero-copy where possible

