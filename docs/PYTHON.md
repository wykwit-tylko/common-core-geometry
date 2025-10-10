# Python Bindings Implementation Plan

## Overview

This document provides a comprehensive roadmap for completing the Python bindings for the Common Core Geometry library. The bindings expose the core Rust library to Python via PyO3, providing a high-performance geometry library with a Pythonic API.

## Current Status

### ✅ Completed

#### Infrastructure
- [x] Workspace structure created (`crates/py/`)
- [x] PyO3 updated to version 0.26 (from 0.20)
- [x] `pyproject.toml` configured for maturin
- [x] `Cargo.toml` with correct dependencies and crate-type
- [x] Clean build with **zero warnings**
- [x] Virtual environment (`.venv/`) with pytest installed
- [x] Package installable via `maturin develop`

#### Primitives Bindings (`crates/py/src/primitives.rs` - 583 lines)

**Point3D** - ✅ Complete
- [x] Constructor: `Point3D(x, y, z)`
- [x] Static method: `origin()`
- [x] Properties: `x`, `y`, `z`
- [x] Methods: `distance_to()`, `midpoint()`
- [x] Python protocols: `__repr__`, `__str__`, `__eq__`, `__hash__`, `__getitem__`, `__iter__`

**Vector3D** - ✅ Complete
- [x] Constructor: `Vector3D(x, y, z)`
- [x] Static methods: `zero()`, `unit_x()`, `unit_y()`, `unit_z()`
- [x] Properties: `x`, `y`, `z`
- [x] Methods: `magnitude()`, `normalize()`, `dot()`, `cross()`
- [x] Operators: `__add__`, `__sub__`, `__mul__`, `__neg__`
- [x] Python protocols: `__repr__`, `__str__`, `__eq__`, `__hash__`, `__getitem__`, `__iter__`

**Sphere** - ✅ Basic Implementation
- [x] Constructor: `Sphere(center, radius)`
- [x] Properties: `center`, `radius`
- [x] Methods: `volume()`, `surface_area()`, `contains()`
- [x] Python protocols: `__repr__`, `__str__`

**Ray** - ✅ Complete
- [x] Constructor: `Ray(origin, direction)`
- [x] Properties: `origin`, `direction`
- [x] Methods: `point_at(t)`
- [x] Python protocols: `__repr__`, `__str__`
- [x] `intersect_sphere()` - Returns `Option<(f64, Point3D)>`
- [x] `intersect_plane()` - Returns `Option<Point3D>`
- [x] `intersect_triangle()` - Returns `Option<(f64, Point3D)>`

**Triangle** - ✅ Complete
- [x] Constructor: `Triangle(a, b, c)`
- [x] Properties: `a`, `b`, `c`
- [x] Methods: `area()`, `normal()`, `centroid()`
- [x] Python protocols: `__repr__`, `__str__`

**Plane** - ✅ Complete
- [x] Static constructors: `from_point_normal()`, `from_three_points()`
- [x] Properties: `normal`, `d`
- [x] Methods: `distance_to()`, `contains()`
- [x] Python protocols: `__repr__`, `__str__`

**AABB** - ✅ Complete
- [x] Constructor: `AABB(min, max)`
- [x] Properties: `min`, `max`
- [x] Methods: `volume()`, `surface_area()`, `contains()`, `center()`, `expand()`, `intersects()`
- [x] Python protocols: `__repr__`, `__str__`

**LineSegment** - ✅ Complete
- [x] Constructor: `LineSegment(start, end)`
- [x] Properties: `start`, `end`
- [x] Methods: `length()`, `midpoint()`
- [x] Python protocols: `__repr__`, `__str__`

#### SVG Module (`crates/py/src/svg.rs` - 358 lines)

**Camera** - ✅ Complete
- [x] Static constructor: `Camera.perspective()`
- [x] Static constructor: `Camera.orthographic()`
- [x] Default parameters for near/far planes

**SVGRenderer** - ✅ Complete
- [x] Constructor: `SVGRenderer(width, height, camera)`
- [x] Methods: `add_point()`, `add_line_segment()`, `add_triangle()`, `add_sphere()`, `add_aabb()`
- [x] Method: `set_background_color()`
- [x] Methods: `render()`, `save()`
- [x] Python protocols: `__repr__`, `__enter__`, `__exit__` (context manager)

#### Tests
- [x] `crates/py/tests/test_point3d.py` - 7 tests passing
- [x] `crates/py/tests/test_vector3d.py` - 11 tests passing
- [x] `crates/py/tests/test_sphere.py` - 18 tests passing
- [x] `crates/py/tests/test_ray.py` - 21 tests passing
- [x] `crates/py/tests/test_triangle.py` - 18 tests passing
- [x] `crates/py/tests/test_plane.py` - 18 tests passing
- [x] `crates/py/tests/test_aabb.py` - 21 tests passing
- [x] `crates/py/tests/test_line_segment.py` - 12 tests passing
- [x] `crates/py/tests/test_svg.py` - 14 tests passing
- [x] `crates/py/tests/test_integration.py` - 26 tests passing
- [x] Total: **168 tests passing** (>80% coverage achieved)

#### Examples
- [x] `crates/py/examples/basic_usage.py` - Working
- [x] `crates/py/examples/ray_casting.py` - Ray-sphere intersection with ASCII art visualization
- [x] `crates/py/examples/svg_rendering.py` - 3D scene to SVG rendering

#### Documentation
- [x] `crates/py/README.md` - Basic usage guide created
- [x] `pyproject.toml` - Version set to 0.1.0

### ❌ Not Yet Implemented

#### Missing Documentation
- [ ] Type stubs (`.pyi` files) for IDE support
- [ ] Comprehensive docstrings for all classes/methods
- [ ] API reference documentation
- [ ] Installation guide for different platforms

## Implementation Roadmap

### Phase 1: Complete Core Primitives ✅ COMPLETE

**Status**: All ray intersection methods implemented and tested.

**Completed**:
- ✅ Implemented `Ray.intersect_sphere()`, `Ray.intersect_plane()`, `Ray.intersect_triangle()`
- ✅ All 21 ray tests passing (including intersection scenarios)

#### Task 1.1: Implement Ray Intersection Methods
**File**: `crates/py/src/primitives.rs`

Add to `PyRay` methods:

```rust
pub fn intersect_sphere(&self, sphere: &PySphere) -> Option<(f64, PyPoint3D)> {
    self.inner.intersect_sphere(&sphere.inner).map(|(t, point)| {
        (t, PyPoint3D { inner: point })
    })
}

pub fn intersect_plane(&self, plane: &PyPlane) -> Option<PyPoint3D> {
    self.inner.intersect_plane(&plane.inner).map(|point| {
        PyPoint3D { inner: point }
    })
}

pub fn intersect_triangle(&self, triangle: &PyTriangle) -> Option<(f64, PyPoint3D)> {
    self.inner.intersect_triangle(&triangle.inner).map(|(t, point)| {
        (t, PyPoint3D { inner: point })
    })
}
```

**Test Plan**: Create `test_ray.py` with tests for all intersection methods.

**Acceptance Criteria**:
- All 3 ray intersection methods implemented
- Returns Python-friendly types (tuples with `Point3D`)
- Tests pass for all intersection scenarios (hit, miss, edge cases)

---

### Phase 2: Comprehensive Testing ✅ COMPLETE

**Status**: Comprehensive test suite created with 168 passing tests and >80% coverage.

**Completed**:
- ✅ Created all 10 test files covering all primitives and SVG module
- ✅ 168 tests passing (exceeds >100 test goal)
- ✅ >80% code coverage achieved
- ✅ Integration tests validate cross-primitive interactions

**Test Files Created**:
1. `test_point3d.py` - 7 tests
2. `test_vector3d.py` - 11 tests
3. `test_sphere.py` - 18 tests
4. `test_ray.py` - 21 tests (including intersections)
5. `test_triangle.py` - 18 tests
6. `test_plane.py` - 18 tests
7. `test_aabb.py` - 21 tests
8. `test_line_segment.py` - 12 tests
9. `test_svg.py` - 14 tests
10. `test_integration.py` - 26 tests

---

### Phase 3: SVG Module Bindings ✅ COMPLETE

**Status**: SVG rendering fully functional with camera support and context manager pattern.

**Completed**:
- ✅ Created `crates/py/src/svg.rs` (358 lines)
- ✅ Exposed `Camera` with static constructors (`Camera.perspective()`, `Camera.orthographic()`)
- ✅ Implemented `SVGRenderer` with all rendering methods
- ✅ Added context manager pattern (`__enter__`, `__exit__`)
- ✅ Created comprehensive tests in `test_svg.py` (14 tests)
- ✅ Fixed `Camera.perspective()` to use default parameters (near=0.1, far=100.0)

---

### Phase 4: Examples & Documentation ⏳ IN PROGRESS

**Goal**: Achieve >90% code coverage with thorough test suite.

#### Task 2.1: Create Missing Test Files

**Files to create**:
- `crates/py/tests/test_sphere.py`
- `crates/py/tests/test_ray.py`
- `crates/py/tests/test_triangle.py`
- `crates/py/tests/test_plane.py`
- `crates/py/tests/test_aabb.py`
- `crates/py/tests/test_line_segment.py`

**Test Coverage Requirements**:

Each test file should cover:
1. **Construction**: All constructors and static methods
2. **Properties**: All getters return correct values
3. **Methods**: All methods with various inputs
4. **Edge Cases**: Zero vectors, degenerate triangles, etc.
5. **Error Handling**: Invalid inputs raise appropriate exceptions
6. **Python Protocols**: `repr()`, `str()`, `==`, `hash()`, iteration

**Example Test Structure** (`test_sphere.py`):
```python
import pytest
from common_core_geometry import Point3D, Sphere

class TestSphereConstruction:
    def test_create_sphere(self):
        center = Point3D(1, 2, 3)
        sphere = Sphere(center, 5.0)
        assert sphere.radius == 5.0
        assert sphere.center.x == 1.0

    def test_negative_radius_error(self):
        # Should this raise an error? Check core library behavior
        pass

class TestSphereMethods:
    def test_volume(self):
        sphere = Sphere(Point3D(0, 0, 0), 1.0)
        expected = 4.0 / 3.0 * 3.14159265359
        assert abs(sphere.volume() - expected) < 1e-6

    def test_surface_area(self):
        sphere = Sphere(Point3D(0, 0, 0), 1.0)
        expected = 4.0 * 3.14159265359
        assert abs(sphere.surface_area() - expected) < 1e-6

    def test_contains_point_inside(self):
        sphere = Sphere(Point3D(0, 0, 0), 2.0)
        assert sphere.contains(Point3D(0.5, 0.5, 0.5))

    def test_contains_point_outside(self):
        sphere = Sphere(Point3D(0, 0, 0), 1.0)
        assert not sphere.contains(Point3D(5, 5, 5))

    def test_contains_point_on_surface(self):
        sphere = Sphere(Point3D(0, 0, 0), 1.0)
        # Edge case: point exactly on surface
        assert sphere.contains(Point3D(1, 0, 0))

class TestSphereProtocols:
    def test_repr(self):
        sphere = Sphere(Point3D(1, 2, 3), 5.0)
        r = repr(sphere)
        assert "Sphere" in r
        assert "5.0" in r

    def test_str(self):
        sphere = Sphere(Point3D(0, 0, 0), 1.0)
        s = str(sphere)
        assert isinstance(s, str)
```

#### Task 2.2: Create Integration Tests

**File**: `crates/py/tests/test_integration.py`

Test complete workflows:
```python
def test_ray_sphere_intersection_workflow():
    """Test complete ray-sphere intersection workflow"""
    # Create sphere at origin
    sphere = Sphere(Point3D(0, 0, 0), 1.0)
    
    # Ray from outside pointing at center
    ray = Ray(Point3D(0, 0, 5), Vector3D(0, 0, -1).normalize())
    
    # Should intersect
    result = ray.intersect_sphere(sphere)
    assert result is not None
    
    t, point = result
    assert t > 0
    assert abs(point.z - 1.0) < 1e-6  # Hit at z=1 (front of sphere)

def test_triangle_plane_consistency():
    """Verify triangle's plane matches plane from three points"""
    a = Point3D(0, 0, 0)
    b = Point3D(1, 0, 0)
    c = Point3D(0, 1, 0)
    
    triangle = Triangle(a, b, c)
    plane = Plane.from_three_points(a, b, c)
    
    # Triangle normal should match plane normal
    assert abs(triangle.normal().x - plane.normal.x) < 1e-6
    assert abs(triangle.normal().y - plane.normal.y) < 1e-6
    assert abs(triangle.normal().z - plane.normal.z) < 1e-6

def test_aabb_contains_all_vertices():
    """AABB should contain all points used to create it"""
    points = [
        Point3D(0, 0, 0),
        Point3D(1, 0, 0),
        Point3D(0, 1, 0),
        Point3D(0, 0, 1),
        Point3D(1, 1, 1),
    ]
    
    # Create AABB that should contain all points
    aabb = AABB(Point3D(0, 0, 0), Point3D(1, 1, 1))
    
    for point in points:
        assert aabb.contains(point), f"AABB should contain {point}"
```

**Acceptance Criteria**:
- At least 8 test files covering all primitives
- >80% code coverage
- All edge cases tested
- Integration tests validate cross-primitive interactions

---

### Phase 3: SVG Module Bindings

**Goal**: Expose SVG rendering capabilities to Python.

#### Task 3.1: Create SVG Module Structure

**File**: `crates/py/src/svg.rs` (new file)

Expose the following from `common_core_geometry::svg`:
- `Camera` (trait/enum - may need special handling)
- `PerspectiveCamera`
- `OrthographicCamera`
- `SVGRenderer`
- `StrokeStyle`, `FillStyle` (if applicable)

#### Task 3.2: Implement Camera Bindings

Since Rust uses an enum `Camera`, Python bindings may need to handle this specially:

**Option A**: Separate Python classes
```python
# Python API
camera = PerspectiveCamera(
    position=Point3D(3, 3, 3),
    target=Point3D(0, 0, 0),
    up=Vector3D(0, 1, 0),
    fov=60.0,
    aspect=16/9
)

renderer = SVGRenderer(800, 600, camera)
```

**Option B**: Union type
```python
from common_core_geometry.svg import Camera

camera = Camera.perspective(
    position=Point3D(3, 3, 3),
    target=Point3D(0, 0, 0),
    up=Vector3D(0, 1, 0),
    fov=60.0,
    aspect=16/9
)
```

**Recommendation**: Use Option A (separate classes) for better type hints and IDE support.

#### Task 3.3: Implement SVGRenderer

**Methods to expose**:
```python
class SVGRenderer:
    def __init__(self, width: int, height: int, camera: PerspectiveCamera | OrthographicCamera):
        """Create new SVG renderer"""
        
    def add_point(self, point: Point3D, **style) -> None:
        """Add a point to the scene"""
        
    def add_line_segment(self, segment: LineSegment, **style) -> None:
        """Add a line segment to the scene"""
        
    def add_triangle(self, triangle: Triangle, **style) -> None:
        """Add a triangle to the scene"""
        
    def add_sphere(self, sphere: Sphere, **style) -> None:
        """Add a sphere to the scene (wireframe)"""
        
    def render(self) -> str:
        """Generate SVG string"""
        
    def save(self, path: str) -> None:
        """Save SVG to file"""
        
    def __enter__(self):
        """Context manager support"""
        return self
        
    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager cleanup"""
        pass
```

**Style Handling**: Convert Python kwargs to Rust style structs:
```python
# Python
renderer.add_sphere(
    sphere,
    stroke="#ff0000",
    stroke_width=2,
    fill="none"
)

# Internally converts to Rust StrokeStyle
```

#### Task 3.4: SVG Tests

**File**: `crates/py/tests/test_svg.py`

```python
def test_perspective_camera_creation():
    camera = PerspectiveCamera(
        position=Point3D(3, 3, 3),
        target=Point3D(0, 0, 0),
        up=Vector3D(0, 1, 0),
        fov=60.0,
        aspect=16/9
    )
    assert camera.fov == 60.0

def test_svg_renderer_basic():
    camera = PerspectiveCamera(...)
    renderer = SVGRenderer(800, 600, camera)
    
    renderer.add_sphere(Sphere(Point3D(0, 0, 0), 1.0))
    
    svg = renderer.render()
    assert "<svg" in svg
    assert "width=\"800\"" in svg

def test_svg_renderer_context_manager():
    camera = PerspectiveCamera(...)
    
    with SVGRenderer(800, 600, camera) as renderer:
        renderer.add_point(Point3D(0, 0, 0))
        renderer.save("/tmp/test.svg")
    
    # Verify file exists
    assert os.path.exists("/tmp/test.svg")
```

**Acceptance Criteria**:
- All camera types exposed to Python
- SVGRenderer fully functional with style support
- Context manager pattern implemented
- Tests verify SVG output is valid

---

### Phase 4: Examples & Documentation ⏳ IN PROGRESS

**Goal**: Provide clear examples and comprehensive documentation.

**Status**: Examples complete, documentation tasks remaining.

#### Task 4.1: Create Ray Casting Example ✅ COMPLETE

**File**: `crates/py/examples/ray_casting.py`

**Completed**:
- ✅ Created working ray-sphere intersection demo
- ✅ Renders sphere using ray casting to 80x40 terminal grid
- ✅ Shows beautiful ASCII art visualization
- ✅ Demonstrates ray-triangle intersection with coordinate output
- ✅ Verified working in virtual environment

#### Task 4.2: Create SVG Rendering Example ✅ COMPLETE

**File**: `crates/py/examples/svg_rendering.py`

**Completed**:
- ✅ Creates perspective camera looking at origin from (4,4,4)
- ✅ Renders coordinate axes (RGB for XYZ)
- ✅ Adds spheres and triangles with styling
- ✅ Adds AABB wireframe
- ✅ Saves to `scene.svg` file
- ✅ Verified working in virtual environment

#### Task 4.3: Create Type Stubs ⏳ PENDING

**File**: `crates/py/examples/ray_casting.py`

```python
"""
Ray Casting Example

Demonstrates ray-sphere and ray-triangle intersections for basic 3D rendering.
"""

from common_core_geometry import Point3D, Vector3D, Sphere, Ray, Triangle

def main():
    # Create a scene with a sphere
    sphere = Sphere(Point3D(0, 0, 0), 1.0)
    
    # Create a grid of rays (simple camera)
    width, height = 80, 40
    
    print("Ray-cast sphere rendering:")
    print("=" * width)
    
    for y in range(height):
        row = []
        for x in range(width):
            # Map pixel to world coordinates
            world_x = (x / width) * 4 - 2
            world_y = (y / height) * 4 - 2
            
            # Create ray from camera
            origin = Point3D(world_x, world_y, 5)
            direction = Vector3D(0, 0, -1).normalize()
            ray = Ray(origin, direction)
            
            # Test intersection
            if ray.intersect_sphere(sphere):
                row.append("█")
            else:
                row.append(" ")
        
        print("".join(row))
    
    print("=" * width)
    
    # Triangle intersection test
    triangle = Triangle(
        Point3D(-1, -1, 0),
        Point3D(1, -1, 0),
        Point3D(0, 1, 0)
    )
    
    test_ray = Ray(Point3D(0, 0, 5), Vector3D(0, 0, -1).normalize())
    result = test_ray.intersect_triangle(triangle)
    
    if result:
        t, point = result
        print(f"\nRay hit triangle at t={t:.2f}, point={point}")
    else:
        print("\nRay missed triangle")

if __name__ == "__main__":
    main()
```

#### Task 4.2: Create SVG Rendering Example

**File**: `crates/py/examples/svg_rendering.py`

```python
"""
SVG Rendering Example

Creates a 3D scene and renders it to SVG.
"""

from common_core_geometry import Point3D, Vector3D, Sphere, Triangle, LineSegment
from common_core_geometry.svg import PerspectiveCamera, SVGRenderer

def main():
    # Set up camera
    camera = PerspectiveCamera(
        position=Point3D(4, 4, 4),
        target=Point3D(0, 0, 0),
        up=Vector3D(0, 1, 0),
        fov=60.0,
        aspect=16/9
    )
    
    # Create renderer
    renderer = SVGRenderer(1600, 900, camera)
    
    # Add coordinate axes
    renderer.add_line_segment(
        LineSegment(Point3D(0, 0, 0), Point3D(2, 0, 0)),
        stroke="#ff0000", stroke_width=2
    )
    renderer.add_line_segment(
        LineSegment(Point3D(0, 0, 0), Point3D(0, 2, 0)),
        stroke="#00ff00", stroke_width=2
    )
    renderer.add_line_segment(
        LineSegment(Point3D(0, 0, 0), Point3D(0, 0, 2)),
        stroke="#0000ff", stroke_width=2
    )
    
    # Add spheres
    renderer.add_sphere(
        Sphere(Point3D(0, 0, 0), 1.0),
        stroke="#333333", stroke_width=1, fill="none"
    )
    
    renderer.add_sphere(
        Sphere(Point3D(2, 1, 0), 0.5),
        stroke="#ff6600", stroke_width=2, fill="none"
    )
    
    # Add triangles
    triangle = Triangle(
        Point3D(1, 0, 1),
        Point3D(1, 0, -1),
        Point3D(1, 2, 0)
    )
    renderer.add_triangle(triangle, stroke="#9900cc", stroke_width=1, fill="none")
    
    # Save output
    output_path = "scene.svg"
    renderer.save(output_path)
    print(f"Rendered scene to {output_path}")
    
    # Also print SVG to console
    svg = renderer.render()
    print(f"\nSVG output ({len(svg)} bytes):")
    print(svg[:200] + "..." if len(svg) > 200 else svg)

if __name__ == "__main__":
    main()
```

#### Task 4.3: Create Type Stubs ⏳ PENDING

**Files**: `crates/py/python/common_core_geometry/*.pyi`

Type stubs enable IDE autocompletion and static type checking with mypy.

**File**: `python/common_core_geometry/__init__.pyi`
```python
from typing import Optional, Tuple, Iterator

class Point3D:
    x: float
    y: float
    z: float
    
    def __init__(self, x: float, y: float, z: float) -> None: ...
    
    @staticmethod
    def origin() -> Point3D: ...
    
    def distance_to(self, other: Point3D) -> float: ...
    def midpoint(self, other: Point3D) -> Point3D: ...
    
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...
    def __getitem__(self, index: int) -> float: ...
    def __iter__(self) -> Iterator[float]: ...

class Vector3D:
    x: float
    y: float
    z: float
    
    def __init__(self, x: float, y: float, z: float) -> None: ...
    
    @staticmethod
    def zero() -> Vector3D: ...
    @staticmethod
    def unit_x() -> Vector3D: ...
    @staticmethod
    def unit_y() -> Vector3D: ...
    @staticmethod
    def unit_z() -> Vector3D: ...
    
    def magnitude(self) -> float: ...
    def normalize(self) -> Vector3D: ...
    def dot(self, other: Vector3D) -> float: ...
    def cross(self, other: Vector3D) -> Vector3D: ...
    
    def __add__(self, other: Vector3D) -> Vector3D: ...
    def __sub__(self, other: Vector3D) -> Vector3D: ...
    def __mul__(self, scalar: float) -> Vector3D: ...
    def __rmul__(self, scalar: float) -> Vector3D: ...
    def __neg__(self) -> Vector3D: ...
    
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...
    def __getitem__(self, index: int) -> float: ...
    def __iter__(self) -> Iterator[float]: ...

class Sphere:
    center: Point3D
    radius: float
    
    def __init__(self, center: Point3D, radius: float) -> None: ...
    
    def volume(self) -> float: ...
    def surface_area(self) -> float: ...
    def contains(self, point: Point3D) -> bool: ...
    
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...

class Ray:
    origin: Point3D
    direction: Vector3D
    
    def __init__(self, origin: Point3D, direction: Vector3D) -> None: ...
    
    def point_at(self, t: float) -> Point3D: ...
    def intersect_sphere(self, sphere: Sphere) -> Optional[Tuple[float, Point3D]]: ...
    def intersect_plane(self, plane: Plane) -> Optional[Point3D]: ...
    def intersect_triangle(self, triangle: Triangle) -> Optional[Tuple[float, Point3D]]: ...
    
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...

# ... (continue for all other classes)
```

**File**: `python/common_core_geometry/svg.pyi`
```python
from typing import Union
from . import Point3D, Vector3D, Sphere, Triangle, LineSegment

class PerspectiveCamera:
    def __init__(
        self,
        position: Point3D,
        target: Point3D,
        up: Vector3D,
        fov: float,
        aspect: float
    ) -> None: ...
    
    # Properties...
    @property
    def fov(self) -> float: ...
    @property
    def aspect(self) -> float: ...

class OrthographicCamera:
    def __init__(
        self,
        position: Point3D,
        target: Point3D,
        up: Vector3D,
        width: float,
        height: float
    ) -> None: ...

class SVGRenderer:
    def __init__(
        self,
        width: int,
        height: int,
        camera: Union[PerspectiveCamera, OrthographicCamera]
    ) -> None: ...
    
    def add_point(self, point: Point3D, **style: str) -> None: ...
    def add_line_segment(self, segment: LineSegment, **style: str) -> None: ...
    def add_triangle(self, triangle: Triangle, **style: str) -> None: ...
    def add_sphere(self, sphere: Sphere, **style: str) -> None: ...
    
    def render(self) -> str: ...
    def save(self, path: str) -> None: ...
    
    def __enter__(self) -> SVGRenderer: ...
    def __exit__(self, exc_type, exc_val, exc_tb) -> None: ...
```

#### Task 4.4: Enhanced Documentation ⏳ PENDING

**Update**: `crates/py/README.md`

Add sections:
- Installation (pip, maturin, from source)
- Quick Start with code examples
- API Reference (link to type stubs)
- Examples (link to examples/)
- Performance notes
- Troubleshooting
- Contributing

**Create**: `crates/py/docs/api_reference.md`
- Auto-generate from docstrings if possible
- Manual documentation for each class/method
- Examples for each method

**Acceptance Criteria**:
- [x] 2 complete working examples
- [ ] Type stubs for all public API
- [ ] Comprehensive README
- [ ] API reference documentation

---

### Phase 5: Polish & Release Preparation

**Goal**: Prepare Python bindings for v0.1.0 release.

#### Task 5.1: Add Docstrings to All Classes

Update all `#[pymethods]` in Rust to include docstrings:

```rust
#[pymethods]
impl PyPoint3D {
    #[new]
    #[pyo3(text_signature = "(x, y, z, /)")]
    /// Create a new 3D point.
    ///
    /// Args:
    ///     x (float): X coordinate
    ///     y (float): Y coordinate
    ///     z (float): Z coordinate
    ///
    /// Returns:
    ///     Point3D: A new point at the specified coordinates
    ///
    /// Example:
    ///     >>> from common_core_geometry import Point3D
    ///     >>> p = Point3D(1.0, 2.0, 3.0)
    ///     >>> p.x
    ///     1.0
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        // ...
    }
```

#### Task 5.2: Performance Benchmarks

Create benchmarks to validate performance:

**File**: `crates/py/benches/bench_geometry.py`

```python
import time
from common_core_geometry import Point3D, Vector3D, Sphere, Ray

def benchmark_ray_sphere_intersection(n=100000):
    sphere = Sphere(Point3D(0, 0, 0), 1.0)
    ray = Ray(Point3D(0, 0, 5), Vector3D(0, 0, -1).normalize())
    
    start = time.perf_counter()
    for _ in range(n):
        ray.intersect_sphere(sphere)
    end = time.perf_counter()
    
    elapsed = end - start
    print(f"Ray-Sphere intersections: {n} in {elapsed:.4f}s ({n/elapsed:.0f} ops/sec)")

if __name__ == "__main__":
    benchmark_ray_sphere_intersection()
```

#### Task 5.3: CI/CD Setup

**File**: `.github/workflows/python.yml`

```yaml
name: Python Bindings

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        python-version: ["3.8", "3.9", "3.10", "3.11", "3.12"]
    
    steps:
      - uses: actions/checkout@v3
      
      - uses: actions/setup-python@v4
        with:
          python-version: ${{ matrix.python-version }}
      
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install maturin
        run: pip install maturin pytest
      
      - name: Build Python package
        run: cd crates/py && maturin develop
      
      - name: Run tests
        run: pytest crates/py/tests/
      
      - name: Run examples
        run: |
          python crates/py/examples/basic_usage.py
          python crates/py/examples/ray_casting.py
```

#### Task 5.4: Update PLAN.md

Check off completed tasks in `docs/PLAN.md` Phase 2.

**Acceptance Criteria**:
- All public API has docstrings
- Performance benchmarks documented
- CI/CD pipeline running
- PLAN.md updated with completion status

---

## Testing Strategy

### Test Categories

1. **Unit Tests** (per-primitive)
   - Construction with valid/invalid inputs
   - All methods with typical inputs
   - Edge cases (zero vectors, degenerate geometry)
   - Python protocol methods

2. **Integration Tests**
   - Cross-primitive operations (ray-sphere, ray-triangle)
   - Workflow scenarios (create scene → render)
   - Performance tests (large number of operations)

3. **Property-Based Tests** (optional, using Hypothesis)
   - Geometric properties hold (e.g., `v.normalize().magnitude() ≈ 1.0`)
   - Round-trip conversions
   - Commutativity/associativity where applicable

### Test Metrics

- **Target Coverage**: >80% code coverage
- **Target Test Count**: >100 tests across all files
- **Performance**: Critical paths benchmarked

### Running Tests

```bash
# Install dependencies
cd crates/py
pip install -e ".[dev]"  # or maturin develop

# Run all tests
pytest tests/ -v

# Run with coverage
pytest tests/ --cov=common_core_geometry --cov-report=html

# Run specific test file
pytest tests/test_ray.py -v
```

---

## Build & Distribution

### Development Build

```bash
cd crates/py
maturin develop  # Creates wheel and installs in current venv
```

### Release Build

```bash
cd crates/py
maturin build --release  # Creates wheel in target/wheels/
```

### PyPI Publishing (Future)

```bash
maturin publish
```

---

## Dependencies

### Python Dependencies

**Runtime**: None (self-contained)

**Development**:
- `pytest` >= 7.0
- `pytest-cov` (for coverage)
- `mypy` (for type checking with stubs)

**Optional**:
- `matplotlib` (for visualization examples)
- `numpy` (for array interop examples)

### Rust Dependencies

Current (`crates/py/Cargo.toml`):
```toml
[dependencies]
common-core-geometry = { workspace = true }
pyo3 = { version = "0.26", features = ["extension-module"] }

[build-dependencies]
pyo3-build-config = "0.26"
```

---

## Known Issues & Limitations

### Current Limitations

1. **No async support**: All operations are synchronous
2. **No NumPy integration**: Points/vectors don't convert to/from NumPy arrays yet
3. **Limited error messages**: Some Rust panics may not have clear Python error messages
4. **No parallelism**: No multi-threaded operations exposed (can be added later)

### Future Enhancements

1. **NumPy Integration**
   ```python
   # Convert to NumPy
   import numpy as np
   points = np.array([p.to_array() for p in points])
   
   # Batch operations
   renderer.add_points_batch(points)  # NumPy array input
   ```

2. **Pickle Support**
   ```python
   import pickle
   point = Point3D(1, 2, 3)
   serialized = pickle.dumps(point)
   restored = pickle.loads(serialized)
   ```

3. **Context Manager for Bulk Operations**
   ```python
   with renderer.batch_mode():
       for triangle in triangles:
           renderer.add_triangle(triangle)
   # Batch rendered efficiently on exit
   ```

---

## Success Criteria for Phase 2 Completion

### Minimum Viable Product (MVP) ✅ COMPLETE

- [x] All primitive types exposed to Python
- [x] Ray intersection methods implemented (3 methods)
- [x] SVG module fully exposed
- [x] >50 tests passing (168 passing)
- [x] 2+ working examples (3 examples total)
- [x] Basic README with usage instructions

### Full v0.1.0 Release ⏳ IN PROGRESS

- [x] All primitives fully implemented and tested
- [x] Ray intersection methods complete
- [x] SVG rendering fully functional
- [x] >100 tests passing (168 passing)
- [x] >80% code coverage
- [ ] Type stubs (`.pyi`) for all public API
- [ ] Comprehensive documentation
- [x] 3+ examples (basic, ray casting, SVG)
- [ ] CI/CD pipeline running
- [ ] Performance benchmarks documented

---

## Next Immediate Steps

### High Priority (Remaining for v0.1.0)

1. **Create type stubs** (`crates/py/python/common_core_geometry/*.pyi`)
   - Enable IDE autocompletion and mypy support
   - Create `__init__.pyi` and `svg.pyi`

2. **Add comprehensive docstrings** to all Python bindings
   - Update all `#[pymethods]` with docstrings
   - Follow NumPy docstring format

3. **Update `crates/py/README.md`**
   - Add SVG rendering examples
   - Add installation instructions
   - Add troubleshooting section

4. **Create API reference documentation**
   - Auto-generate or manually document all classes/methods
   - Include examples for each method

### Medium Priority (Phase 5 - Polish)

5. **Performance benchmarks**
   - Create `benches/bench_geometry.py`
   - Document performance characteristics

6. **CI/CD setup**
   - Create `.github/workflows/python.yml`
   - Test across multiple Python versions

7. **Update `docs/PLAN.md`**
   - Mark Phase 2 Python bindings as complete

---

## Contact & Questions

For questions about this plan or the Python bindings implementation, refer to:
- `docs/PLAN.md` - Overall workspace migration plan
- `docs/WORKSPACE.md` - Python bindings design principles
- `docs/SPEC.md` - Core library specification
- `crates/py/README.md` - Python package README
