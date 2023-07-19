
pub fn interpolate_2d_f(x1: f64, y1: f64, x2: f64, y2: f64) -> [f64;2] {
    let div: f64 = x2 - x1;
    let m: f64;
    match div as i64 {
        0 => m = 0.0,
        _ => m = (y2 - y1) / (x2 - x1),
    }
    let n: f64 = y1 - m * x1;
    [
        m,
        n
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m_0_n_0() {
        let result = interpolate_2d_f(1.0, 1.0, 1.0, 1.0);
        assert_eq!(result[0], 0.0);
        assert_eq!(result[1], 1.0);
    }

    #[test]
    fn m_1_n_0() {
        let result = interpolate_2d_f(2.0, 2.0, 10.0, 10.0);
        assert_eq!(result[0], 1.0);
        assert_eq!(result[1], 0.0);
    }

    #[test]
    fn m_0_n_1() {
        let result = interpolate_2d_f(1.0, 2.0, 2.0, 2.0);
        assert_eq!(result[0], 0.0);
        assert_eq!(result[1], 2.0);
    }

    #[test]
    fn m_x_n_y() {
        let m: f64 = -371.0;
        let n: f64 = 503.0;
        let (x1, x2) = (578.0, -959.0);
        let (y1, y2) = (m * x1 + n, m * x2 + n);
        let result = interpolate_2d_f(x1, y1, x2, y2);
        assert_eq!(result[0], m);
        assert_eq!(result[1], n);
    }
}

