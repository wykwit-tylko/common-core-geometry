import { describe, it } from 'bun:test';
import { expect } from 'bun:test';
import { Point3D, Vector3D, LineSegment, Triangle, Sphere, AABB, Camera, SVGRenderer } from '../pkg-nodejs/common_core_geometry_wasm.js';

describe('Camera', () => {
    it('perspective creation', () => {
        const position = new Point3D(0, 0, 5);
        const target = new Point3D(0, 0, 0);
        const up = new Vector3D(0, 1, 0);
        const camera = Camera.perspective(position, target, up, 60, 16/9, 0.1, 100);
        
        expect(camera).toBeDefined();
        const viewMatrix = camera.viewMatrix();
        expect(viewMatrix).toBeInstanceOf(Float64Array);
        expect(viewMatrix.length).toBe(16);
        
        const projMatrix = camera.projectionMatrix();
        expect(projMatrix).toBeInstanceOf(Float64Array);
        expect(projMatrix.length).toBe(16);
    });

    it('orthographic creation', () => {
        const position = new Point3D(0, 0, 5);
        const target = new Point3D(0, 0, 0);
        const up = new Vector3D(0, 1, 0);
        const camera = Camera.orthographic(position, target, up, 10, 10);
        
        expect(camera).toBeDefined();
        const viewMatrix = camera.viewMatrix();
        expect(viewMatrix).toBeInstanceOf(Float64Array);
        expect(viewMatrix.length).toBe(16);
    });
});

describe('SVGRenderer', () => {
    it('creation', () => {
        const camera = Camera.perspective(
            new Point3D(0, 0, 5),
            new Point3D(0, 0, 0),
            new Vector3D(0, 1, 0),
            60, 1, 0.1, 100
        );
        const renderer = new SVGRenderer(800, 600, camera);
        
        expect(renderer).toBeDefined();
        const svg = renderer.toSvgString();
        expect(svg).toContain('<svg');
        expect(svg).toContain('</svg>');
    });

    it('setBackground', () => {
        const camera = Camera.perspective(
            new Point3D(0, 0, 5),
            new Point3D(0, 0, 0),
            new Vector3D(0, 1, 0),
            60, 1, 0.1, 100
        );
        const renderer = new SVGRenderer(800, 600, camera);
        renderer.setBackground('#ffffff');
        
        const svg = renderer.toSvgString();
        expect(svg).toContain('#ffffff');
        expect(svg).toContain('<rect');
    });

    it('addPoint', () => {
        const camera = Camera.perspective(
            new Point3D(0, 0, 5),
            new Point3D(0, 0, 0),
            new Vector3D(0, 1, 0),
            60, 1, 0.1, 100
        );
        const renderer = new SVGRenderer(800, 600, camera);
        const point = new Point3D(0, 0, 0);
        renderer.addPoint(point, '#ff0000', 5);
        
        const svg = renderer.toSvgString();
        expect(svg).toContain('<circle');
        expect(svg).toContain('#ff0000');
    });

    it('addLineSegment', () => {
        const camera = Camera.perspective(
            new Point3D(0, 0, 5),
            new Point3D(0, 0, 0),
            new Vector3D(0, 1, 0),
            60, 1, 0.1, 100
        );
        const renderer = new SVGRenderer(800, 600, camera);
        const segment = new LineSegment(
            new Point3D(-1, 0, 0),
            new Point3D(1, 0, 0)
        );
        renderer.addLineSegment(segment, '#00ff00', 2);
        
        const svg = renderer.toSvgString();
        expect(svg).toContain('<line');
        expect(svg).toContain('#00ff00');
    });

    it('addTriangle', () => {
        const camera = Camera.perspective(
            new Point3D(0, 0, 5),
            new Point3D(0, 0, 0),
            new Vector3D(0, 1, 0),
            60, 1, 0.1, 100
        );
        const renderer = new SVGRenderer(800, 600, camera);
        const triangle = new Triangle(
            new Point3D(0, 1, 0),
            new Point3D(-1, -1, 0),
            new Point3D(1, -1, 0)
        );
        renderer.addTriangle(triangle, '#0000ff', '#ccccff', 1);
        
        const svg = renderer.toSvgString();
        expect(svg).toContain('<polygon');
        expect(svg).toContain('#0000ff');
        expect(svg).toContain('#ccccff');
    });

    it('addTriangle without fill', () => {
        const camera = Camera.perspective(
            new Point3D(0, 0, 5),
            new Point3D(0, 0, 0),
            new Vector3D(0, 1, 0),
            60, 1, 0.1, 100
        );
        const renderer = new SVGRenderer(800, 600, camera);
        const triangle = new Triangle(
            new Point3D(0, 1, 0),
            new Point3D(-1, -1, 0),
            new Point3D(1, -1, 0)
        );
        renderer.addTriangle(triangle, '#0000ff', undefined, 1);
        
        const svg = renderer.toSvgString();
        expect(svg).toContain('<polygon');
        expect(svg).toContain('#0000ff');
        expect(svg).toContain('fill="none"');
    });

    it('addSphere', () => {
        const camera = Camera.perspective(
            new Point3D(0, 0, 5),
            new Point3D(0, 0, 0),
            new Vector3D(0, 1, 0),
            60, 1, 0.1, 100
        );
        const renderer = new SVGRenderer(800, 600, camera);
        const sphere = new Sphere(new Point3D(0, 0, 0), 1);
        renderer.addSphere(sphere, '#ff00ff', 2);
        
        const svg = renderer.toSvgString();
        expect(svg).toContain('<circle');
        expect(svg).toContain('#ff00ff');
    });

    it('addAabb', () => {
        const camera = Camera.perspective(
            new Point3D(0, 0, 5),
            new Point3D(0, 0, 0),
            new Vector3D(0, 1, 0),
            60, 1, 0.1, 100
        );
        const renderer = new SVGRenderer(800, 600, camera);
        const aabb = new AABB(
            new Point3D(-1, -1, -1),
            new Point3D(1, 1, 1)
        );
        renderer.addAabb(aabb, '#00ffff', 1);
        
        const svg = renderer.toSvgString();
        expect(svg).toContain('<line');
        expect(svg).toContain('#00ffff');
    });

    it('complex scene', () => {
        const camera = Camera.perspective(
            new Point3D(5, 5, 5),
            new Point3D(0, 0, 0),
            new Vector3D(0, 1, 0),
            60, 1, 0.1, 100
        );
        const renderer = new SVGRenderer(800, 600, camera);
        
        renderer.setBackground('#ffffff');
        renderer.addPoint(new Point3D(0, 0, 0), '#ff0000', 5);
        renderer.addLineSegment(
            new LineSegment(new Point3D(0, 0, 0), new Point3D(1, 0, 0)),
            '#ff0000',
            2
        );
        renderer.addLineSegment(
            new LineSegment(new Point3D(0, 0, 0), new Point3D(0, 1, 0)),
            '#00ff00',
            2
        );
        renderer.addLineSegment(
            new LineSegment(new Point3D(0, 0, 0), new Point3D(0, 0, 1)),
            '#0000ff',
            2
        );
        
        const svg = renderer.toSvgString();
        expect(svg).toContain('<svg');
        expect(svg).toContain('</svg>');
        expect(svg).toContain('#ffffff');
        expect(svg).toContain('#ff0000');
        expect(svg).toContain('#00ff00');
        expect(svg).toContain('#0000ff');
    });
});

