use super::camera::Camera;
use super::projection::project_point;
use crate::primitives::{LineSegment, Point3D, Sphere, Triangle, AABB};
use std::fmt;
use std::io::Write;

#[derive(Debug, Clone)]
pub enum SVGElement {
    Circle {
        cx: f64,
        cy: f64,
        r: f64,
        stroke: String,
        fill: Option<String>,
        stroke_width: f64,
    },
    Line {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        stroke: String,
        stroke_width: f64,
    },
    Polygon {
        points: Vec<(f64, f64)>,
        stroke: String,
        fill: Option<String>,
        stroke_width: f64,
    },
}

impl SVGElement {
    fn to_svg_string(&self) -> String {
        match self {
            SVGElement::Circle {
                cx,
                cy,
                r,
                stroke,
                fill,
                stroke_width,
            } => {
                let fill_str = fill
                    .as_ref()
                    .map(|f| format!("fill=\"{}\"", f))
                    .unwrap_or_else(|| "fill=\"none\"".to_string());
                format!(
                    "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{:.2}\" stroke=\"{}\" {} stroke-width=\"{:.2}\" />",
                    cx, cy, r, stroke, fill_str, stroke_width
                )
            }
            SVGElement::Line {
                x1,
                y1,
                x2,
                y2,
                stroke,
                stroke_width,
            } => {
                format!(
                    "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" stroke-width=\"{:.2}\" />",
                    x1, y1, x2, y2, stroke, stroke_width
                )
            }
            SVGElement::Polygon {
                points,
                stroke,
                fill,
                stroke_width,
            } => {
                let points_str = points
                    .iter()
                    .map(|(x, y)| format!("{:.2},{:.2}", x, y))
                    .collect::<Vec<_>>()
                    .join(" ");
                let fill_str = fill
                    .as_ref()
                    .map(|f| format!("fill=\"{}\"", f))
                    .unwrap_or_else(|| "fill=\"none\"".to_string());
                format!(
                    "<polygon points=\"{}\" stroke=\"{}\" {} stroke-width=\"{:.2}\" />",
                    points_str, stroke, fill_str, stroke_width
                )
            }
        }
    }
}

pub struct SVGRenderer {
    width: usize,
    height: usize,
    camera: Camera,
    background: Option<String>,
    elements: Vec<SVGElement>,
}

impl SVGRenderer {
    pub fn new(width: usize, height: usize, camera: Camera) -> Self {
        Self {
            width,
            height,
            camera,
            background: None,
            elements: Vec::new(),
        }
    }

    pub fn set_background(&mut self, color: &str) {
        self.background = Some(color.to_string());
    }

    pub fn add_point(&mut self, point: &Point3D, color: &str, size: f64) {
        let (x, y) = project_point(point, &self.camera, self.width, self.height);
        self.elements.push(SVGElement::Circle {
            cx: x,
            cy: y,
            r: size,
            stroke: color.to_string(),
            fill: Some(color.to_string()),
            stroke_width: 1.0,
        });
    }

    pub fn add_line_segment(&mut self, segment: &LineSegment, color: &str, width: f64) {
        let (x1, y1) = project_point(&segment.start, &self.camera, self.width, self.height);
        let (x2, y2) = project_point(&segment.end, &self.camera, self.width, self.height);
        self.elements.push(SVGElement::Line {
            x1,
            y1,
            x2,
            y2,
            stroke: color.to_string(),
            stroke_width: width,
        });
    }

    pub fn add_triangle(
        &mut self,
        triangle: &Triangle,
        stroke: &str,
        fill: Option<&str>,
        width: f64,
    ) {
        let (x1, y1) = project_point(&triangle.a, &self.camera, self.width, self.height);
        let (x2, y2) = project_point(&triangle.b, &self.camera, self.width, self.height);
        let (x3, y3) = project_point(&triangle.c, &self.camera, self.width, self.height);

        self.elements.push(SVGElement::Polygon {
            points: vec![(x1, y1), (x2, y2), (x3, y3)],
            stroke: stroke.to_string(),
            fill: fill.map(|s| s.to_string()),
            stroke_width: width,
        });
    }

    pub fn add_sphere(&mut self, sphere: &Sphere, color: &str, width: f64) {
        let (cx, cy) = project_point(&sphere.center, &self.camera, self.width, self.height);

        let radius_point = Point3D::new(
            sphere.center.x + sphere.radius,
            sphere.center.y,
            sphere.center.z,
        );
        let (rx, _) = project_point(&radius_point, &self.camera, self.width, self.height);
        let projected_radius = (rx - cx).abs();

        self.elements.push(SVGElement::Circle {
            cx,
            cy,
            r: projected_radius,
            stroke: color.to_string(),
            fill: None,
            stroke_width: width,
        });
    }

    pub fn add_aabb(&mut self, aabb: &AABB, color: &str, width: f64) {
        let min = &aabb.min;
        let max = &aabb.max;

        let corners = [
            Point3D::new(min.x, min.y, min.z),
            Point3D::new(max.x, min.y, min.z),
            Point3D::new(max.x, max.y, min.z),
            Point3D::new(min.x, max.y, min.z),
            Point3D::new(min.x, min.y, max.z),
            Point3D::new(max.x, min.y, max.z),
            Point3D::new(max.x, max.y, max.z),
            Point3D::new(min.x, max.y, max.z),
        ];

        let edges = [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ];

        for (i, j) in edges.iter() {
            let segment = LineSegment::new(corners[*i], corners[*j]).unwrap();
            self.add_line_segment(&segment, color, width);
        }
    }

    pub fn to_svg_string(&self) -> String {
        format!("{}", self)
    }

    pub fn to_file(&self, path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        file.write_all(self.to_svg_string().as_bytes())?;
        Ok(())
    }
}

impl fmt::Display for SVGRenderer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",
            self.width, self.height
        )?;

        if let Some(bg) = &self.background {
            writeln!(
                f,
                "  <rect width=\"100%\" height=\"100%\" fill=\"{}\" />",
                bg
            )?;
        }

        for element in &self.elements {
            writeln!(f, "  {}", element.to_svg_string())?;
        }

        write!(f, "</svg>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::Vector3D;

    #[test]
    fn test_svg_renderer_creation() {
        let camera = Camera::perspective(
            Point3D::new(0.0, 0.0, 5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            60.0,
            16.0 / 9.0,
            0.1,
            100.0,
        );

        let renderer = SVGRenderer::new(800, 600, camera);
        assert_eq!(renderer.width, 800);
        assert_eq!(renderer.height, 600);
    }

    #[test]
    fn test_add_point() {
        let camera = Camera::perspective(
            Point3D::new(0.0, 0.0, 5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            60.0,
            1.0,
            0.1,
            100.0,
        );

        let mut renderer = SVGRenderer::new(800, 600, camera);
        renderer.add_point(&Point3D::new(0.0, 0.0, 0.0), "#ff0000", 5.0);
        assert_eq!(renderer.elements.len(), 1);
    }

    #[test]
    fn test_add_line_segment() {
        let camera = Camera::perspective(
            Point3D::new(0.0, 0.0, 5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            60.0,
            1.0,
            0.1,
            100.0,
        );

        let mut renderer = SVGRenderer::new(800, 600, camera);
        let segment =
            LineSegment::new(Point3D::new(-1.0, 0.0, 0.0), Point3D::new(1.0, 0.0, 0.0)).unwrap();
        renderer.add_line_segment(&segment, "#00ff00", 2.0);
        assert_eq!(renderer.elements.len(), 1);
    }

    #[test]
    fn test_add_triangle() {
        let camera = Camera::perspective(
            Point3D::new(0.0, 0.0, 5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            60.0,
            1.0,
            0.1,
            100.0,
        );

        let mut renderer = SVGRenderer::new(800, 600, camera);
        let triangle = Triangle::new(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, -1.0, 0.0),
            Point3D::new(1.0, -1.0, 0.0),
        )
        .unwrap();
        renderer.add_triangle(&triangle, "#0000ff", Some("#ccccff"), 1.0);
        assert_eq!(renderer.elements.len(), 1);
    }

    #[test]
    fn test_add_sphere() {
        let camera = Camera::perspective(
            Point3D::new(0.0, 0.0, 5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            60.0,
            1.0,
            0.1,
            100.0,
        );

        let mut renderer = SVGRenderer::new(800, 600, camera);
        let sphere = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0).unwrap();
        renderer.add_sphere(&sphere, "#ff00ff", 2.0);
        assert_eq!(renderer.elements.len(), 1);
    }

    #[test]
    fn test_add_aabb() {
        let camera = Camera::perspective(
            Point3D::new(0.0, 0.0, 5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            60.0,
            1.0,
            0.1,
            100.0,
        );

        let mut renderer = SVGRenderer::new(800, 600, camera);
        let aabb = AABB::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0)).unwrap();
        renderer.add_aabb(&aabb, "#00ffff", 1.0);
        assert_eq!(renderer.elements.len(), 12);
    }

    #[test]
    fn test_set_background() {
        let camera = Camera::perspective(
            Point3D::new(0.0, 0.0, 5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            60.0,
            1.0,
            0.1,
            100.0,
        );

        let mut renderer = SVGRenderer::new(800, 600, camera);
        renderer.set_background("#ffffff");
        assert_eq!(renderer.background, Some("#ffffff".to_string()));
    }

    #[test]
    fn test_to_string() {
        let camera = Camera::perspective(
            Point3D::new(0.0, 0.0, 5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            60.0,
            1.0,
            0.1,
            100.0,
        );

        let mut renderer = SVGRenderer::new(800, 600, camera);
        renderer.set_background("#ffffff");
        renderer.add_point(&Point3D::new(0.0, 0.0, 0.0), "#ff0000", 5.0);

        let svg_string = renderer.to_string();
        assert!(svg_string.contains("<svg"));
        assert!(svg_string.contains("</svg>"));
        assert!(svg_string.contains("#ffffff"));
        assert!(svg_string.contains("#ff0000"));
    }
}
