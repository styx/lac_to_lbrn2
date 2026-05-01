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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SegmentKind;

    #[test]
    fn empty_input_returns_empty() {
        assert!(parse("").is_empty());
    }

    #[test]
    fn lone_m_with_no_coordinates_returns_empty() {
        // The parser breaks immediately when fewer than 2 tokens follow M
        assert!(parse("M").is_empty());
    }

    #[test]
    fn move_command_creates_one_subpath_with_one_segment() {
        let result = parse("M 10 20");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0].kind, SegmentKind::Move);
        assert_eq!(result[0][0].params, vec![10.0, 20.0]);
    }

    #[test]
    fn line_command_appends_segment_to_current_subpath() {
        let result = parse("M 0 0 L 10 20");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[0][1].kind, SegmentKind::Line);
        assert_eq!(result[0][1].params, vec![10.0, 20.0]);
    }

    #[test]
    fn bezier_command_stores_params_as_cp1_cp2_endpoint() {
        // C reads: ex ey cp1x cp1y cp2x cp2y, stores as [cp1x, cp1y, cp2x, cp2y, ex, ey]
        let result = parse("M 0 0 C 1 2 3 4 5 6");
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[0][1].kind, SegmentKind::Bezier);
        assert_eq!(result[0][1].params, vec![3.0, 4.0, 5.0, 6.0, 1.0, 2.0]);
    }

    #[test]
    fn close_command_consumed_without_adding_segment() {
        let result = parse("M 0 0 L 10 10 Z");
        assert_eq!(result[0].len(), 2); // only Move + Line
    }

    #[test]
    fn lowercase_z_also_consumed() {
        let result = parse("M 0 0 L 5 5 z");
        assert_eq!(result[0].len(), 2);
    }

    #[test]
    fn second_move_starts_new_subpath() {
        let result = parse("M 0 0 L 5 5 M 10 10 L 15 15");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0][0].params, vec![0.0, 0.0]);
        assert_eq!(result[1][0].params, vec![10.0, 10.0]);
    }

    #[test]
    fn negative_coordinates_parsed_correctly() {
        let result = parse("M -5 -3");
        assert_eq!(result[0][0].params, vec![-5.0, -3.0]);
    }

    #[test]
    fn scientific_notation_parsed_correctly() {
        let result = parse("M 1.5e+2 -3.14e-1");
        assert!((result[0][0].params[0] - 150.0).abs() < 1e-9);
        assert!((result[0][0].params[1] - (-0.314)).abs() < 1e-9);
    }

    #[test]
    fn implicit_lines_after_move_coordinates() {
        // Additional coordinate pairs after M become L segments
        let result = parse("M 0 0 10 20");
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[0][1].kind, SegmentKind::Line);
        assert_eq!(result[0][1].params, vec![10.0, 20.0]);
    }

    #[test]
    fn multiple_line_pairs_all_parsed() {
        let result = parse("M 0 0 L 1 2 3 4 5 6");
        assert_eq!(result[0].len(), 4); // Move + 3 Lines
    }

    #[test]
    fn truncated_bezier_with_only_4_tokens_skipped() {
        // Only 4 tokens after C — not enough for a full bezier (need 6); loop skips it
        let result = parse("M 0 0 C 1 2 3 4");
        // bezier loop condition i+5 < len is not met → no bezier added
        assert_eq!(result[0].len(), 1); // just the Move
    }
}
