#[must_use]
pub fn id_str(v: &serde_json::Value) -> Option<String> {
    if let Some(s) = v.as_str() {
        return Some(s.to_string());
    }
    if let Some(n) = v.as_u64() {
        return Some(n.to_string());
    }
    if let Some(n) = v.as_i64() {
        return Some(n.to_string());
    }
    None
}

#[must_use]
pub fn fnum(n: f64) -> String {
    if n == 0.0 {
        return "0".to_string();
    }

    let abs = n.abs();
    let exp = abs.log10().floor() as i32;

    if (-4..10).contains(&exp) {
        let decimal_places = (9 - exp).max(0) as usize;
        let s = format!("{:.prec$}", n, prec = decimal_places);
        if decimal_places > 0 && s.contains('.') {
            s.trim_end_matches('0').trim_end_matches('.').to_string()
        } else {
            s
        }
    } else {
        let s = format!("{:.9e}", n);
        let parts: Vec<&str> = s.splitn(2, 'e').collect();
        if parts.len() == 2 {
            let mantissa = parts[0].trim_end_matches('0').trim_end_matches('.');
            let exponent: i32 = parts[1].parse().unwrap_or(0);
            if exponent >= 0 {
                format!("{}e+{}", mantissa, exponent)
            } else {
                format!("{}e{}", mantissa, exponent)
            }
        } else {
            s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn fnum_zero() {
        assert_eq!(fnum(0.0), "0");
    }

    #[test]
    fn fnum_one() {
        assert_eq!(fnum(1.0), "1");
    }

    #[test]
    fn fnum_decimal() {
        assert_eq!(fnum(1.5), "1.5");
    }

    #[test]
    fn fnum_negative() {
        assert_eq!(fnum(-3.14), "-3.14");
    }

    #[test]
    fn fnum_trailing_zeros_trimmed() {
        assert_eq!(fnum(1.10), "1.1");
    }

    #[test]
    fn fnum_integer_hundreds() {
        assert_eq!(fnum(100.0), "100");
    }

    #[test]
    fn fnum_large_uses_scientific_positive_exponent() {
        assert_eq!(fnum(1e10), "1e+10");
    }

    #[test]
    fn fnum_large_negative_uses_scientific() {
        assert_eq!(fnum(-1e10), "-1e+10");
    }

    #[test]
    fn fnum_small_uses_scientific_negative_exponent() {
        assert_eq!(fnum(1e-5), "1e-5");
    }

    #[test]
    fn fnum_boundary_1e_neg4_uses_decimal() {
        // exp = -4 is the inclusive lower bound of (-4..10), so decimal format is used
        assert_eq!(fnum(1e-4), "0.0001");
    }

    #[test]
    fn fnum_boundary_1e9_uses_decimal() {
        // exp = 9 is still inside (-4..10), so no scientific notation
        assert_eq!(fnum(1e9), "1000000000");
    }

    #[test]
    fn id_str_from_string_value() {
        assert_eq!(id_str(&json!("abc")), Some("abc".to_string()));
    }

    #[test]
    fn id_str_from_unsigned_integer() {
        assert_eq!(id_str(&json!(42u64)), Some("42".to_string()));
    }

    #[test]
    fn id_str_from_negative_integer() {
        let v = serde_json::Value::Number(serde_json::Number::from(-1i64));
        assert_eq!(id_str(&v), Some("-1".to_string()));
    }

    #[test]
    fn id_str_from_null_returns_none() {
        assert_eq!(id_str(&json!(null)), None);
    }

    #[test]
    fn id_str_from_bool_returns_none() {
        assert_eq!(id_str(&json!(true)), None);
    }

    #[test]
    fn id_str_from_array_returns_none() {
        assert_eq!(id_str(&json!([])), None);
    }
}
