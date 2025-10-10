import { describe, it } from 'bun:test';
import { expect } from 'bun:test';
import {
  Point3D,
  Vector3D,
  Sphere,
  Ray,
  Triangle,
  Plane,
  AABB,
  LineSegment,
} from '../pkg-nodejs/common_core_geometry_wasm.js';

describe('Point3D', () => {
  it('should create a point', () => {
    const p = new Point3D(1.0, 2.0, 3.0);
    expect(p.x).toBe(1.0);
    expect(p.y).toBe(2.0);
    expect(p.z).toBe(3.0);
  });

  it('should calculate distance', () => {
    const p1 = new Point3D(0, 0, 0);
    const p2 = new Point3D(3, 4, 0);
    expect(p1.distanceTo(p2)).toBe(5.0);
  });

  it('should convert to/from array', () => {
    const p = new Point3D(1, 2, 3);
    const arr = p.toArray();
    const p2 = Point3D.fromArray(arr);
    expect(p2.x).toBe(1);
    expect(p2.y).toBe(2);
    expect(p2.z).toBe(3);
  });
});

describe('Vector3D', () => {
  it('should create a vector', () => {
    const v = new Vector3D(1.0, 0.0, 0.0);
    expect(v.x).toBe(1.0);
    expect(v.magnitude()).toBe(1.0);
  });

  it('should compute dot product', () => {
    const v1 = new Vector3D(1, 0, 0);
    const v2 = new Vector3D(0, 1, 0);
    expect(v1.dot(v2)).toBe(0.0);
  });

  it('should compute cross product', () => {
    const v1 = new Vector3D(1, 0, 0);
    const v2 = new Vector3D(0, 1, 0);
    const cross = v1.cross(v2);
    expect(cross.x).toBe(0);
    expect(cross.y).toBe(0);
    expect(cross.z).toBe(1);
  });
});

describe('Sphere', () => {
  it('should create a sphere', () => {
    const center = new Point3D(0, 0, 0);
    const sphere = new Sphere(center, 1.0);
    expect(sphere.radius).toBe(1.0);
  });

  it('should calculate volume', () => {
    const center = new Point3D(0, 0, 0);
    const sphere = new Sphere(center, 1.0);
    const volume = sphere.volume();
    expect(Math.abs(volume - 4.1887902)).toBeLessThan(0.0001);
  });

  it('should test point containment', () => {
    const center = new Point3D(0, 0, 0);
    const sphere = new Sphere(center, 1.0);
    const inside = new Point3D(0.5, 0, 0);
    const outside = new Point3D(2, 0, 0);
    expect(sphere.contains(inside)).toBe(true);
    expect(sphere.contains(outside)).toBe(false);
  });
});

describe('Ray', () => {
  it('should create a ray', () => {
    const origin = new Point3D(0, 0, 0);
    const direction = new Vector3D(1, 0, 0);
    const ray = new Ray(origin, direction);
    expect(ray.origin.x).toBe(0);
    expect(ray.direction.x).toBe(1);
  });

  it('should compute point at parameter t', () => {
    const origin = new Point3D(0, 0, 0);
    const direction = new Vector3D(1, 0, 0);
    const ray = new Ray(origin, direction);
    const p = ray.pointAt(5.0);
    expect(p.x).toBe(5.0);
    expect(p.y).toBe(0.0);
  });

  it('should intersect sphere', () => {
    const origin = new Point3D(-5, 0, 0);
    const direction = new Vector3D(1, 0, 0);
    const ray = new Ray(origin, direction);
    const sphere = new Sphere(new Point3D(0, 0, 0), 1.0);
    const result = ray.intersectSphere(sphere);
    expect(result).not.toBe(null);
    expect(result.t1).toBeGreaterThan(0);
    expect(result.t2).toBeGreaterThan(0);
  });
});

describe('Triangle', () => {
  it('should create a triangle', () => {
    const a = new Point3D(0, 0, 0);
    const b = new Point3D(1, 0, 0);
    const c = new Point3D(0, 1, 0);
    const tri = new Triangle(a, b, c);
    expect(tri.a.x).toBe(0);
  });

  it('should calculate area', () => {
    const a = new Point3D(0, 0, 0);
    const b = new Point3D(2, 0, 0);
    const c = new Point3D(0, 2, 0);
    const tri = new Triangle(a, b, c);
    expect(tri.area()).toBe(2.0);
  });

  it('should calculate centroid', () => {
    const a = new Point3D(0, 0, 0);
    const b = new Point3D(3, 0, 0);
    const c = new Point3D(0, 3, 0);
    const tri = new Triangle(a, b, c);
    const centroid = tri.centroid();
    expect(centroid.x).toBe(1.0);
    expect(centroid.y).toBe(1.0);
  });
});

describe('Plane', () => {
  it('should create from point and normal', () => {
    const point = new Point3D(0, 0, 0);
    const normal = new Vector3D(0, 0, 1);
    const plane = Plane.fromPointNormal(point, normal);
    expect(plane.containsPoint(point)).toBe(true);
  });

  it('should create from three points', () => {
    const p1 = new Point3D(0, 0, 0);
    const p2 = new Point3D(1, 0, 0);
    const p3 = new Point3D(0, 1, 0);
    const plane = Plane.fromThreePoints(p1, p2, p3);
    expect(plane.containsPoint(p1)).toBe(true);
    expect(plane.containsPoint(p2)).toBe(true);
    expect(plane.containsPoint(p3)).toBe(true);
  });

  it('should calculate distance to point', () => {
    const point = new Point3D(0, 0, 0);
    const normal = new Vector3D(0, 0, 1);
    const plane = Plane.fromPointNormal(point, normal);
    const testPoint = new Point3D(0, 0, 5);
    expect(plane.distanceToPoint(testPoint)).toBe(5.0);
  });
});

describe('AABB', () => {
  it('should create an AABB', () => {
    const min = new Point3D(0, 0, 0);
    const max = new Point3D(1, 1, 1);
    const aabb = new AABB(min, max);
    expect(aabb.min.x).toBe(0);
    expect(aabb.max.x).toBe(1);
  });

  it('should calculate volume', () => {
    const min = new Point3D(0, 0, 0);
    const max = new Point3D(2, 3, 4);
    const aabb = new AABB(min, max);
    expect(aabb.volume()).toBe(24.0);
  });

  it('should test point containment', () => {
    const min = new Point3D(0, 0, 0);
    const max = new Point3D(1, 1, 1);
    const aabb = new AABB(min, max);
    const inside = new Point3D(0.5, 0.5, 0.5);
    const outside = new Point3D(2, 0, 0);
    expect(aabb.containsPoint(inside)).toBe(true);
    expect(aabb.containsPoint(outside)).toBe(false);
  });

  it('should test intersection', () => {
    const aabb1 = new AABB(new Point3D(0, 0, 0), new Point3D(1, 1, 1));
    const aabb2 = new AABB(new Point3D(0.5, 0.5, 0.5), new Point3D(1.5, 1.5, 1.5));
    const aabb3 = new AABB(new Point3D(2, 0, 0), new Point3D(3, 1, 1));
    expect(aabb1.intersects(aabb2)).toBe(true);
    expect(aabb1.intersects(aabb3)).toBe(false);
  });
});

describe('LineSegment', () => {
  it('should create a line segment', () => {
    const start = new Point3D(0, 0, 0);
    const end = new Point3D(1, 0, 0);
    const seg = new LineSegment(start, end);
    expect(seg.start.x).toBe(0);
    expect(seg.end.x).toBe(1);
  });

  it('should calculate length', () => {
    const start = new Point3D(0, 0, 0);
    const end = new Point3D(3, 4, 0);
    const seg = new LineSegment(start, end);
    expect(seg.length()).toBe(5.0);
  });

  it('should calculate midpoint', () => {
    const start = new Point3D(0, 0, 0);
    const end = new Point3D(2, 2, 2);
    const seg = new LineSegment(start, end);
    const mid = seg.midpoint();
    expect(mid.x).toBe(1);
    expect(mid.y).toBe(1);
    expect(mid.z).toBe(1);
  });
});
