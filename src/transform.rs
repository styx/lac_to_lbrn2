#[derive(Debug, Clone)]
pub struct Transform {
    m: [f64; 6],
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            m: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        }
    }

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

    pub fn from_array(arr: [f64; 6]) -> Self {
        Self { m: arr }
    }

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
