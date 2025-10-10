use super::camera::Camera;
use crate::primitives::Point3D;

pub fn multiply_matrix_point(matrix: &[[f64; 4]; 4], point: &Point3D) -> [f64; 4] {
    let x = matrix[0][0] * point.x + matrix[0][1] * point.y + matrix[0][2] * point.z + matrix[0][3];
    let y = matrix[1][0] * point.x + matrix[1][1] * point.y + matrix[1][2] * point.z + matrix[1][3];
    let z = matrix[2][0] * point.x + matrix[2][1] * point.y + matrix[2][2] * point.z + matrix[2][3];
    let w = matrix[3][0] * point.x + matrix[3][1] * point.y + matrix[3][2] * point.z + matrix[3][3];
    [x, y, z, w]
}

pub fn multiply_matrices(a: &[[f64; 4]; 4], b: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            result[i][j] = (0..4).map(|k| a[i][k] * b[k][j]).sum();
        }
    }
    result
}

pub fn perspective_divide(homogeneous: [f64; 4]) -> (f64, f64, f64) {
    if homogeneous[3].abs() < 1e-10 {
        (homogeneous[0], homogeneous[1], homogeneous[2])
    } else {
        (
            homogeneous[0] / homogeneous[3],
            homogeneous[1] / homogeneous[3],
            homogeneous[2] / homogeneous[3],
        )
    }
}

pub fn project_point(point: &Point3D, camera: &Camera, width: usize, height: usize) -> (f64, f64) {
    let view_matrix = camera.view_matrix();
    let projection_matrix = camera.projection_matrix();
    let vp_matrix = multiply_matrices(&projection_matrix, &view_matrix);

    let homogeneous = multiply_matrix_point(&vp_matrix, point);
    let (ndc_x, ndc_y, _ndc_z) = perspective_divide(homogeneous);

    let screen_x = (ndc_x + 1.0) * 0.5 * width as f64;
    let screen_y = (1.0 - ndc_y) * 0.5 * height as f64;

    (screen_x, screen_y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::Vector3D;

    #[test]
    fn test_multiply_matrix_point() {
        let identity: [[f64; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let point = Point3D::new(1.0, 2.0, 3.0);
        let result = multiply_matrix_point(&identity, &point);
        assert_eq!(result, [1.0, 2.0, 3.0, 1.0]);
    }

    #[test]
    fn test_multiply_matrices() {
        let identity: [[f64; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let result = multiply_matrices(&identity, &identity);
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    assert_eq!(result[i][j], 1.0);
                } else {
                    assert_eq!(result[i][j], 0.0);
                }
            }
        }
    }

    #[test]
    fn test_perspective_divide() {
        let homogeneous = [2.0, 4.0, 6.0, 2.0];
        let (x, y, z) = perspective_divide(homogeneous);
        assert_eq!(x, 1.0);
        assert_eq!(y, 2.0);
        assert_eq!(z, 3.0);
    }

    #[test]
    fn test_project_point() {
        let camera = Camera::perspective(
            Point3D::new(0.0, 0.0, 5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            60.0,
            1.0,
            0.1,
            100.0,
        );

        let point = Point3D::new(0.0, 0.0, 0.0);
        let (screen_x, screen_y) = project_point(&point, &camera, 800, 600);

        assert!(screen_x >= 0.0 && screen_x <= 800.0);
        assert!(screen_y >= 0.0 && screen_y <= 600.0);
    }

    #[test]
    fn test_project_point_orthographic() {
        let camera = Camera::orthographic(
            Point3D::new(0.0, 0.0, 5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            10.0,
            10.0,
        );

        let point = Point3D::new(0.0, 0.0, 0.0);
        let (screen_x, screen_y) = project_point(&point, &camera, 800, 600);

        assert!(screen_x >= 0.0 && screen_x <= 800.0);
        assert!(screen_y >= 0.0 && screen_y <= 600.0);
    }
}
