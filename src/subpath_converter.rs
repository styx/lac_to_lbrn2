use crate::transform::Transform;
use crate::types::{Segment, SegmentKind, Vertex};
use crate::utils::fnum;

pub fn convert(
    segments: &[Segment],
    is_closed: bool,
    transform: &Transform,
    ox: f64,
    oy: f64,
) -> Option<(String, String)> {
    if segments.len() < 2 {
        return None;
    }

    let mut vertices: Vec<Vertex> = Vec::new();
    let mut prims: Vec<String> = Vec::new();

    let (sx, sy) = transform.apply(segments[0].params[0], segments[0].params[1]);
    vertices.push(Vertex::new(sx - ox, sy - oy));

    for seg in segments.iter().skip(1) {
        let vi = vertices.len();

        match seg.kind {
            SegmentKind::Line => {
                let (x, y) = transform.apply(seg.params[0], seg.params[1]);
                vertices.last_mut().expect("at least one vertex pushed before loop").c0 = None;
                vertices.push(Vertex::new(x - ox, y - oy));
                prims.push(format!("L{} {}", vi - 1, vi));
            }
            SegmentKind::Bezier => {
                let (cp1x, cp1y) = transform.apply(seg.params[0], seg.params[1]);
                let (cp2x, cp2y) = transform.apply(seg.params[2], seg.params[3]);
                let (ex, ey) = transform.apply(seg.params[4], seg.params[5]);
                vertices.last_mut().expect("at least one vertex pushed before loop").c0 = Some([cp1x - ox, cp1y - oy]);
                let mut new_v = Vertex::new(ex - ox, ey - oy);
                new_v.c1 = Some([cp2x - ox, cp2y - oy]);
                vertices.push(new_v);
                prims.push(format!("B{} {}", vi - 1, vi));
            }
            SegmentKind::Move => {}
        }
    }

    if is_closed && vertices.len() > 1 {
        vertices.last_mut().expect("len > 1 guaranteed by branch condition").c0 = None;
        vertices.first_mut().expect("len > 1 guaranteed by branch condition").c1 = None;
        prims.push(format!("L{} 0", vertices.len() - 1));
    }

    let vert_list: String = vertices.iter().map(fmt_vertex).collect();
    let prim_list: String = prims.join("");

    if prim_list.is_empty() {
        return None;
    }

    Some((vert_list, prim_list))
}

fn fmt_vertex(v: &Vertex) -> String {
    let mut s = format!("V{} {}", fnum(v.x), fnum(v.y));
    match v.c0 {
        Some([cx, cy]) => s.push_str(&format!("c0x{}c0y{}", fnum(cx), fnum(cy))),
        None => s.push_str("c0x1"),
    }
    match v.c1 {
        Some([cx, cy]) => s.push_str(&format!("c1x{}c1y{}", fnum(cx), fnum(cy))),
        None => s.push_str("c1x1"),
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform::Transform;
    use crate::types::{Segment, SegmentKind};

    fn move_seg(x: f64, y: f64) -> Segment {
        Segment::new(SegmentKind::Move, vec![x, y])
    }

    fn line_seg(x: f64, y: f64) -> Segment {
        Segment::new(SegmentKind::Line, vec![x, y])
    }

    fn bezier_seg(cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, ex: f64, ey: f64) -> Segment {
        Segment::new(SegmentKind::Bezier, vec![cp1x, cp1y, cp2x, cp2y, ex, ey])
    }

    #[test]
    fn empty_segments_returns_none() {
        assert!(convert(&[], false, &Transform::identity(), 0.0, 0.0).is_none());
    }

    #[test]
    fn single_segment_returns_none() {
        // Need at least a Move + one other segment to emit a primitive
        let segs = vec![move_seg(0.0, 0.0)];
        assert!(convert(&segs, false, &Transform::identity(), 0.0, 0.0).is_none());
    }

    #[test]
    fn move_plus_line_produces_l_primitive() {
        let segs = vec![move_seg(0.0, 0.0), line_seg(10.0, 0.0)];
        let (verts, prims) = convert(&segs, false, &Transform::identity(), 0.0, 0.0).unwrap();
        assert!(verts.contains("V0 0"));
        assert!(verts.contains("V10 0"));
        assert!(prims.contains("L0 1"));
    }

    #[test]
    fn closed_path_appends_closing_primitive() {
        let segs = vec![move_seg(0.0, 0.0), line_seg(10.0, 0.0)];
        let (_, prims) = convert(&segs, true, &Transform::identity(), 0.0, 0.0).unwrap();
        // Closing line from last vertex back to vertex 0
        assert!(prims.contains("L1 0"));
    }

    #[test]
    fn offset_is_subtracted_from_all_coordinates() {
        let segs = vec![move_seg(5.0, 10.0), line_seg(15.0, 20.0)];
        let (verts, _) = convert(&segs, false, &Transform::identity(), 5.0, 10.0).unwrap();
        assert!(verts.contains("V0 0"));   // (5-5, 10-10)
        assert!(verts.contains("V10 10")); // (15-5, 20-10)
    }

    #[test]
    fn bezier_segment_produces_b_primitive() {
        let segs = vec![
            move_seg(0.0, 0.0),
            bezier_seg(1.0, 0.0, 2.0, 1.0, 3.0, 0.0),
        ];
        let (_, prims) = convert(&segs, false, &Transform::identity(), 0.0, 0.0).unwrap();
        assert!(prims.contains("B0 1"));
    }

    #[test]
    fn transform_is_applied_to_coordinates() {
        // Scale x2 in x, x3 in y
        let t = Transform::parse(Some("2 0 0 3 0 0"));
        let segs = vec![move_seg(1.0, 1.0), line_seg(2.0, 2.0)];
        let (verts, _) = convert(&segs, false, &t, 0.0, 0.0).unwrap();
        assert!(verts.contains("V2 3"));  // start: (1*2, 1*3)
        assert!(verts.contains("V4 6"));  // end:   (2*2, 2*3)
    }
}
