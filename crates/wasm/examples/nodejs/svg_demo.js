import {
    AABB,
    Camera,
    init,
    LineSegment,
    Point3D,
    Sphere,
    SVGRenderer,
    Triangle,
    Vector3D,
} from "../../pkg-nodejs/common_core_geometry_wasm.js";
import fs from "fs";

init();

const camera = Camera.perspective(
    new Point3D(5, 5, 5),
    new Point3D(0, 0, 0),
    new Vector3D(0, 1, 0),
    60,
    1,
    0.1,
    100,
);

const renderer = new SVGRenderer(800, 600, camera);

renderer.setBackground("#f0f0f0");

const origin = new Point3D(0, 0, 0);
renderer.addPoint(origin, "#000000", 5);

const xAxis = new LineSegment(new Point3D(0, 0, 0), new Point3D(2, 0, 0));
renderer.addLineSegment(xAxis, "#ff0000", 2);

const yAxis = new LineSegment(new Point3D(0, 0, 0), new Point3D(0, 2, 0));
renderer.addLineSegment(yAxis, "#00ff00", 2);

const zAxis = new LineSegment(new Point3D(0, 0, 0), new Point3D(0, 0, 2));
renderer.addLineSegment(zAxis, "#0000ff", 2);

const triangle = new Triangle(
    new Point3D(1, 0, 0),
    new Point3D(0, 1, 0),
    new Point3D(0, 0, 1),
);
renderer.addTriangle(triangle, "#ff00ff", "#ffccff", 1.5);

const sphere = new Sphere(new Point3D(-1, 1, 0), 0.5);
renderer.addSphere(sphere, "#00ffff", 2);

const aabb = new AABB(
    new Point3D(-0.5, -0.5, -0.5),
    new Point3D(0.5, 0.5, 0.5),
);
renderer.addAabb(aabb, "#888888", 1);

const svgString = renderer.toSvgString();

fs.writeFileSync("output.svg", svgString);
console.log("SVG written to output.svg");
console.log("\nSVG Preview (first 500 chars):");
console.log(svgString.substring(0, 500) + "...");
