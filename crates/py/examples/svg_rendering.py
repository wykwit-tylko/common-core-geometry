from common_core_geometry import Point3D, Vector3D, Sphere, Triangle, LineSegment, AABB, Camera, SVGRenderer

def main():
    camera = Camera.perspective(
        Point3D(4, 4, 4),
        Point3D(0, 0, 0),
        Vector3D(0, 1, 0),
        60.0,
        16/9
    )
    
    renderer = SVGRenderer(1600, 900, camera)
    renderer.set_background("#f5f5f5")
    
    renderer.add_line_segment(
        LineSegment(Point3D(0, 0, 0), Point3D(2, 0, 0)),
        color="#ff0000", width=3
    )
    renderer.add_line_segment(
        LineSegment(Point3D(0, 0, 0), Point3D(0, 2, 0)),
        color="#00ff00", width=3
    )
    renderer.add_line_segment(
        LineSegment(Point3D(0, 0, 0), Point3D(0, 0, 2)),
        color="#0000ff", width=3
    )
    
    renderer.add_sphere(
        Sphere(Point3D(0, 0, 0), 1.0),
        color="#333333", width=2
    )
    
    renderer.add_sphere(
        Sphere(Point3D(2, 1, 0), 0.5),
        color="#ff6600", width=2
    )
    
    triangle = Triangle(
        Point3D(1, 0, 1),
        Point3D(1, 0, -1),
        Point3D(1, 2, 0)
    )
    renderer.add_triangle(triangle, stroke="#9900cc", fill="#cc99ff", width=2)
    
    aabb = AABB(Point3D(-1.5, -0.5, -1.5), Point3D(-0.5, 0.5, -0.5))
    renderer.add_aabb(aabb, color="#00cccc", width=1)
    
    output_path = "scene.svg"
    renderer.save(output_path)
    print(f"Rendered scene to {output_path}")
    
    svg = renderer.render()
    print(f"\nSVG output ({len(svg)} bytes):")
    print(svg[:200] + "..." if len(svg) > 200 else svg)

if __name__ == "__main__":
    main()
