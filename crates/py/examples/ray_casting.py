from common_core_geometry import Point3D, Vector3D, Sphere, Ray, Triangle

def main():
    sphere = Sphere(Point3D(0, 0, 0), 1.0)
    
    width, height = 80, 40
    
    print("Ray-cast sphere rendering:")
    print("=" * width)
    
    for y in range(height):
        row = []
        for x in range(width):
            world_x = (x / width) * 4 - 2
            world_y = (y / height) * 4 - 2
            
            origin = Point3D(world_x, world_y, 5)
            direction = Vector3D(0, 0, -1).normalize()
            ray = Ray(origin, direction)
            
            result = ray.intersect_sphere(sphere)
            if result is not None:
                row.append("█")
            else:
                row.append(" ")
        
        print("".join(row))
    
    print("=" * width)
    
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
