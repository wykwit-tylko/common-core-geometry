/* tslint:disable */
/* eslint-disable */
export function init(): void;
export class AABB {
  free(): void;
  [Symbol.dispose](): void;
  constructor(min: Point3D, max: Point3D);
  static fromPoints(points: Point3D[]): AABB;
  center(): Point3D;
  size(): Vector3D;
  volume(): number;
  surfaceArea(): number;
  diagonal(): number;
  containsPoint(point: Point3D): boolean;
  intersects(other: AABB): boolean;
  union(other: AABB): AABB;
  expandByPoint(point: Point3D): AABB;
  expandByScalar(amount: number): AABB;
  readonly min: Point3D;
  readonly max: Point3D;
}
export class Camera {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static perspective(position: Point3D, target: Point3D, up: Vector3D, fov: number, aspect: number, near: number, far: number): Camera;
  static orthographic(position: Point3D, target: Point3D, up: Vector3D, width: number, height: number): Camera;
  viewMatrix(): Float64Array;
  projectionMatrix(): Float64Array;
}
export class LineSegment {
  free(): void;
  [Symbol.dispose](): void;
  constructor(start: Point3D, end: Point3D);
  length(): number;
  direction(): Vector3D;
  midpoint(): Point3D;
  pointAt(t: number): Point3D;
  closestPoint(point: Point3D): Point3D;
  distanceToPoint(point: Point3D): number;
  readonly start: Point3D;
  readonly end: Point3D;
}
export class Plane {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static fromPointNormal(point: Point3D, normal: Vector3D): Plane;
  static fromThreePoints(p1: Point3D, p2: Point3D, p3: Point3D): Plane;
  distanceToPoint(point: Point3D): number;
  closestPoint(point: Point3D): Point3D;
  containsPoint(point: Point3D): boolean;
  flipNormal(): Plane;
  isParallel(other: Plane): boolean;
  readonly normal: Vector3D;
  readonly d: number;
}
export class Point3D {
  free(): void;
  [Symbol.dispose](): void;
  constructor(x: number, y: number, z: number);
  distanceTo(other: Point3D): number;
  midpoint(other: Point3D): Point3D;
  translate(vector: Vector3D): Point3D;
  toArray(): Float64Array;
  static fromArray(arr: Float64Array): Point3D;
  static origin(): Point3D;
  readonly x: number;
  readonly y: number;
  readonly z: number;
}
export class Ray {
  free(): void;
  [Symbol.dispose](): void;
  constructor(origin: Point3D, direction: Vector3D);
  pointAt(t: number): Point3D;
  intersectSphere(sphere: Sphere): any;
  intersectPlane(plane: Plane): any;
  intersectTriangle(triangle: Triangle): any;
  readonly origin: Point3D;
  readonly direction: Vector3D;
}
export class SVGRenderer {
  free(): void;
  [Symbol.dispose](): void;
  constructor(width: number, height: number, camera: Camera);
  setBackground(color: string): void;
  addPoint(point: Point3D, color: string, size: number): void;
  addLineSegment(segment: LineSegment, color: string, width: number): void;
  addTriangle(triangle: Triangle, stroke: string, fill: string | null | undefined, width: number): void;
  addSphere(sphere: Sphere, color: string, width: number): void;
  addAabb(aabb: AABB, color: string, width: number): void;
  toSvgString(): string;
}
export class Sphere {
  free(): void;
  [Symbol.dispose](): void;
  constructor(center: Point3D, radius: number);
  volume(): number;
  surfaceArea(): number;
  contains(point: Point3D): boolean;
  readonly center: Point3D;
  readonly radius: number;
}
export class Triangle {
  free(): void;
  [Symbol.dispose](): void;
  constructor(a: Point3D, b: Point3D, c: Point3D);
  normal(): Vector3D;
  area(): number;
  centroid(): Point3D;
  barycentricCoords(point: Point3D): Float64Array;
  containsPoint(point: Point3D): boolean;
  readonly a: Point3D;
  readonly b: Point3D;
  readonly c: Point3D;
}
export class Vector3D {
  free(): void;
  [Symbol.dispose](): void;
  constructor(x: number, y: number, z: number);
  magnitude(): number;
  normalize(): Vector3D;
  dot(other: Vector3D): number;
  cross(other: Vector3D): Vector3D;
  add(other: Vector3D): Vector3D;
  sub(other: Vector3D): Vector3D;
  scale(scalar: number): Vector3D;
  angle(other: Vector3D): number;
  projectOnto(other: Vector3D): Vector3D;
  isParallel(other: Vector3D): boolean;
  isPerpendicular(other: Vector3D): boolean;
  toArray(): Float64Array;
  static fromArray(arr: Float64Array): Vector3D;
  static zero(): Vector3D;
  static unitX(): Vector3D;
  static unitY(): Vector3D;
  static unitZ(): Vector3D;
  static fromPoints(from: Point3D, to: Point3D): Vector3D;
  readonly x: number;
  readonly y: number;
  readonly z: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_sphere_free: (a: number, b: number) => void;
  readonly sphere_new: (a: number, b: number) => [number, number, number];
  readonly sphere_center: (a: number) => number;
  readonly sphere_radius: (a: number) => number;
  readonly sphere_volume: (a: number) => number;
  readonly sphere_surfaceArea: (a: number) => number;
  readonly sphere_contains: (a: number, b: number) => number;
  readonly __wbg_svgrenderer_free: (a: number, b: number) => void;
  readonly svgrenderer_new: (a: number, b: number, c: number) => number;
  readonly svgrenderer_setBackground: (a: number, b: number, c: number) => void;
  readonly svgrenderer_addPoint: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly svgrenderer_addLineSegment: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly svgrenderer_addTriangle: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly svgrenderer_addSphere: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly svgrenderer_addAabb: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly svgrenderer_toSvgString: (a: number) => [number, number];
  readonly __wbg_camera_free: (a: number, b: number) => void;
  readonly camera_perspective: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly camera_orthographic: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly camera_viewMatrix: (a: number) => [number, number];
  readonly camera_projectionMatrix: (a: number) => [number, number];
  readonly __wbg_plane_free: (a: number, b: number) => void;
  readonly plane_fromPointNormal: (a: number, b: number) => [number, number, number];
  readonly plane_fromThreePoints: (a: number, b: number, c: number) => [number, number, number];
  readonly plane_normal: (a: number) => number;
  readonly plane_d: (a: number) => number;
  readonly plane_distanceToPoint: (a: number, b: number) => number;
  readonly plane_closestPoint: (a: number, b: number) => number;
  readonly plane_containsPoint: (a: number, b: number) => number;
  readonly plane_flipNormal: (a: number) => number;
  readonly plane_isParallel: (a: number, b: number) => number;
  readonly __wbg_triangle_free: (a: number, b: number) => void;
  readonly triangle_new: (a: number, b: number, c: number) => [number, number, number];
  readonly triangle_b: (a: number) => number;
  readonly triangle_c: (a: number) => number;
  readonly triangle_normal: (a: number) => number;
  readonly triangle_area: (a: number) => number;
  readonly triangle_centroid: (a: number) => number;
  readonly triangle_barycentricCoords: (a: number, b: number) => [number, number];
  readonly triangle_containsPoint: (a: number, b: number) => number;
  readonly triangle_a: (a: number) => number;
  readonly __wbg_linesegment_free: (a: number, b: number) => void;
  readonly linesegment_new: (a: number, b: number) => [number, number, number];
  readonly linesegment_start: (a: number) => number;
  readonly linesegment_end: (a: number) => number;
  readonly linesegment_length: (a: number) => number;
  readonly linesegment_direction: (a: number) => number;
  readonly linesegment_midpoint: (a: number) => number;
  readonly linesegment_pointAt: (a: number, b: number) => number;
  readonly linesegment_closestPoint: (a: number, b: number) => number;
  readonly linesegment_distanceToPoint: (a: number, b: number) => number;
  readonly __wbg_point3d_free: (a: number, b: number) => void;
  readonly point3d_new: (a: number, b: number, c: number) => number;
  readonly point3d_x: (a: number) => number;
  readonly point3d_y: (a: number) => number;
  readonly point3d_z: (a: number) => number;
  readonly point3d_distanceTo: (a: number, b: number) => number;
  readonly point3d_midpoint: (a: number, b: number) => number;
  readonly point3d_translate: (a: number, b: number) => number;
  readonly point3d_toArray: (a: number) => [number, number];
  readonly point3d_fromArray: (a: number, b: number) => [number, number, number];
  readonly point3d_origin: () => number;
  readonly init: () => void;
  readonly __wbg_ray_free: (a: number, b: number) => void;
  readonly ray_new: (a: number, b: number) => [number, number, number];
  readonly ray_origin: (a: number) => number;
  readonly ray_direction: (a: number) => number;
  readonly ray_pointAt: (a: number, b: number) => number;
  readonly ray_intersectSphere: (a: number, b: number) => any;
  readonly ray_intersectPlane: (a: number, b: number) => any;
  readonly ray_intersectTriangle: (a: number, b: number) => any;
  readonly __wbg_vector3d_free: (a: number, b: number) => void;
  readonly vector3d_new: (a: number, b: number, c: number) => number;
  readonly vector3d_x: (a: number) => number;
  readonly vector3d_y: (a: number) => number;
  readonly vector3d_z: (a: number) => number;
  readonly vector3d_magnitude: (a: number) => number;
  readonly vector3d_normalize: (a: number) => [number, number, number];
  readonly vector3d_dot: (a: number, b: number) => number;
  readonly vector3d_cross: (a: number, b: number) => number;
  readonly vector3d_add: (a: number, b: number) => number;
  readonly vector3d_sub: (a: number, b: number) => number;
  readonly vector3d_scale: (a: number, b: number) => number;
  readonly vector3d_angle: (a: number, b: number) => number;
  readonly vector3d_projectOnto: (a: number, b: number) => number;
  readonly vector3d_isParallel: (a: number, b: number) => number;
  readonly vector3d_isPerpendicular: (a: number, b: number) => number;
  readonly vector3d_toArray: (a: number) => [number, number];
  readonly vector3d_fromArray: (a: number, b: number) => [number, number, number];
  readonly vector3d_zero: () => number;
  readonly vector3d_unitX: () => number;
  readonly vector3d_unitY: () => number;
  readonly vector3d_unitZ: () => number;
  readonly vector3d_fromPoints: (a: number, b: number) => number;
  readonly __wbg_aabb_free: (a: number, b: number) => void;
  readonly aabb_new: (a: number, b: number) => [number, number, number];
  readonly aabb_fromPoints: (a: number, b: number) => [number, number, number];
  readonly aabb_min: (a: number) => number;
  readonly aabb_max: (a: number) => number;
  readonly aabb_center: (a: number) => number;
  readonly aabb_size: (a: number) => number;
  readonly aabb_volume: (a: number) => number;
  readonly aabb_surfaceArea: (a: number) => number;
  readonly aabb_diagonal: (a: number) => number;
  readonly aabb_containsPoint: (a: number, b: number) => number;
  readonly aabb_intersects: (a: number, b: number) => number;
  readonly aabb_union: (a: number, b: number) => number;
  readonly aabb_expandByPoint: (a: number, b: number) => number;
  readonly aabb_expandByScalar: (a: number, b: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
