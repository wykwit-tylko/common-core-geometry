import {
  init,
  Point3D,
  Ray,
  Sphere,
  Triangle,
  Vector3D,
} from "../../pkg-nodejs/common_core_geometry_wasm.js";

init();

console.log("=== Common Core Geometry WASM Demo ===\n");

console.log("1. Creating 3D points:");
const origin = Point3D.origin();
const p1 = new Point3D(1, 2, 3);
console.log(`   Origin: (${origin.x}, ${origin.y}, ${origin.z})`);
console.log(`   Point 1: (${p1.x}, ${p1.y}, ${p1.z})`);
console.log(`   Distance: ${origin.distanceTo(p1).toFixed(3)}\n`);

console.log("2. Working with vectors:");
const v1 = new Vector3D(1, 0, 0);
const v2 = new Vector3D(0, 1, 0);
const cross = v1.cross(v2);
console.log(`   v1: (${v1.x}, ${v1.y}, ${v1.z})`);
console.log(`   v2: (${v2.x}, ${v2.y}, ${v2.z})`);
console.log(`   v1 × v2: (${cross.x}, ${cross.y}, ${cross.z})`);
console.log(`   Dot product: ${v1.dot(v2)}\n`);

console.log("3. Ray-sphere intersection:");
const rayOrigin = new Point3D(-5, 0, 0);
const rayDir = new Vector3D(1, 0, 0);
const ray = new Ray(rayOrigin, rayDir);
const sphere = new Sphere(Point3D.origin(), 1.0);
const intersection = ray.intersectSphere(sphere);
if (intersection) {
  console.log(
    `   Ray hits sphere at t1=${intersection.t1.toFixed(3)}, t2=${
      intersection.t2.toFixed(3)
    }`,
  );
  const hitPoint = ray.pointAt(intersection.t1);
  console.log(
    `   First hit point: (${hitPoint.x.toFixed(3)}, ${hitPoint.y.toFixed(3)}, ${
      hitPoint.z.toFixed(3)
    })\n`,
  );
}

console.log("4. Triangle properties:");
const tri = new Triangle(
  new Point3D(0, 0, 0),
  new Point3D(3, 0, 0),
  new Point3D(0, 3, 0),
);
const normal = tri.normal();
const centroid = tri.centroid();
console.log(`   Area: ${tri.area()}`);
console.log(
  `   Normal: (${normal.x.toFixed(3)}, ${normal.y.toFixed(3)}, ${
    normal.z.toFixed(3)
  })`,
);
console.log(`   Centroid: (${centroid.x}, ${centroid.y}, ${centroid.z})\n`);

console.log("5. Sphere properties:");
console.log(`   Volume: ${sphere.volume().toFixed(3)}`);
console.log(`   Surface area: ${sphere.surfaceArea().toFixed(3)}`);
console.log(`   Contains origin: ${sphere.contains(origin)}`);
console.log(`   Contains (2,0,0): ${sphere.contains(new Point3D(2, 0, 0))}\n`);

console.log("Demo complete!");
