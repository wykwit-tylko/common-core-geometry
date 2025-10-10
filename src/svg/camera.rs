use crate::primitives::{Point3D, Vector3D};

#[derive(Debug, Clone)]
pub enum Camera {
    Perspective {
        position: Point3D,
        target: Point3D,
        up: Vector3D,
        fov: f64,
        aspect: f64,
        near: f64,
        far: f64,
    },
    Orthographic {
        position: Point3D,
        target: Point3D,
        up: Vector3D,
        width: f64,
        height: f64,
    },
}

impl Camera {
    pub fn perspective(
        position: Point3D,
        target: Point3D,
        up: Vector3D,
        fov: f64,
        aspect: f64,
        near: f64,
        far: f64,
    ) -> Self {
        Camera::Perspective {
            position,
            target,
            up,
            fov,
            aspect,
            near,
            far,
        }
    }

    pub fn orthographic(
        position: Point3D,
        target: Point3D,
        up: Vector3D,
        width: f64,
        height: f64,
    ) -> Self {
        Camera::Orthographic {
            position,
            target,
            up,
            width,
            height,
        }
    }

    pub fn view_matrix(&self) -> [[f64; 4]; 4] {
        let (position, target, up) = match self {
            Camera::Perspective {
                position,
                target,
                up,
                ..
            } => (position, target, up),
            Camera::Orthographic {
                position,
                target,
                up,
                ..
            } => (position, target, up),
        };

        let forward = Vector3D::new(
            target.x - position.x,
            target.y - position.y,
            target.z - position.z,
        )
        .normalize()
        .unwrap_or_else(|_| Vector3D::new(0.0, 0.0, -1.0));

        let right = forward
            .cross(up)
            .normalize()
            .unwrap_or_else(|_| Vector3D::new(1.0, 0.0, 0.0));
        let camera_up = right.cross(&forward);

        [
            [
                right.x,
                right.y,
                right.z,
                -right.dot(&Vector3D::new(position.x, position.y, position.z)),
            ],
            [
                camera_up.x,
                camera_up.y,
                camera_up.z,
                -camera_up.dot(&Vector3D::new(position.x, position.y, position.z)),
            ],
            [
                -forward.x,
                -forward.y,
                -forward.z,
                forward.dot(&Vector3D::new(position.x, position.y, position.z)),
            ],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }

    pub fn projection_matrix(&self) -> [[f64; 4]; 4] {
        match self {
            Camera::Perspective {
                fov,
                aspect,
                near,
                far,
                ..
            } => {
                let fov_rad = fov.to_radians();
                let tan_half_fov = (fov_rad / 2.0).tan();

                let a = 1.0 / (aspect * tan_half_fov);
                let b = 1.0 / tan_half_fov;
                let c = -(far + near) / (far - near);
                let d = -(2.0 * far * near) / (far - near);

                [
                    [a, 0.0, 0.0, 0.0],
                    [0.0, b, 0.0, 0.0],
                    [0.0, 0.0, c, d],
                    [0.0, 0.0, -1.0, 0.0],
                ]
            }
            Camera::Orthographic { width, height, .. } => {
                let right = width / 2.0;
                let left = -right;
                let top = height / 2.0;
                let bottom = -top;
                let near = -1.0;
                let far = 1.0;

                [
                    [
                        2.0 / (right - left),
                        0.0,
                        0.0,
                        -(right + left) / (right - left),
                    ],
                    [
                        0.0,
                        2.0 / (top - bottom),
                        0.0,
                        -(top + bottom) / (top - bottom),
                    ],
                    [0.0, 0.0, -2.0 / (far - near), -(far + near) / (far - near)],
                    [0.0, 0.0, 0.0, 1.0],
                ]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perspective_camera_creation() {
        let camera = Camera::perspective(
            Point3D::new(0.0, 0.0, 5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            60.0,
            16.0 / 9.0,
            0.1,
            100.0,
        );

        match camera {
            Camera::Perspective { fov, aspect, .. } => {
                assert_eq!(fov, 60.0);
                assert_eq!(aspect, 16.0 / 9.0);
            }
            _ => panic!("Expected perspective camera"),
        }
    }

    #[test]
    fn test_orthographic_camera_creation() {
        let camera = Camera::orthographic(
            Point3D::new(0.0, 0.0, 5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            10.0,
            10.0,
        );

        match camera {
            Camera::Orthographic { width, height, .. } => {
                assert_eq!(width, 10.0);
                assert_eq!(height, 10.0);
            }
            _ => panic!("Expected orthographic camera"),
        }
    }

    #[test]
    fn test_view_matrix() {
        let camera = Camera::perspective(
            Point3D::new(0.0, 0.0, 5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            60.0,
            1.0,
            0.1,
            100.0,
        );

        let view_matrix = camera.view_matrix();
        assert_eq!(view_matrix[3][3], 1.0);
    }

    #[test]
    fn test_projection_matrix_perspective() {
        let camera = Camera::perspective(
            Point3D::new(0.0, 0.0, 5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            60.0,
            1.0,
            0.1,
            100.0,
        );

        let proj_matrix = camera.projection_matrix();
        assert_eq!(proj_matrix[3][2], -1.0);
    }

    #[test]
    fn test_projection_matrix_orthographic() {
        let camera = Camera::orthographic(
            Point3D::new(0.0, 0.0, 5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            10.0,
            10.0,
        );

        let proj_matrix = camera.projection_matrix();
        assert_eq!(proj_matrix[3][3], 1.0);
    }
}
