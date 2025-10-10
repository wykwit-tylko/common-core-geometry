# common-core-geometry Python Bindings

Python bindings for the common-core-geometry library, providing high-performance 3D geometry primitives and operations with SVG rendering capabilities.

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/common-core-geometry
cd common-core-geometry/crates/py

# Install maturin
pip install maturin

# Build and install in development mode
maturin develop

# Or build a wheel
maturin build --release
```

## Features

### Core Primitives

- **Point3D** - 3D points with coordinate access and distance calculations
- **Vector3D** - 3D vectors with full arithmetic operations (add, subtract, multiply, cross product, dot product, normalize)
- **Sphere** - Spheres with center, radius, volume, surface area, and containment checks
- **Ray** - Rays with origin, direction, and intersection methods
- **Triangle** - Triangles with area, normal, and centroid calculations
- **Plane** - Planes with distance and containment checks
- **AABB** - Axis-aligned bounding boxes with intersection tests and expansion
- **LineSegment** - Line segments with length and midpoint calculations

### Ray Intersections

- Ray-Sphere intersection with distance and point
- Ray-Plane intersection
- Ray-Triangle intersection using Möller-Trumbore algorithm

### SVG Rendering

- Perspective and orthographic cameras
- Render 3D scenes to SVG format
- Customizable colors, stroke widths, and fills
- Support for all primitive types

## Quick Start

### Basic Geometry

```python
from common_core_geometry import Point3D, Vector3D, Sphere, Ray

# Create points and vectors
p = Point3D(1.0, 2.0, 3.0)
v = Vector3D(0.0, 0.0, 1.0)

# Distance between points
p2 = Point3D(4.0, 6.0, 8.0)
distance = p.distance_to(p2)

# Vector operations
v1 = Vector3D(1, 0, 0)
v2 = Vector3D(0, 1, 0)
cross = v1.cross(v2)  # Returns Vector3D(0, 0, 1)
dot = v1.dot(v2)      # Returns 0.0

# Sphere containment
sphere = Sphere(Point3D(0.0, 0.0, 0.0), 5.0)
print(f"Volume: {sphere.volume()}")           # ~523.6
print(f"Contains point: {sphere.contains(p)}") # True
```

### Ray Casting

```python
from common_core_geometry import Point3D, Vector3D, Sphere, Ray, Triangle

# Ray-sphere intersection
sphere = Sphere(Point3D(0, 0, 0), 1.0)
ray = Ray(Point3D(0, 0, 5), Vector3D(0, 0, -1).normalize())

result = ray.intersect_sphere(sphere)
if result:
    t, point = result
    print(f"Hit at distance {t}, point: {point}")

# Ray-triangle intersection
triangle = Triangle(
    Point3D(-1, -1, 0),
    Point3D(1, -1, 0),
    Point3D(0, 1, 0)
)

result = ray.intersect_triangle(triangle)
if result:
    t, point = result
    print(f"Hit triangle at {point}")
```

### SVG Rendering

```python
from common_core_geometry import Point3D, Vector3D, Sphere, Triangle, LineSegment
from common_core_geometry.svg import Camera, SVGRenderer

# Create a perspective camera
camera = Camera.perspective(
    position=Point3D(4, 4, 4),
    target=Point3D(0, 0, 0),
    up=Vector3D(0, 1, 0),
    fov=60.0,
    aspect=16/9
)

# Create renderer
renderer = SVGRenderer(1600, 900, camera)
renderer.set_background("#f0f0f0")

# Add coordinate axes
renderer.add_line_segment(
    LineSegment(Point3D(0, 0, 0), Point3D(2, 0, 0)),
    color="#ff0000", width=2  # X axis - red
)
renderer.add_line_segment(
    LineSegment(Point3D(0, 0, 0), Point3D(0, 2, 0)),
    color="#00ff00", width=2  # Y axis - green
)
renderer.add_line_segment(
    LineSegment(Point3D(0, 0, 0), Point3D(0, 0, 2)),
    color="#0000ff", width=2  # Z axis - blue
)

# Add a sphere
renderer.add_sphere(
    Sphere(Point3D(0, 0, 0), 1.0),
    color="#333333", width=1
)

# Add a triangle
triangle = Triangle(
    Point3D(1, 0, 1),
    Point3D(1, 0, -1),
    Point3D(1, 2, 0)
)
renderer.add_triangle(triangle, stroke="#9900cc", fill="#cc99ff", width=1)

# Save to file
renderer.save("scene.svg")

# Or get SVG string
svg = renderer.render()
print(svg)
```

### Context Manager Pattern

```python
from common_core_geometry.svg import Camera, SVGRenderer

with SVGRenderer(800, 600, camera) as renderer:
    renderer.add_sphere(Sphere(Point3D(0, 0, 0), 1.0))
    renderer.save("output.svg")
```

## Examples

The `examples/` directory contains complete working examples:

- **`basic_usage.py`** - Introduction to core primitives
- **`ray_casting.py`** - Ray-sphere intersection with ASCII art visualization
- **`svg_rendering.py`** - 3D scene rendering to SVG

Run examples:

```bash
python examples/ray_casting.py
python examples/svg_rendering.py
```

## API Reference

All classes and methods include comprehensive docstrings. Use Python's built-in help system:

```python
from common_core_geometry import Vector3D
help(Vector3D)
help(Vector3D.cross)
```

### Type Hints

Type stubs (`.pyi` files) are included for full IDE support and type checking with mypy:

```bash
mypy your_script.py
```

## Performance

The library is built on Rust and provides native performance for all geometry operations. Typical performance characteristics:

- Vector operations: Millions per second
- Ray-sphere intersections: Hundreds of thousands per second
- SVG rendering: Fast enough for interactive applications

## Testing

The library includes 168 comprehensive tests covering all primitives and operations:

```bash
pytest tests/ -v
```

## Requirements

- Python 3.8 or higher
- No runtime dependencies (self-contained binary)

### Development Dependencies

- pytest (for testing)
- maturin (for building from source)
- mypy (for type checking)

## Troubleshooting

### Import Errors

If you see `ImportError: No module named 'common_core_geometry'`, ensure the package is installed:

```bash
pip list | grep common-core-geometry
```

If building from source, make sure you're in a virtual environment and have run `maturin develop`.

### IDE Autocomplete Not Working

Type stubs are located in `python/common_core_geometry/`. Most IDEs should detect them automatically. If not, you may need to configure your IDE to include the package path.

## Contributing

Contributions are welcome! Please see the main repository for contribution guidelines.

## License

MIT
