use crate::types::{Segment, SegmentKind};

pub fn parse(path_data: &str) -> Vec<Vec<Segment>> {
    let tokens = tokenize(path_data);
    let mut subpaths: Vec<Vec<Segment>> = Vec::new();
    let mut current: Option<Vec<Segment>> = None;
    let mut i = 0;

    while i < tokens.len() {
        match tokens[i].as_str() {
            "M" => {
                i += 1;
                if i + 1 >= tokens.len() {
                    break;
                }
                let x = tokens[i].parse::<f64>().unwrap_or(0.0);
                i += 1;
                let y = tokens[i].parse::<f64>().unwrap_or(0.0);
                i += 1;
                if let Some(c) = current.take() {
                    subpaths.push(c);
                }
                let mut segs = vec![Segment::new(SegmentKind::Move, vec![x, y])];
                while i + 1 < tokens.len() && !is_cmd(tokens[i].as_str()) {
                    let lx = tokens[i].parse::<f64>().unwrap_or(0.0);
                    i += 1;
                    let ly = tokens[i].parse::<f64>().unwrap_or(0.0);
                    i += 1;
                    segs.push(Segment::new(SegmentKind::Line, vec![lx, ly]));
                }
                current = Some(segs);
            }
            "C" => {
                i += 1;
                while i + 5 < tokens.len() && !is_cmd(tokens[i].as_str()) {
                    let ex = tokens[i].parse::<f64>().unwrap_or(0.0);
                    i += 1;
                    let ey = tokens[i].parse::<f64>().unwrap_or(0.0);
                    i += 1;
                    let cp1x = tokens[i].parse::<f64>().unwrap_or(0.0);
                    i += 1;
                    let cp1y = tokens[i].parse::<f64>().unwrap_or(0.0);
                    i += 1;
                    let cp2x = tokens[i].parse::<f64>().unwrap_or(0.0);
                    i += 1;
                    let cp2y = tokens[i].parse::<f64>().unwrap_or(0.0);
                    i += 1;
                    if let Some(ref mut segs) = current {
                        segs.push(Segment::new(
                            SegmentKind::Bezier,
                            vec![cp1x, cp1y, cp2x, cp2y, ex, ey],
                        ));
                    }
                }
            }
            "L" => {
                i += 1;
                while i + 1 < tokens.len() && !is_cmd(tokens[i].as_str()) {
                    let lx = tokens[i].parse::<f64>().unwrap_or(0.0);
                    i += 1;
                    let ly = tokens[i].parse::<f64>().unwrap_or(0.0);
                    i += 1;
                    if let Some(ref mut segs) = current {
                        segs.push(Segment::new(SegmentKind::Line, vec![lx, ly]));
                    }
                }
            }
            "Z" | "z" => {
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    if let Some(c) = current {
        subpaths.push(c);
    }

    subpaths
}

fn tokenize(s: &str) -> Vec<String> {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < len {
        let c = bytes[i] as char;
        if matches!(c, 'M' | 'C' | 'L' | 'Z' | 'z') {
            tokens.push(c.to_string());
            i += 1;
        } else if c == '-' || c.is_ascii_digit() || c == '.' {
            let start = i;
            if c == '-' {
                i += 1;
            }
            while i < len && (bytes[i] as char).is_ascii_digit() {
                i += 1;
            }
            if i < len && bytes[i] == b'.' {
                i += 1;
                while i < len && (bytes[i] as char).is_ascii_digit() {
                    i += 1;
                }
            }
            if i < len && (bytes[i] == b'e' || bytes[i] == b'E') {
                i += 1;
                if i < len && (bytes[i] == b'+' || bytes[i] == b'-') {
                    i += 1;
                }
                while i < len && (bytes[i] as char).is_ascii_digit() {
                    i += 1;
                }
            }
            let tok = &s[start..i];
            if tok != "-" && !tok.is_empty() {
                tokens.push(tok.to_string());
            }
        } else {
            i += 1;
        }
    }

    tokens
}

fn is_cmd(s: &str) -> bool {
    matches!(s, "M" | "C" | "L" | "Z" | "z")
}
