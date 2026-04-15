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
