# Common Core Geometry

3D geometry engine written in Rust with Python and WebAssembly bindings. A heavily vibe-coded experiment.

The main goal is to demonstrate how to make non-trivial libraries for Python and JS/TS that can share the same common Rust core.
You should have a look at each separate crate for details.

## Features

- **8 Core Primitives**: Point3D, Vector3D, LineSegment, Ray, Plane, Triangle, Sphere, AABB
- **Intersection Operations**: Ray-sphere, ray-plane, ray-triangle, AABB-AABB
- **Distance Metrics**: Euclidean, Manhattan, Chebyshev
- **Transformations**: Translation and scaling via `Transformable` trait
- **SVG Rendering**: 3D to 2D projection with perspective and orthographic cameras
- **Zero Dependencies**: Pure Rust core with no external dependencies
- **Multi-Platform**: Rust library, Python bindings (via PyO3), WebAssembly (browser/Node.js)

## Quick Start

### Rust

```rust
use common_core_geometry::{Point3D, Vector3D, Ray, Sphere};
use common_core_geometry::operations::ray_sphere_intersection;

let origin = Point3D::new(0.0, 0.0, 5.0);
let direction = Vector3D::new(0.0, 0.0, -1.0);
let ray = Ray::new(origin, direction)?;

let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 2.0)?;

if let Some((t1, t2)) = ray_sphere_intersection(&ray, &sphere) {
    println!("Hit at t={} and t={}", t1, t2);
}
```

### Python

```python
from common_core_geometry import Point3D, Vector3D, Sphere, Ray

sphere = Sphere(Point3D(0, 0, 0), 1.0)
ray = Ray(Point3D(0, 0, 5), Vector3D(0, 0, -1).normalize())

result = ray.intersect_sphere(sphere)
if result:
    t, point = result
    print(f"Hit at distance {t}, point: {point}")
```

### JavaScript/TypeScript

```javascript
import * as geometry from '../path/to/do.js';

const p1 = new geometry.Point3D(0, 0, 0);
const p2 = new geometry.Point3D(3, 4, 0);
const distance = p1.distanceTo(p2); // 5.0
```

## Examples

### SVG Rendering (Rust)

```rust
use common_core_geometry::{Point3D, Vector3D, Sphere, Camera, SVGRenderer};

let camera = Camera::perspective(
    Point3D::new(10.0, 10.0, 10.0),
    Point3D::new(0.0, 0.0, 0.0),
    Vector3D::new(0.0, 1.0, 0.0),
    60.0, 16.0 / 9.0, 0.1, 100.0
);

let mut renderer = SVGRenderer::new(1920, 1080, camera);
renderer.set_background("#1a1a1a");

let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 2.0)?;
renderer.add_sphere(&sphere, "#ff6b6b", 2.0);

renderer.to_file("scene.svg")?;
```

### SVG Rendering (Python)

```python
from common_core_geometry import Point3D, Vector3D, Sphere
from common_core_geometry.svg import Camera, SVGRenderer

camera = Camera.perspective(
    position=Point3D(4, 4, 4),
    target=Point3D(0, 0, 0),
    up=Vector3D(0, 1, 0),
    fov=60.0,
    aspect=16/9
)

renderer = SVGRenderer(1600, 900, camera)
renderer.add_sphere(Sphere(Point3D(0, 0, 0), 1.0), color="#ff6b6b", width=2)
renderer.save("scene.svg")
```

### Interactive Examples

See the example directories for complete working examples:

- **Rust**: `crates/core/examples/` - Run with `cargo run --example ray_casting`
- **Python**: `crates/py/examples/` - Run with `python examples/svg_rendering.py`
- **Web**: `crates/wasm/examples/` - Run a web server and open `http://localhost:3000/examples/`

## Architecture

```
common-core-geometry/
├── crates/
│   ├── core/      # Rust library (primitives, operations, SVG)
│   ├── py/        # Python bindings (PyO3/Maturin)
│   └── wasm/      # WebAssembly bindings (wasm-bindgen)
```

### Core Modules

- **primitives**: Point3D, Vector3D, Ray, Sphere, Triangle, AABB, LineSegment, Plane
- **operations**: Distance metrics, ray intersections, transformations
- **svg**: Camera systems and SVG rendering
- **error**: Error types and Result aliases

## Testing

### Rust

```bash
cargo test                    # Run all tests
cargo test --doc              # Run doc tests
cargo run --example ray_casting
```

### Python

```bash
cd crates/py
pytest tests/ -v              # 168 tests
python examples/ray_casting.py
```

### WebAssembly

```bash
cd crates/wasm
bun run test                   # 36 Bun tests
```

## Development

### Prerequisites

- **Rust**: `rustup` (edition 2021)
- **Python**: Python 3.8+, `maturin` for building
- **WASM**: `wasm-pack` for building, `bun` for testing

### Build Commands

```bash
# Rust core library
cargo build --release

# Python bindings
cd crates/py
maturin develop --release

# WebAssembly
cd crates/wasm
bun run build:web       # Web target → pkg-web/
bun run build:nodejs    # Node.js target → pkg-nodejs/
```

### Clean Build Artifacts

```bash
# From workspace root
cargo clean              # Cleans Rust target/ directory

# From WASM crate
cd crates/wasm
bun run clean            # Cleans pkg*/ and node_modules/
```

## Documentation

```bash
# Rust API docs
cargo doc --open

# Python help
python -c "from common_core_geometry import Vector3D; help(Vector3D)"

# TypeScript types
# Type definitions (.d.ts) included in WASM packages
```

## License

MIT

## Future plans...

- [ ] Additional primitives (Cone, Cylinder, Capsule)
- [ ] Polygon clipping operations
- [ ] Convex hull algorithms
- [ ] Spatial data structures (BVH, Octree)
