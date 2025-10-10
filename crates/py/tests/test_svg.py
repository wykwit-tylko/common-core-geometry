import pytest
import os
import tempfile
from common_core_geometry import Point3D, Vector3D, LineSegment, Triangle, Sphere, AABB, Camera, SVGRenderer


class TestCamera:
    def test_perspective_construction(self):
        position = Point3D(0.0, 0.0, 5.0)
        target = Point3D(0.0, 0.0, 0.0)
        up = Vector3D(0.0, 1.0, 0.0)
        
        camera = Camera.perspective(position, target, up, 60.0, 1.5)
        assert camera is not None
        assert "Camera" in repr(camera)
    
    def test_perspective_with_near_far(self):
        position = Point3D(0.0, 0.0, 5.0)
        target = Point3D(0.0, 0.0, 0.0)
        up = Vector3D(0.0, 1.0, 0.0)
        
        camera = Camera.perspective(position, target, up, 60.0, 1.5, near=0.5, far=200.0)
        assert camera is not None
    
    def test_orthographic_construction(self):
        position = Point3D(0.0, 0.0, 5.0)
        target = Point3D(0.0, 0.0, 0.0)
        up = Vector3D(0.0, 1.0, 0.0)
        
        camera = Camera.orthographic(position, target, up, 10.0, 10.0)
        assert camera is not None
        assert "Camera" in repr(camera)


class TestSVGRenderer:
    def test_basic_construction(self):
        camera = Camera.perspective(
            Point3D(0.0, 0.0, 5.0),
            Point3D(0.0, 0.0, 0.0),
            Vector3D(0.0, 1.0, 0.0),
            60.0,
            1.0
        )
        renderer = SVGRenderer(800, 600, camera)
        assert renderer is not None
        assert "SVGRenderer" in repr(renderer)
    
    def test_set_background(self):
        camera = Camera.perspective(
            Point3D(0.0, 0.0, 5.0),
            Point3D(0.0, 0.0, 0.0),
            Vector3D(0.0, 1.0, 0.0),
            60.0,
            1.0
        )
        renderer = SVGRenderer(800, 600, camera)
        renderer.set_background("#ffffff")
    
    def test_add_point(self):
        camera = Camera.perspective(
            Point3D(0.0, 0.0, 5.0),
            Point3D(0.0, 0.0, 0.0),
            Vector3D(0.0, 1.0, 0.0),
            60.0,
            1.0
        )
        renderer = SVGRenderer(800, 600, camera)
        point = Point3D(0.0, 0.0, 0.0)
        renderer.add_point(point, color="#ff0000", size=5.0)
    
    def test_add_line_segment(self):
        camera = Camera.perspective(
            Point3D(0.0, 0.0, 5.0),
            Point3D(0.0, 0.0, 0.0),
            Vector3D(0.0, 1.0, 0.0),
            60.0,
            1.0
        )
        renderer = SVGRenderer(800, 600, camera)
        segment = LineSegment(Point3D(0.0, 0.0, 0.0), Point3D(1.0, 1.0, 1.0))
        renderer.add_line_segment(segment, color="#00ff00", width=2.0)
    
    def test_add_triangle(self):
        camera = Camera.perspective(
            Point3D(0.0, 0.0, 5.0),
            Point3D(0.0, 0.0, 0.0),
            Vector3D(0.0, 1.0, 0.0),
            60.0,
            1.0
        )
        renderer = SVGRenderer(800, 600, camera)
        triangle = Triangle(
            Point3D(0.0, 0.0, 0.0),
            Point3D(1.0, 0.0, 0.0),
            Point3D(0.5, 1.0, 0.0)
        )
        renderer.add_triangle(triangle, stroke="#0000ff", fill="#ccccff", width=1.5)
    
    def test_add_sphere(self):
        camera = Camera.perspective(
            Point3D(0.0, 0.0, 5.0),
            Point3D(0.0, 0.0, 0.0),
            Vector3D(0.0, 1.0, 0.0),
            60.0,
            1.0
        )
        renderer = SVGRenderer(800, 600, camera)
        sphere = Sphere(Point3D(0.0, 0.0, 0.0), 1.0)
        renderer.add_sphere(sphere, color="#ff00ff", width=2.0)
    
    def test_add_aabb(self):
        camera = Camera.perspective(
            Point3D(0.0, 0.0, 5.0),
            Point3D(0.0, 0.0, 0.0),
            Vector3D(0.0, 1.0, 0.0),
            60.0,
            1.0
        )
        renderer = SVGRenderer(800, 600, camera)
        aabb = AABB(Point3D(-1.0, -1.0, -1.0), Point3D(1.0, 1.0, 1.0))
        renderer.add_aabb(aabb, color="#00ffff", width=1.0)
    
    def test_render_produces_svg(self):
        camera = Camera.perspective(
            Point3D(0.0, 0.0, 5.0),
            Point3D(0.0, 0.0, 0.0),
            Vector3D(0.0, 1.0, 0.0),
            60.0,
            1.0
        )
        renderer = SVGRenderer(800, 600, camera)
        renderer.add_point(Point3D(0.0, 0.0, 0.0))
        
        svg = renderer.render()
        assert svg.startswith('<svg')
        assert 'width="800"' in svg
        assert 'height="600"' in svg
        assert '</svg>' in svg
    
    def test_save_to_file(self):
        camera = Camera.perspective(
            Point3D(0.0, 0.0, 5.0),
            Point3D(0.0, 0.0, 0.0),
            Vector3D(0.0, 1.0, 0.0),
            60.0,
            1.0
        )
        renderer = SVGRenderer(800, 600, camera)
        renderer.add_point(Point3D(0.0, 0.0, 0.0))
        
        with tempfile.NamedTemporaryFile(mode='w', suffix='.svg', delete=False) as f:
            temp_path = f.name
        
        try:
            renderer.save(temp_path)
            assert os.path.exists(temp_path)
            
            with open(temp_path, 'r') as f:
                content = f.read()
                assert content.startswith('<svg')
                assert 'width="800"' in content
                assert '</svg>' in content
        finally:
            if os.path.exists(temp_path):
                os.remove(temp_path)
    
    def test_context_manager(self):
        camera = Camera.perspective(
            Point3D(0.0, 0.0, 5.0),
            Point3D(0.0, 0.0, 0.0),
            Vector3D(0.0, 1.0, 0.0),
            60.0,
            1.0
        )
        
        with SVGRenderer(800, 600, camera) as renderer:
            renderer.add_point(Point3D(0.0, 0.0, 0.0))
            svg = renderer.render()
            assert '<svg' in svg
    
    def test_complex_scene(self):
        camera = Camera.perspective(
            Point3D(5.0, 5.0, 5.0),
            Point3D(0.0, 0.0, 0.0),
            Vector3D(0.0, 1.0, 0.0),
            60.0,
            1.5
        )
        renderer = SVGRenderer(1200, 800, camera)
        renderer.set_background("#f0f0f0")
        
        renderer.add_point(Point3D(0.0, 0.0, 0.0), color="#000000", size=5.0)
        
        renderer.add_line_segment(
            LineSegment(Point3D(0.0, 0.0, 0.0), Point3D(2.0, 0.0, 0.0)),
            color="#ff0000",
            width=2.0
        )
        
        renderer.add_triangle(
            Triangle(
                Point3D(0.0, 0.0, 0.0),
                Point3D(1.0, 0.0, 0.0),
                Point3D(0.5, 1.0, 0.0)
            ),
            stroke="#0000ff",
            fill="#ccccff"
        )
        
        renderer.add_sphere(
            Sphere(Point3D(-1.5, 0.0, 0.0), 0.5),
            color="#00ff00"
        )
        
        renderer.add_aabb(
            AABB(Point3D(-0.5, -0.5, -0.5), Point3D(0.5, 0.5, 0.5)),
            color="#ff00ff"
        )
        
        svg = renderer.render()
        assert '<svg' in svg
        assert 'width="1200"' in svg
        assert 'height="800"' in svg
