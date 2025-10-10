use common_core_geometry::operations::{
    ray_aabb_intersection, ray_sphere_intersection, ray_triangle_intersection, Transformable,
};
use common_core_geometry::{Camera, Point3D, Ray, SVGRenderer, Sphere, Triangle, Vector3D, AABB};

#[test]
fn test_scene_with_multiple_primitives() {
    let sphere1 = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 2.0).unwrap();
    let sphere2 = Sphere::new(Point3D::new(5.0, 0.0, 0.0), 1.5).unwrap();

    let aabb = AABB::new(
        Point3D::new(-3.0, -3.0, -3.0),
        Point3D::new(-1.0, -1.0, -1.0),
    )
    .unwrap();

    let triangle = Triangle::new(
        Point3D::new(0.0, 3.0, 0.0),
        Point3D::new(1.0, 3.0, 0.0),
        Point3D::new(0.5, 4.0, 0.0),
    )
    .unwrap();

    assert_eq!(sphere1.center, Point3D::new(0.0, 0.0, 0.0));
    assert_eq!(sphere2.center, Point3D::new(5.0, 0.0, 0.0));
    assert_eq!(aabb.min, Point3D::new(-3.0, -3.0, -3.0));
    assert_eq!(triangle.a, Point3D::new(0.0, 3.0, 0.0));
}

#[test]
fn test_ray_casting_through_scene() {
    let origin = Point3D::new(0.0, 0.0, 10.0);
    let direction = Vector3D::new(0.0, 0.0, -1.0);
    let ray = Ray::new(origin, direction).unwrap();

    let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 2.0).unwrap();
    let aabb = AABB::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0)).unwrap();
    let triangle = Triangle::new(
        Point3D::new(-2.0, -2.0, 5.0),
        Point3D::new(2.0, -2.0, 5.0),
        Point3D::new(0.0, 2.0, 5.0),
    )
    .unwrap();

    let sphere_hit = ray_sphere_intersection(&ray, &sphere);
    assert!(sphere_hit.is_some());
    let (t1, t2) = sphere_hit.unwrap();
    assert!(t1 > 0.0 && t2 > 0.0);
    assert!(t1 < 10.0);

    let aabb_hit = ray_aabb_intersection(&ray, &aabb);
    assert!(aabb_hit.is_some());
    let (t_min, t_max) = aabb_hit.unwrap();
    assert!(t_min > 0.0 && t_max > 0.0);

    let triangle_hit = ray_triangle_intersection(&ray, &triangle);
    assert!(triangle_hit.is_some());
    let t = triangle_hit.unwrap();
    assert!(t > 0.0);
    assert_eq!(t, 5.0);
}

#[test]
fn test_ray_casting_misses() {
    let origin = Point3D::new(10.0, 10.0, 10.0);
    let direction = Vector3D::new(1.0, 0.0, 0.0);
    let ray = Ray::new(origin, direction).unwrap();

    let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0).unwrap();
    let aabb = AABB::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0)).unwrap();

    assert!(ray_sphere_intersection(&ray, &sphere).is_none());
    assert!(ray_aabb_intersection(&ray, &aabb).is_none());
}

#[test]
fn test_transformations_in_scene() {
    let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0).unwrap();
    let offset = Vector3D::new(5.0, 3.0, 2.0);
    let translated = sphere.translate(&offset);

    assert_eq!(translated.center, Point3D::new(5.0, 3.0, 2.0));
    assert_eq!(translated.radius, 1.0);

    let center = Point3D::new(0.0, 0.0, 0.0);
    let scaled = sphere.scale(&center, 3.0);
    assert_eq!(scaled.center, Point3D::new(0.0, 0.0, 0.0));
    assert_eq!(scaled.radius, 3.0);

    let aabb = AABB::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(2.0, 2.0, 2.0)).unwrap();
    let scaled_aabb = aabb.scale(&center, 2.0);

    assert_eq!(scaled_aabb.min, Point3D::new(0.0, 0.0, 0.0));
    assert_eq!(scaled_aabb.max, Point3D::new(4.0, 4.0, 4.0));
}

#[test]
fn test_svg_output_generation() {
    let camera = Camera::perspective(
        Point3D::new(10.0, 10.0, 10.0),
        Point3D::new(0.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
        60.0,
        800.0 / 600.0,
        0.1,
        100.0,
    );

    let mut renderer = SVGRenderer::new(800, 600, camera);
    renderer.set_background("white");

    let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 2.0).unwrap();
    renderer.add_sphere(&sphere, "blue", 2.0);

    let aabb = AABB::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0)).unwrap();
    renderer.add_aabb(&aabb, "red", 1.0);

    let svg_output = renderer.to_svg_string();

    assert!(svg_output.contains("<svg"));
    assert!(svg_output.contains("</svg>"));
    assert!(svg_output.contains("width=\"800\""));
    assert!(svg_output.contains("height=\"600\""));
    assert!(svg_output.contains("fill=\"white\""));
    assert!(svg_output.contains("stroke=\"blue\""));
    assert!(svg_output.contains("stroke=\"red\""));
}

#[test]
fn test_camera_projections_perspective() {
    let camera = Camera::perspective(
        Point3D::new(0.0, 0.0, 10.0),
        Point3D::new(0.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
        90.0,
        1.0,
        0.1,
        100.0,
    );

    let mut renderer = SVGRenderer::new(800, 800, camera);

    let point1 = Point3D::new(0.0, 0.0, 0.0);
    renderer.add_point(&point1, "red", 5.0);

    let point2 = Point3D::new(1.0, 1.0, 1.0);
    renderer.add_point(&point2, "green", 5.0);

    let svg = renderer.to_svg_string();
    assert!(svg.contains("fill=\"red\""));
    assert!(svg.contains("fill=\"green\""));
}

#[test]
fn test_camera_projections_orthographic() {
    let camera = Camera::orthographic(
        Point3D::new(0.0, 0.0, 10.0),
        Point3D::new(0.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
        10.0,
        10.0,
    );

    let mut renderer = SVGRenderer::new(800, 800, camera);

    let point = Point3D::new(0.0, 0.0, 0.0);
    renderer.add_point(&point, "blue", 5.0);

    let svg = renderer.to_svg_string();
    assert!(svg.contains("fill=\"blue\""));
    assert!(svg.contains("<svg"));
}

#[test]
fn test_complex_scene_workflow() {
    let camera = Camera::perspective(
        Point3D::new(15.0, 15.0, 15.0),
        Point3D::new(0.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
        60.0,
        16.0 / 9.0,
        0.1,
        100.0,
    );

    let mut renderer = SVGRenderer::new(1920, 1080, camera);
    renderer.set_background("#f0f0f0");

    let sphere1 = Sphere::new(Point3D::new(-3.0, 0.0, 0.0), 1.5).unwrap();
    let sphere2 = Sphere::new(Point3D::new(3.0, 0.0, 0.0), 1.5).unwrap();
    let sphere3 = Sphere::new(Point3D::new(0.0, 3.0, 0.0), 1.0).unwrap();

    renderer.add_sphere(&sphere1, "#ff6b6b", 2.0);
    renderer.add_sphere(&sphere2, "#4ecdc4", 2.0);
    renderer.add_sphere(&sphere3, "#ffe66d", 2.0);

    let floor = AABB::new(
        Point3D::new(-10.0, -2.0, -10.0),
        Point3D::new(10.0, -1.9, 10.0),
    )
    .unwrap();
    renderer.add_aabb(&floor, "#95a5a6", 1.0);

    let triangle = Triangle::new(
        Point3D::new(0.0, 0.0, -5.0),
        Point3D::new(2.0, 0.0, -5.0),
        Point3D::new(1.0, 2.0, -5.0),
    )
    .unwrap();
    renderer.add_triangle(&triangle, "#e056fd", Some("#e056fd55"), 2.0);

    let ray_origin = Point3D::new(0.0, 10.0, 0.0);
    let ray_direction = Vector3D::new(0.0, -1.0, 0.0);
    let ray = Ray::new(ray_origin, ray_direction).unwrap();

    let hit_sphere3 = ray_sphere_intersection(&ray, &sphere3);
    assert!(hit_sphere3.is_some());

    let svg = renderer.to_svg_string();
    assert!(svg.len() > 500);
    assert!(svg.contains("svg"));
    assert!(svg.contains("#ff6b6b"));
    assert!(svg.contains("#4ecdc4"));
    assert!(svg.contains("#ffe66d"));
}

#[test]
fn test_scene_with_transformations_and_rendering() {
    let camera = Camera::perspective(
        Point3D::new(8.0, 8.0, 8.0),
        Point3D::new(0.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
        60.0,
        1.0,
        0.1,
        50.0,
    );

    let mut renderer = SVGRenderer::new(800, 800, camera);

    let base_sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0).unwrap();

    let offset1 = Vector3D::new(3.0, 0.0, 0.0);
    let sphere1 = base_sphere.translate(&offset1);
    renderer.add_sphere(&sphere1, "red", 2.0);

    let offset2 = Vector3D::new(-3.0, 0.0, 0.0);
    let sphere2 = base_sphere.translate(&offset2);
    renderer.add_sphere(&sphere2, "blue", 2.0);

    let center = Point3D::new(0.0, 0.0, 0.0);
    let sphere3 = base_sphere.scale(&center, 2.0);
    renderer.add_sphere(&sphere3, "green", 3.0);

    let svg = renderer.to_svg_string();
    assert!(svg.contains("red"));
    assert!(svg.contains("blue"));
    assert!(svg.contains("green"));

    assert_eq!(sphere1.center, Point3D::new(3.0, 0.0, 0.0));
    assert_eq!(sphere2.center, Point3D::new(-3.0, 0.0, 0.0));
    assert_eq!(sphere3.radius, 2.0);
}
