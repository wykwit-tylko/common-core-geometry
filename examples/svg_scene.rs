use common_core_geometry::{Camera, Point3D, SVGRenderer, Sphere, Triangle, Vector3D, AABB};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating 3D scene as SVG...\n");

    let camera = Camera::perspective(
        Point3D::new(15.0, 12.0, 15.0),
        Point3D::new(0.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
        60.0,
        16.0 / 9.0,
        0.1,
        100.0,
    );

    let mut renderer = SVGRenderer::new(1920, 1080, camera);
    renderer.set_background("#0a0a0a");

    println!("Creating scene geometry:");

    let center_sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 2.5)?;
    renderer.add_sphere(&center_sphere, "#ff6b6b", 3.0);
    println!("  ✓ Added center sphere (red)");

    let left_sphere = Sphere::new(Point3D::new(-5.0, 0.0, 0.0), 1.5)?;
    renderer.add_sphere(&left_sphere, "#4ecdc4", 2.5);
    println!("  ✓ Added left sphere (cyan)");

    let right_sphere = Sphere::new(Point3D::new(5.0, 0.0, 0.0), 1.5)?;
    renderer.add_sphere(&right_sphere, "#ffe66d", 2.5);
    println!("  ✓ Added right sphere (yellow)");

    let top_sphere = Sphere::new(Point3D::new(0.0, 4.0, 0.0), 1.0)?;
    renderer.add_sphere(&top_sphere, "#a8e6cf", 2.0);
    println!("  ✓ Added top sphere (green)");

    let floor = AABB::new(
        Point3D::new(-10.0, -3.0, -10.0),
        Point3D::new(10.0, -2.5, 10.0),
    )?;
    renderer.add_aabb(&floor, "#95a5a6", 1.5);
    println!("  ✓ Added floor (gray)");

    let bounding_box = AABB::new(Point3D::new(-3.0, -1.0, -3.0), Point3D::new(3.0, 2.0, 3.0))?;
    renderer.add_aabb(&bounding_box, "#3498db", 1.0);
    println!("  ✓ Added bounding box (blue)");

    let back_triangle = Triangle::new(
        Point3D::new(-4.0, 0.0, -8.0),
        Point3D::new(4.0, 0.0, -8.0),
        Point3D::new(0.0, 6.0, -8.0),
    )?;
    renderer.add_triangle(&back_triangle, "#e056fd", Some("#e056fd33"), 2.0);
    println!("  ✓ Added background triangle (purple)");

    let front_triangle = Triangle::new(
        Point3D::new(-2.0, -2.0, 5.0),
        Point3D::new(2.0, -2.0, 5.0),
        Point3D::new(0.0, 1.0, 5.0),
    )?;
    renderer.add_triangle(&front_triangle, "#ff9ff3", None, 2.0);
    println!("  ✓ Added foreground triangle (pink)");

    let points = vec![
        (Point3D::new(0.0, 0.0, 0.0), "#ffffff", 8.0),
        (Point3D::new(3.0, 3.0, 3.0), "#ffff00", 6.0),
        (Point3D::new(-3.0, 3.0, 3.0), "#00ffff", 6.0),
    ];

    for (point, color, size) in points {
        renderer.add_point(&point, color, size);
    }
    println!("  ✓ Added reference points");

    println!("\nRendering scene to SVG...");
    renderer.to_file("scene.svg")?;

    println!("\n✅ Scene successfully saved to 'scene.svg'");
    println!("   Open the file in a web browser to view the rendered 3D scene!");

    let svg_content = renderer.to_svg_string();
    println!("\nScene statistics:");
    println!("  - SVG size: {} bytes", svg_content.len());
    println!("  - Canvas: 1920x1080 pixels");
    println!("  - Camera: Perspective (60° FOV)");
    println!("  - Primitives: 8 spheres, 3 AABBs, 2 triangles, 3 points");

    Ok(())
}
