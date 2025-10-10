pub const EPSILON: f64 = 1e-10;

#[inline]
pub fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[inline]
pub fn approx_zero(a: f64) -> bool {
    a.abs() < EPSILON
}

#[inline]
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

#[inline]
pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / std::f64::consts::PI
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approx_eq() {
        assert!(approx_eq(1.0, 1.0));
        assert!(approx_eq(1.0, 1.0 + EPSILON / 2.0));
        assert!(!approx_eq(1.0, 1.0 + EPSILON * 2.0));
    }

    #[test]
    fn test_approx_zero() {
        assert!(approx_zero(0.0));
        assert!(approx_zero(EPSILON / 2.0));
        assert!(!approx_zero(EPSILON * 2.0));
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(clamp(-5.0, 0.0, 10.0), 0.0);
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn test_angle_conversion() {
        assert!(approx_eq(degrees_to_radians(180.0), std::f64::consts::PI));
        assert!(approx_eq(radians_to_degrees(std::f64::consts::PI), 180.0));
    }
}
