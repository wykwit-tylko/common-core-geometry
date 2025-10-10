# Common Core Geometry - Workspace Specification

## Overview

This document specifies the workspace organization for the Common Core Geometry library, including the core Rust library and bindings for Python and JavaScript/WebAssembly.

## Workspace Structure

```
common-core-geometry/
├── Cargo.toml                    # Workspace manifest
├── crates/
│   ├── core/                     # Core Rust library
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── primitives/
│   │   │   ├── operations/
│   │   │   ├── svg/
│   │   │   ├── utils/
│   │   │   └── error.rs
│   │   ├── tests/
│   │   │   └── integration_tests.rs
│   │   └── examples/
│   │       ├── ray_casting.rs
│   │       └── svg_scene.rs
│   ├── py/                       # Python bindings (PyO3)
│   │   ├── Cargo.toml
│   │   ├── pyproject.toml
│   │   ├── src/
│   │   │   └── lib.rs
│   │   ├── tests/
│   │   │   └── test_geometry.py
│   │   ├── examples/
│   │   │   └── basic_usage.py
│   │   └── README.md
│   └── wasm/                     # WebAssembly bindings (wasm-bindgen)
│       ├── Cargo.toml
│       ├── package.json
│       ├── src/
│       │   └── lib.rs
│       ├── tests/
│       │   └── geometry.test.js
│       ├── examples/
│       │   └── index.html
│       └── README.md
├── docs/
│   ├── SPEC.md                   # Core library specification
│   ├── WORKSPACE_SPEC.md         # This document
│   ├── PLAN.md
│   └── API.md
├── README.md
├── CHANGELOG.md
├── LICENSE
└── .gitignore
```

## Core Library (`crates/core`)

### Purpose
The foundational Rust library containing all geometric primitives, operations, and utilities.

### Key Properties
- **Name**: `common-core-geometry`
- **Type**: Rust library crate
- **Dependencies**: Minimal (only `approx` for testing)
- **Features**: SVG rendering, serialization support (optional)

### API Surface
As defined in `docs/SPEC.md`:
- Geometric primitives (Point3D, Vector3D, LineSegment, Plane, Sphere, AABB, Triangle, Ray)
- Operations (distance, intersection, transformation)
- SVG rendering module
- Error handling with `GeometryError`

### Stability
- Core API is stable for v0.1.0
- All 109 tests passing
- Zero clippy warnings
- Well-documented with examples

## Python Bindings (`crates/py`)

### Purpose
Expose Common Core Geometry functionality to Python via PyO3, enabling use in scientific computing, data visualization, and general Python applications.

### Technology Stack
- **PyO3**: Rust-Python bindings
- **maturin**: Build tool for PyO3 projects
- **pytest**: Testing framework
- **type stubs**: `.pyi` files for type hints

### Package Details
- **PyPI Name**: `common-core-geometry`
- **Import Name**: `common_core_geometry`
- **Python Versions**: 3.8+
- **Platforms**: Linux, macOS, Windows (x86_64, aarch64)

### API Design Principles

#### 1. Pythonic Naming
```python
# Rust: Point3D::new(x, y, z)
# Python: Point3D(x, y, z)

# Rust: vector.magnitude()
# Python: vector.magnitude() or len(vector)

# Rust: vector.normalized()
# Python: vector.normalize() (returns new) or vector.normalized (property)
```

#### 2. Python Integration
```python
# Rich repr for REPL
>>> point = Point3D(1, 2, 3)
>>> point
Point3D(x=1.0, y=2.0, z=3.0)

# String conversion
>>> str(point)
"Point3D(1.0, 2.0, 3.0)"

# Sequence protocol for unpacking
>>> x, y, z = point

# Operator overloading
>>> p1 + Vector3D(1, 0, 0)
Point3D(x=2.0, y=2.0, z=3.0)

# Context managers for resources
>>> with SVGRenderer(800, 600, camera) as renderer:
...     renderer.add_sphere(sphere)
...     renderer.save("output.svg")
```

### Example Usage

```python
from common_core_geometry import Point3D, Vector3D, Sphere, Ray
from common_core_geometry.svg import PerspectiveCamera, SVGRenderer

# Create primitives
center = Point3D(0, 0, 0)
sphere = Sphere(center, 1.0)

# Ray casting
origin = Point3D(0, 0, 5)
direction = Vector3D(0, 0, -1).normalize()
ray = Ray(origin, direction)

# Intersection test
intersection = ray.intersect_sphere(sphere)
if intersection:
    t, point = intersection
    print(f"Hit at t={t}: {point}")

# SVG rendering
camera = PerspectiveCamera(
    position=Point3D(3, 3, 3),
    target=Point3D(0, 0, 0),
    up=Vector3D(0, 1, 0),
    fov=60.0,
    aspect=16/9
)

renderer = SVGRenderer(800, 600, camera)
renderer.add_sphere(sphere, stroke="#ff0000", stroke_width=2)
renderer.save("output.svg")
```

### Testing Strategy
- Unit tests for all primitive types
- Integration tests for complex workflows

### Build
```bash
# Development build
maturin develop

# Release build
maturin build --release
```

## WebAssembly Bindings (`crates/wasm`)

### Purpose
Enable Common Core Geometry in web browsers and Node.js via WebAssembly, supporting interactive 3D visualizations and computational geometry in JavaScript/TypeScript.

### Technology Stack
- **wasm-bindgen**: Rust-WASM bindings
- **wasm-pack**: Build and packaging tool
- **TypeScript**: Type definitions
- **Vitest** or **Jest**: Testing framework

### Package Details
- **Module Format**: ES modules, CommonJS, UMD
- **Target**: `bundler`, `web`, `nodejs`
- **TypeScript**: Full type definitions included

### API Design Principles

#### 1. JavaScript Naming Conventions
```javascript
// CamelCase for classes
const point = new Point3D(1, 2, 3);

// camelCase for methods
const dist = point.distanceTo(otherPoint);

// Properties where appropriate
console.log(sphere.radius);
```

#### 3. JavaScript Integration
```javascript
// JSON serialization
const json = point.toJSON();
const point2 = Point3D.fromJSON(json);

// Array conversion
const arr = point.toArray(); // [x, y, z]
const point3 = Point3D.fromArray([1, 2, 3]);

// Plain object conversion
const obj = point.toObject(); // { x: 1, y: 2, z: 3 }
```

#### 4. TypeScript Support
```typescript
export class Point3D {
    constructor(x: number, y: number, z: number);
    readonly x: number;
    readonly y: number;
    readonly z: number;
    distanceTo(other: Point3D): number;
    toArray(): [number, number, number];
    free(): void;
}

export class Sphere {
    constructor(center: Point3D, radius: number);
    readonly center: Point3D;
    readonly radius: number;
    volume(): number;
    surfaceArea(): number;
    contains(point: Point3D): boolean;
    free(): void;
}
```

### Browser Integration

```html
<!DOCTYPE html>
<html>
<head>
    <title>Common Core Geometry Demo</title>
</head>
<body>
    <div id="canvas"></div>
    <script type="module">
        import init, { Point3D, Sphere } from './pkg/common_core_geometry.js';
        
        async function run() {
            await init(); // Initialize WASM
            
            const point = new Point3D(1, 2, 3);
            const sphere = new Sphere(point, 5.0);
            
            console.log('Volume:', sphere.volume());
        }
        
        run();
    </script>
</body>
</html>
```

### Testing Strategy
- Unit tests for all exported functions
- Integration tests for rendering pipeline

### Build

```bash
# Build for bundlers (webpack, vite, etc.)
wasm-pack build --target bundler

# Build for web (no bundler)
wasm-pack build --target web

# Build for Node.js
wasm-pack build --target nodejs
```

## Workspace Configuration

### Root `Cargo.toml`

```toml
[workspace]
members = [
    "crates/core",
    "crates/py",
    "crates/wasm",
]
resolver = "3"

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT OR Apache-2.0"

[workspace.dependencies]
common-core-geometry = { path = "crates/core" }
```

### Shared Version Management
- All crates share the same version number
- Version is defined in workspace `Cargo.toml`
- Releases are synchronized across all packages

## Security Considerations

### Input Validation
- Validate all inputs from Python/JS
- Prevent buffer overflows in array operations
- Handle malformed data gracefully

### Resource Limits
- Limit mesh sizes to prevent memory exhaustion
- Timeout protection for expensive operations
- Clear error messages for resource limits

### WASM Security
- No unsafe operations exposed to JS
- Memory safety guaranteed by Rust
- No eval or dynamic code execution

## Future Enhancements

### Core Library
- Mesh support with advanced operations
- Spatial acceleration structures (BVH, octree)
- CSG operations
- Quaternion rotations
- Matrix transformations

### Python Bindings
- Pandas integration for bulk operations
- Matplotlib 3D plotting integration
- Async support for heavy computations
- GPU acceleration via CuPy

### WASM Bindings
- WebGL integration for hardware rendering
- WebWorker support for parallel operations
- Streaming for large datasets
- Three.js integration helpers

## Success Criteria

### v0.1.0 Release
- ✅ Core library with 100% test coverage
- ✅ Python package builds
- ✅ WASM package builds
- ✅ Documentation for all three packages
- ✅ Example applications for each platform

