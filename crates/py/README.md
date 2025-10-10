# common-core-geometry Python Bindings

Python bindings for the common-core-geometry library, providing high-performance 3D geometry primitives and operations.

## Installation

```bash
pip install common-core-geometry
```

## Features

- **Point3D** - 3D points with coordinate access
- **Vector3D** - 3D vectors with arithmetic operations (add, subtract, multiply, divide, negate)
- **Sphere** - Spheres with center, radius, volume, surface area, and containment checks
- **Ray** - Rays with origin and direction
- **Triangle** - Triangles with area, normal, and centroid calculations
- **Plane** - Planes with distance and containment checks
- **AABB** - Axis-aligned bounding boxes with intersection tests
- **LineSegment** - Line segments with length and midpoint calculations

## Quick Start

```python
from common_core_geometry import Point3D, Vector3D, Sphere, Ray

p = Point3D(1.0, 2.0, 3.0)
v = Vector3D(0.0, 0.0, 1.0)

sphere = Sphere(Point3D(0.0, 0.0, 0.0), 5.0)
print(f"Volume: {sphere.volume()}")
print(f"Contains point: {sphere.contains(p)}")

ray = Ray(Point3D(0.0, 0.0, 0.0), Vector3D(1.0, 0.0, 0.0))
point_on_ray = ray.point_at(2.5)
```

## Examples

See the `examples/` directory for more detailed usage examples.

## Documentation

For full API documentation, visit [documentation URL].

## License

MIT
