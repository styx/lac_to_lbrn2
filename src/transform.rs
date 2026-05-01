#[derive(Debug, Clone)]
pub struct Transform {
    m: [f64; 6],
}

impl Transform {
    #[must_use]
    pub fn identity() -> Self {
        Self {
            m: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        }
    }

    #[must_use]
    pub fn parse(s: Option<&str>) -> Self {
        let input = s.unwrap_or("1 0 0 1 0 0");
        let vals: Vec<f64> = input
            .split_whitespace()
            .filter_map(|t| t.parse().ok())
            .collect();
        if vals.len() >= 6 {
            Self::from_array([vals[0], vals[1], vals[2], vals[3], vals[4], vals[5]])
        } else {
            Self::identity()
        }
    }

    #[must_use]
    pub fn from_array(arr: [f64; 6]) -> Self {
        Self { m: arr }
    }

    #[must_use]
    pub fn compose(&self, other: &Transform) -> Transform {
        let [a1, b1, c1, d1, tx1, ty1] = self.m;
        let [a2, b2, c2, d2, tx2, ty2] = other.m;
        Transform::from_array([
            a2 * a1 + c2 * b1,
            b2 * a1 + d2 * b1,
            a2 * c1 + c2 * d1,
            b2 * c1 + d2 * d1,
            a2 * tx1 + c2 * ty1 + tx2,
            b2 * tx1 + d2 * ty1 + ty2,
        ])
    }

    #[must_use]
    pub fn apply(&self, x: f64, y: f64) -> (f64, f64) {
        let [a, b, c, d, tx, ty] = self.m;
        (a * x + c * y + tx, b * x + d * y + ty)
    }

    pub fn scale_x(&self) -> f64 {
        self.m[0]
    }
    pub fn scale_y(&self) -> f64 {
        self.m[3]
    }
    pub fn tx(&self) -> f64 {
        self.m[4]
    }
    pub fn ty(&self) -> f64 {
        self.m[5]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    #[test]
    fn identity_is_no_op() {
        let (x, y) = Transform::identity().apply(3.0, 5.0);
        assert!(approx_eq(x, 3.0));
        assert!(approx_eq(y, 5.0));
    }

    #[test]
    fn parse_translation() {
        let t = Transform::parse(Some("1 0 0 1 5 10"));
        let (x, y) = t.apply(0.0, 0.0);
        assert!(approx_eq(x, 5.0));
        assert!(approx_eq(y, 10.0));
    }

    #[test]
    fn parse_none_returns_identity() {
        let t = Transform::parse(None);
        let (x, y) = t.apply(7.0, 3.0);
        assert!(approx_eq(x, 7.0));
        assert!(approx_eq(y, 3.0));
    }

    #[test]
    fn parse_non_numeric_falls_back_to_identity() {
        let t = Transform::parse(Some("not a transform"));
        let (x, y) = t.apply(1.0, 2.0);
        assert!(approx_eq(x, 1.0));
        assert!(approx_eq(y, 2.0));
    }

    #[test]
    fn parse_too_few_values_falls_back_to_identity() {
        let t = Transform::parse(Some("1 0 0 1 5"));
        let (x, y) = t.apply(1.0, 2.0);
        assert!(approx_eq(x, 1.0));
        assert!(approx_eq(y, 2.0));
    }

    #[test]
    fn compose_with_identity_leaves_transform_unchanged() {
        let t = Transform::parse(Some("2 0 0 3 4 5"));
        let id = Transform::identity();
        let (x1, y1) = t.compose(&id).apply(1.0, 1.0);
        let (x2, y2) = t.apply(1.0, 1.0);
        assert!(approx_eq(x1, x2));
        assert!(approx_eq(y1, y2));
    }

    #[test]
    fn compose_two_translations_adds_offsets() {
        // self applied first, then other: total translation = (3+5, 4+6) = (8, 10)
        let t1 = Transform::parse(Some("1 0 0 1 3 4"));
        let t2 = Transform::parse(Some("1 0 0 1 5 6"));
        let (x, y) = t1.compose(&t2).apply(0.0, 0.0);
        assert!(approx_eq(x, 8.0));
        assert!(approx_eq(y, 10.0));
    }

    #[test]
    fn scale_accessors_return_diagonal_elements() {
        let t = Transform::parse(Some("2 0 0 3 0 0"));
        assert!(approx_eq(t.scale_x(), 2.0));
        assert!(approx_eq(t.scale_y(), 3.0));
    }

    #[test]
    fn tx_ty_accessors_return_translation_components() {
        let t = Transform::parse(Some("1 0 0 1 7 8"));
        assert!(approx_eq(t.tx(), 7.0));
        assert!(approx_eq(t.ty(), 8.0));
    }

    #[test]
    fn apply_scaling_transform() {
        let t = Transform::parse(Some("2 0 0 3 0 0"));
        let (x, y) = t.apply(5.0, 4.0);
        assert!(approx_eq(x, 10.0)); // 2 * 5
        assert!(approx_eq(y, 12.0)); // 3 * 4
    }
}
