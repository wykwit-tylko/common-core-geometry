let wasm;

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_3.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    }
}

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_3.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedFloat64ArrayMemory0 = null;

function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
}

function getArrayF64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);
}

function passArrayF64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8, 8) >>> 0;
    getFloat64ArrayMemory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

export function init() {
    wasm.init();
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

const AABBFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_aabb_free(ptr >>> 0, 1));

export class AABB {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(AABB.prototype);
        obj.__wbg_ptr = ptr;
        AABBFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        AABBFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_aabb_free(ptr, 0);
    }
    /**
     * @param {Point3D} min
     * @param {Point3D} max
     */
    constructor(min, max) {
        _assertClass(min, Point3D);
        _assertClass(max, Point3D);
        const ret = wasm.aabb_new(min.__wbg_ptr, max.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        AABBFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {Point3D[]} points
     * @returns {AABB}
     */
    static fromPoints(points) {
        const ptr0 = passArrayJsValueToWasm0(points, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.aabb_fromPoints(ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return AABB.__wrap(ret[0]);
    }
    /**
     * @returns {Point3D}
     */
    get min() {
        const ret = wasm.aabb_min(this.__wbg_ptr);
        return Point3D.__wrap(ret);
    }
    /**
     * @returns {Point3D}
     */
    get max() {
        const ret = wasm.aabb_max(this.__wbg_ptr);
        return Point3D.__wrap(ret);
    }
    /**
     * @returns {Point3D}
     */
    center() {
        const ret = wasm.aabb_center(this.__wbg_ptr);
        return Point3D.__wrap(ret);
    }
    /**
     * @returns {Vector3D}
     */
    size() {
        const ret = wasm.aabb_size(this.__wbg_ptr);
        return Vector3D.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    volume() {
        const ret = wasm.aabb_volume(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    surfaceArea() {
        const ret = wasm.aabb_surfaceArea(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    diagonal() {
        const ret = wasm.aabb_diagonal(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Point3D} point
     * @returns {boolean}
     */
    containsPoint(point) {
        _assertClass(point, Point3D);
        const ret = wasm.aabb_containsPoint(this.__wbg_ptr, point.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {AABB} other
     * @returns {boolean}
     */
    intersects(other) {
        _assertClass(other, AABB);
        const ret = wasm.aabb_intersects(this.__wbg_ptr, other.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {AABB} other
     * @returns {AABB}
     */
    union(other) {
        _assertClass(other, AABB);
        const ret = wasm.aabb_union(this.__wbg_ptr, other.__wbg_ptr);
        return AABB.__wrap(ret);
    }
    /**
     * @param {Point3D} point
     * @returns {AABB}
     */
    expandByPoint(point) {
        _assertClass(point, Point3D);
        const ret = wasm.aabb_expandByPoint(this.__wbg_ptr, point.__wbg_ptr);
        return AABB.__wrap(ret);
    }
    /**
     * @param {number} amount
     * @returns {AABB}
     */
    expandByScalar(amount) {
        const ret = wasm.aabb_expandByScalar(this.__wbg_ptr, amount);
        return AABB.__wrap(ret);
    }
}
if (Symbol.dispose) AABB.prototype[Symbol.dispose] = AABB.prototype.free;

const CameraFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_camera_free(ptr >>> 0, 1));

export class Camera {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Camera.prototype);
        obj.__wbg_ptr = ptr;
        CameraFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CameraFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_camera_free(ptr, 0);
    }
    /**
     * @param {Point3D} position
     * @param {Point3D} target
     * @param {Vector3D} up
     * @param {number} fov
     * @param {number} aspect
     * @param {number} near
     * @param {number} far
     * @returns {Camera}
     */
    static perspective(position, target, up, fov, aspect, near, far) {
        _assertClass(position, Point3D);
        _assertClass(target, Point3D);
        _assertClass(up, Vector3D);
        const ret = wasm.camera_perspective(position.__wbg_ptr, target.__wbg_ptr, up.__wbg_ptr, fov, aspect, near, far);
        return Camera.__wrap(ret);
    }
    /**
     * @param {Point3D} position
     * @param {Point3D} target
     * @param {Vector3D} up
     * @param {number} width
     * @param {number} height
     * @returns {Camera}
     */
    static orthographic(position, target, up, width, height) {
        _assertClass(position, Point3D);
        _assertClass(target, Point3D);
        _assertClass(up, Vector3D);
        const ret = wasm.camera_orthographic(position.__wbg_ptr, target.__wbg_ptr, up.__wbg_ptr, width, height);
        return Camera.__wrap(ret);
    }
    /**
     * @returns {Float64Array}
     */
    viewMatrix() {
        const ret = wasm.camera_viewMatrix(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {Float64Array}
     */
    projectionMatrix() {
        const ret = wasm.camera_projectionMatrix(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
}
if (Symbol.dispose) Camera.prototype[Symbol.dispose] = Camera.prototype.free;

const LineSegmentFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_linesegment_free(ptr >>> 0, 1));

export class LineSegment {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LineSegmentFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_linesegment_free(ptr, 0);
    }
    /**
     * @param {Point3D} start
     * @param {Point3D} end
     */
    constructor(start, end) {
        _assertClass(start, Point3D);
        _assertClass(end, Point3D);
        const ret = wasm.linesegment_new(start.__wbg_ptr, end.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        LineSegmentFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {Point3D}
     */
    get start() {
        const ret = wasm.linesegment_start(this.__wbg_ptr);
        return Point3D.__wrap(ret);
    }
    /**
     * @returns {Point3D}
     */
    get end() {
        const ret = wasm.linesegment_end(this.__wbg_ptr);
        return Point3D.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    length() {
        const ret = wasm.linesegment_length(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Vector3D}
     */
    direction() {
        const ret = wasm.linesegment_direction(this.__wbg_ptr);
        return Vector3D.__wrap(ret);
    }
    /**
     * @returns {Point3D}
     */
    midpoint() {
        const ret = wasm.linesegment_midpoint(this.__wbg_ptr);
        return Point3D.__wrap(ret);
    }
    /**
     * @param {number} t
     * @returns {Point3D}
     */
    pointAt(t) {
        const ret = wasm.linesegment_pointAt(this.__wbg_ptr, t);
        return Point3D.__wrap(ret);
    }
    /**
     * @param {Point3D} point
     * @returns {Point3D}
     */
    closestPoint(point) {
        _assertClass(point, Point3D);
        const ret = wasm.linesegment_closestPoint(this.__wbg_ptr, point.__wbg_ptr);
        return Point3D.__wrap(ret);
    }
    /**
     * @param {Point3D} point
     * @returns {number}
     */
    distanceToPoint(point) {
        _assertClass(point, Point3D);
        const ret = wasm.linesegment_distanceToPoint(this.__wbg_ptr, point.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) LineSegment.prototype[Symbol.dispose] = LineSegment.prototype.free;

const PlaneFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_plane_free(ptr >>> 0, 1));

export class Plane {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Plane.prototype);
        obj.__wbg_ptr = ptr;
        PlaneFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PlaneFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_plane_free(ptr, 0);
    }
    /**
     * @param {Point3D} point
     * @param {Vector3D} normal
     * @returns {Plane}
     */
    static fromPointNormal(point, normal) {
        _assertClass(point, Point3D);
        _assertClass(normal, Vector3D);
        const ret = wasm.plane_fromPointNormal(point.__wbg_ptr, normal.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Plane.__wrap(ret[0]);
    }
    /**
     * @param {Point3D} p1
     * @param {Point3D} p2
     * @param {Point3D} p3
     * @returns {Plane}
     */
    static fromThreePoints(p1, p2, p3) {
        _assertClass(p1, Point3D);
        _assertClass(p2, Point3D);
        _assertClass(p3, Point3D);
        const ret = wasm.plane_fromThreePoints(p1.__wbg_ptr, p2.__wbg_ptr, p3.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Plane.__wrap(ret[0]);
    }
    /**
     * @returns {Vector3D}
     */
    get normal() {
        const ret = wasm.plane_normal(this.__wbg_ptr);
        return Vector3D.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    get d() {
        const ret = wasm.plane_d(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Point3D} point
     * @returns {number}
     */
    distanceToPoint(point) {
        _assertClass(point, Point3D);
        const ret = wasm.plane_distanceToPoint(this.__wbg_ptr, point.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Point3D} point
     * @returns {Point3D}
     */
    closestPoint(point) {
        _assertClass(point, Point3D);
        const ret = wasm.plane_closestPoint(this.__wbg_ptr, point.__wbg_ptr);
        return Point3D.__wrap(ret);
    }
    /**
     * @param {Point3D} point
     * @returns {boolean}
     */
    containsPoint(point) {
        _assertClass(point, Point3D);
        const ret = wasm.plane_containsPoint(this.__wbg_ptr, point.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {Plane}
     */
    flipNormal() {
        const ret = wasm.plane_flipNormal(this.__wbg_ptr);
        return Plane.__wrap(ret);
    }
    /**
     * @param {Plane} other
     * @returns {boolean}
     */
    isParallel(other) {
        _assertClass(other, Plane);
        const ret = wasm.plane_isParallel(this.__wbg_ptr, other.__wbg_ptr);
        return ret !== 0;
    }
}
if (Symbol.dispose) Plane.prototype[Symbol.dispose] = Plane.prototype.free;

const Point3DFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_point3d_free(ptr >>> 0, 1));

export class Point3D {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Point3D.prototype);
        obj.__wbg_ptr = ptr;
        Point3DFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof Point3D)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        Point3DFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_point3d_free(ptr, 0);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {number} z
     */
    constructor(x, y, z) {
        const ret = wasm.point3d_new(x, y, z);
        this.__wbg_ptr = ret >>> 0;
        Point3DFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.point3d_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.point3d_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get z() {
        const ret = wasm.point3d_z(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Point3D} other
     * @returns {number}
     */
    distanceTo(other) {
        _assertClass(other, Point3D);
        const ret = wasm.point3d_distanceTo(this.__wbg_ptr, other.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Point3D} other
     * @returns {Point3D}
     */
    midpoint(other) {
        _assertClass(other, Point3D);
        const ret = wasm.point3d_midpoint(this.__wbg_ptr, other.__wbg_ptr);
        return Point3D.__wrap(ret);
    }
    /**
     * @param {Vector3D} vector
     * @returns {Point3D}
     */
    translate(vector) {
        _assertClass(vector, Vector3D);
        const ret = wasm.point3d_translate(this.__wbg_ptr, vector.__wbg_ptr);
        return Point3D.__wrap(ret);
    }
    /**
     * @returns {Float64Array}
     */
    toArray() {
        const ret = wasm.point3d_toArray(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @param {Float64Array} arr
     * @returns {Point3D}
     */
    static fromArray(arr) {
        const ptr0 = passArrayF64ToWasm0(arr, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.point3d_fromArray(ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Point3D.__wrap(ret[0]);
    }
    /**
     * @returns {Point3D}
     */
    static origin() {
        const ret = wasm.point3d_origin();
        return Point3D.__wrap(ret);
    }
}
if (Symbol.dispose) Point3D.prototype[Symbol.dispose] = Point3D.prototype.free;

const RayFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_ray_free(ptr >>> 0, 1));

export class Ray {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RayFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ray_free(ptr, 0);
    }
    /**
     * @param {Point3D} origin
     * @param {Vector3D} direction
     */
    constructor(origin, direction) {
        _assertClass(origin, Point3D);
        _assertClass(direction, Vector3D);
        const ret = wasm.ray_new(origin.__wbg_ptr, direction.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        RayFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {Point3D}
     */
    get origin() {
        const ret = wasm.ray_origin(this.__wbg_ptr);
        return Point3D.__wrap(ret);
    }
    /**
     * @returns {Vector3D}
     */
    get direction() {
        const ret = wasm.ray_direction(this.__wbg_ptr);
        return Vector3D.__wrap(ret);
    }
    /**
     * @param {number} t
     * @returns {Point3D}
     */
    pointAt(t) {
        const ret = wasm.ray_pointAt(this.__wbg_ptr, t);
        return Point3D.__wrap(ret);
    }
    /**
     * @param {Sphere} sphere
     * @returns {any}
     */
    intersectSphere(sphere) {
        _assertClass(sphere, Sphere);
        const ret = wasm.ray_intersectSphere(this.__wbg_ptr, sphere.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Plane} plane
     * @returns {any}
     */
    intersectPlane(plane) {
        _assertClass(plane, Plane);
        const ret = wasm.ray_intersectPlane(this.__wbg_ptr, plane.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Triangle} triangle
     * @returns {any}
     */
    intersectTriangle(triangle) {
        _assertClass(triangle, Triangle);
        const ret = wasm.ray_intersectTriangle(this.__wbg_ptr, triangle.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) Ray.prototype[Symbol.dispose] = Ray.prototype.free;

const SVGRendererFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_svgrenderer_free(ptr >>> 0, 1));

export class SVGRenderer {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SVGRendererFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_svgrenderer_free(ptr, 0);
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {Camera} camera
     */
    constructor(width, height, camera) {
        _assertClass(camera, Camera);
        const ret = wasm.svgrenderer_new(width, height, camera.__wbg_ptr);
        this.__wbg_ptr = ret >>> 0;
        SVGRendererFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {string} color
     */
    setBackground(color) {
        const ptr0 = passStringToWasm0(color, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.svgrenderer_setBackground(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {Point3D} point
     * @param {string} color
     * @param {number} size
     */
    addPoint(point, color, size) {
        _assertClass(point, Point3D);
        const ptr0 = passStringToWasm0(color, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.svgrenderer_addPoint(this.__wbg_ptr, point.__wbg_ptr, ptr0, len0, size);
    }
    /**
     * @param {LineSegment} segment
     * @param {string} color
     * @param {number} width
     */
    addLineSegment(segment, color, width) {
        _assertClass(segment, LineSegment);
        const ptr0 = passStringToWasm0(color, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.svgrenderer_addLineSegment(this.__wbg_ptr, segment.__wbg_ptr, ptr0, len0, width);
    }
    /**
     * @param {Triangle} triangle
     * @param {string} stroke
     * @param {string | null | undefined} fill
     * @param {number} width
     */
    addTriangle(triangle, stroke, fill, width) {
        _assertClass(triangle, Triangle);
        const ptr0 = passStringToWasm0(stroke, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(fill) ? 0 : passStringToWasm0(fill, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        wasm.svgrenderer_addTriangle(this.__wbg_ptr, triangle.__wbg_ptr, ptr0, len0, ptr1, len1, width);
    }
    /**
     * @param {Sphere} sphere
     * @param {string} color
     * @param {number} width
     */
    addSphere(sphere, color, width) {
        _assertClass(sphere, Sphere);
        const ptr0 = passStringToWasm0(color, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.svgrenderer_addSphere(this.__wbg_ptr, sphere.__wbg_ptr, ptr0, len0, width);
    }
    /**
     * @param {AABB} aabb
     * @param {string} color
     * @param {number} width
     */
    addAabb(aabb, color, width) {
        _assertClass(aabb, AABB);
        const ptr0 = passStringToWasm0(color, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.svgrenderer_addAabb(this.__wbg_ptr, aabb.__wbg_ptr, ptr0, len0, width);
    }
    /**
     * @returns {string}
     */
    toSvgString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.svgrenderer_toSvgString(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) SVGRenderer.prototype[Symbol.dispose] = SVGRenderer.prototype.free;

const SphereFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_sphere_free(ptr >>> 0, 1));

export class Sphere {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SphereFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sphere_free(ptr, 0);
    }
    /**
     * @param {Point3D} center
     * @param {number} radius
     */
    constructor(center, radius) {
        _assertClass(center, Point3D);
        const ret = wasm.sphere_new(center.__wbg_ptr, radius);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        SphereFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {Point3D}
     */
    get center() {
        const ret = wasm.sphere_center(this.__wbg_ptr);
        return Point3D.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    get radius() {
        const ret = wasm.sphere_radius(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    volume() {
        const ret = wasm.sphere_volume(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    surfaceArea() {
        const ret = wasm.sphere_surfaceArea(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Point3D} point
     * @returns {boolean}
     */
    contains(point) {
        _assertClass(point, Point3D);
        const ret = wasm.sphere_contains(this.__wbg_ptr, point.__wbg_ptr);
        return ret !== 0;
    }
}
if (Symbol.dispose) Sphere.prototype[Symbol.dispose] = Sphere.prototype.free;

const TriangleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_triangle_free(ptr >>> 0, 1));

export class Triangle {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TriangleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_triangle_free(ptr, 0);
    }
    /**
     * @param {Point3D} a
     * @param {Point3D} b
     * @param {Point3D} c
     */
    constructor(a, b, c) {
        _assertClass(a, Point3D);
        _assertClass(b, Point3D);
        _assertClass(c, Point3D);
        const ret = wasm.triangle_new(a.__wbg_ptr, b.__wbg_ptr, c.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        TriangleFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {Point3D}
     */
    get a() {
        const ret = wasm.plane_normal(this.__wbg_ptr);
        return Point3D.__wrap(ret);
    }
    /**
     * @returns {Point3D}
     */
    get b() {
        const ret = wasm.triangle_b(this.__wbg_ptr);
        return Point3D.__wrap(ret);
    }
    /**
     * @returns {Point3D}
     */
    get c() {
        const ret = wasm.triangle_c(this.__wbg_ptr);
        return Point3D.__wrap(ret);
    }
    /**
     * @returns {Vector3D}
     */
    normal() {
        const ret = wasm.triangle_normal(this.__wbg_ptr);
        return Vector3D.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    area() {
        const ret = wasm.triangle_area(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Point3D}
     */
    centroid() {
        const ret = wasm.triangle_centroid(this.__wbg_ptr);
        return Point3D.__wrap(ret);
    }
    /**
     * @param {Point3D} point
     * @returns {Float64Array}
     */
    barycentricCoords(point) {
        _assertClass(point, Point3D);
        const ret = wasm.triangle_barycentricCoords(this.__wbg_ptr, point.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @param {Point3D} point
     * @returns {boolean}
     */
    containsPoint(point) {
        _assertClass(point, Point3D);
        const ret = wasm.triangle_containsPoint(this.__wbg_ptr, point.__wbg_ptr);
        return ret !== 0;
    }
}
if (Symbol.dispose) Triangle.prototype[Symbol.dispose] = Triangle.prototype.free;

const Vector3DFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_vector3d_free(ptr >>> 0, 1));

export class Vector3D {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Vector3D.prototype);
        obj.__wbg_ptr = ptr;
        Vector3DFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        Vector3DFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_vector3d_free(ptr, 0);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {number} z
     */
    constructor(x, y, z) {
        const ret = wasm.vector3d_new(x, y, z);
        this.__wbg_ptr = ret >>> 0;
        Vector3DFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.vector3d_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.vector3d_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get z() {
        const ret = wasm.vector3d_z(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    magnitude() {
        const ret = wasm.vector3d_magnitude(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Vector3D}
     */
    normalize() {
        const ret = wasm.vector3d_normalize(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Vector3D.__wrap(ret[0]);
    }
    /**
     * @param {Vector3D} other
     * @returns {number}
     */
    dot(other) {
        _assertClass(other, Vector3D);
        const ret = wasm.vector3d_dot(this.__wbg_ptr, other.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Vector3D} other
     * @returns {Vector3D}
     */
    cross(other) {
        _assertClass(other, Vector3D);
        const ret = wasm.vector3d_cross(this.__wbg_ptr, other.__wbg_ptr);
        return Vector3D.__wrap(ret);
    }
    /**
     * @param {Vector3D} other
     * @returns {Vector3D}
     */
    add(other) {
        _assertClass(other, Vector3D);
        const ret = wasm.vector3d_add(this.__wbg_ptr, other.__wbg_ptr);
        return Vector3D.__wrap(ret);
    }
    /**
     * @param {Vector3D} other
     * @returns {Vector3D}
     */
    sub(other) {
        _assertClass(other, Vector3D);
        const ret = wasm.vector3d_sub(this.__wbg_ptr, other.__wbg_ptr);
        return Vector3D.__wrap(ret);
    }
    /**
     * @param {number} scalar
     * @returns {Vector3D}
     */
    scale(scalar) {
        const ret = wasm.vector3d_scale(this.__wbg_ptr, scalar);
        return Vector3D.__wrap(ret);
    }
    /**
     * @param {Vector3D} other
     * @returns {number}
     */
    angle(other) {
        _assertClass(other, Vector3D);
        const ret = wasm.vector3d_angle(this.__wbg_ptr, other.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Vector3D} other
     * @returns {Vector3D}
     */
    projectOnto(other) {
        _assertClass(other, Vector3D);
        const ret = wasm.vector3d_projectOnto(this.__wbg_ptr, other.__wbg_ptr);
        return Vector3D.__wrap(ret);
    }
    /**
     * @param {Vector3D} other
     * @returns {boolean}
     */
    isParallel(other) {
        _assertClass(other, Vector3D);
        const ret = wasm.vector3d_isParallel(this.__wbg_ptr, other.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {Vector3D} other
     * @returns {boolean}
     */
    isPerpendicular(other) {
        _assertClass(other, Vector3D);
        const ret = wasm.vector3d_isPerpendicular(this.__wbg_ptr, other.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {Float64Array}
     */
    toArray() {
        const ret = wasm.vector3d_toArray(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @param {Float64Array} arr
     * @returns {Vector3D}
     */
    static fromArray(arr) {
        const ptr0 = passArrayF64ToWasm0(arr, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.vector3d_fromArray(ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Vector3D.__wrap(ret[0]);
    }
    /**
     * @returns {Vector3D}
     */
    static zero() {
        const ret = wasm.vector3d_zero();
        return Vector3D.__wrap(ret);
    }
    /**
     * @returns {Vector3D}
     */
    static unitX() {
        const ret = wasm.vector3d_unitX();
        return Vector3D.__wrap(ret);
    }
    /**
     * @returns {Vector3D}
     */
    static unitY() {
        const ret = wasm.vector3d_unitY();
        return Vector3D.__wrap(ret);
    }
    /**
     * @returns {Vector3D}
     */
    static unitZ() {
        const ret = wasm.vector3d_unitZ();
        return Vector3D.__wrap(ret);
    }
    /**
     * @param {Point3D} from
     * @param {Point3D} to
     * @returns {Vector3D}
     */
    static fromPoints(from, to) {
        _assertClass(from, Point3D);
        _assertClass(to, Point3D);
        const ret = wasm.vector3d_fromPoints(from.__wbg_ptr, to.__wbg_ptr);
        return Vector3D.__wrap(ret);
    }
}
if (Symbol.dispose) Vector3D.prototype[Symbol.dispose] = Vector3D.prototype.free;

const EXPECTED_RESPONSE_TYPES = new Set(['basic', 'cors', 'default']);

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                const validResponse = module.ok && EXPECTED_RESPONSE_TYPES.has(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_new_19c25a3f2fa63a02 = function() {
        const ret = new Object();
        return ret;
    };
    imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
        const ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_point3d_new = function(arg0) {
        const ret = Point3D.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_point3d_unwrap = function(arg0) {
        const ret = Point3D.__unwrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_set_453345bcda80b89a = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = Reflect.set(arg0, arg1, arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
        const ret = arg1.stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_wbindgendebugstring_99ef257a3ddda34d = function(arg0, arg1) {
        const ret = debugString(arg1);
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_wbindgenthrow_451ec1a8469d7eb6 = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_cast_2241b6af4c4b2941 = function(arg0, arg1) {
        // Cast intrinsic for `Ref(String) -> Externref`.
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_cast_d6cd19b81560fd6e = function(arg0) {
        // Cast intrinsic for `F64 -> Externref`.
        const ret = arg0;
        return ret;
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_3;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedFloat64ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('common_core_geometry_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
