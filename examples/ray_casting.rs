use common_core_geometry::operations::ray_sphere_intersection;
use common_core_geometry::{Point3D, Ray, Sphere, Vector3D};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Ray Casting Example ===\n");

    let ray_origin = Point3D::new(0.0, 0.0, 10.0);
    let ray_direction = Vector3D::new(0.0, 0.0, -1.0);
    let ray = Ray::new(ray_origin, ray_direction)?;

    println!("Ray:");
    println!(
        "  Origin: ({}, {}, {})",
        ray.origin.x, ray.origin.y, ray.origin.z
    );
    println!(
        "  Direction: ({}, {}, {})\n",
        ray.direction.x, ray.direction.y, ray.direction.z
    );

    let spheres = vec![
        (
            Sphere::new(Point3D::new(0.0, 0.0, 0.0), 2.0)?,
            "Center Sphere",
            "red",
        ),
        (
            Sphere::new(Point3D::new(5.0, 0.0, 0.0), 1.5)?,
            "Right Sphere",
            "blue",
        ),
        (
            Sphere::new(Point3D::new(-5.0, 0.0, 0.0), 1.5)?,
            "Left Sphere",
            "green",
        ),
        (
            Sphere::new(Point3D::new(0.0, 5.0, 0.0), 1.0)?,
            "Top Sphere",
            "yellow",
        ),
    ];

    println!("Testing ray intersections:\n");

    for (sphere, name, _color) in &spheres {
        print!("  {}: ", name);
        match ray_sphere_intersection(&ray, sphere) {
            Some((t1, t2)) => {
                let hit1 = ray.point_at(t1);
                let hit2 = ray.point_at(t2);
                println!("HIT!");
                println!(
                    "    First intersection at t={:.2} -> ({:.2}, {:.2}, {:.2})",
                    t1, hit1.x, hit1.y, hit1.z
                );
                println!(
                    "    Second intersection at t={:.2} -> ({:.2}, {:.2}, {:.2})",
                    t2, hit2.x, hit2.y, hit2.z
                );
            }
            None => {
                println!("MISS");
            }
        }
    }

    println!("\n=== Finding closest hit ===\n");

    let mut closest_hit: Option<(f64, &str)> = None;

    for (sphere, name, _color) in &spheres {
        if let Some((t1, _t2)) = ray_sphere_intersection(&ray, sphere) {
            if t1 > 0.0 {
                match closest_hit {
                    None => closest_hit = Some((t1, name)),
                    Some((current_t, _)) => {
                        if t1 < current_t {
                            closest_hit = Some((t1, name));
                        }
                    }
                }
            }
        }
    }

    match closest_hit {
        Some((t, name)) => {
            let hit_point = ray.point_at(t);
            println!("Closest hit: {} at t={:.2}", name, t);
            println!(
                "Hit point: ({:.2}, {:.2}, {:.2})",
                hit_point.x, hit_point.y, hit_point.z
            );
        }
        None => {
            println!("No hits detected!");
        }
    }

    Ok(())
}
