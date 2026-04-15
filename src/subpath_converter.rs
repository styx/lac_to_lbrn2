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
                vertices.last_mut().unwrap().c0 = None;
                vertices.push(Vertex::new(x - ox, y - oy));
                prims.push(format!("L{} {}", vi - 1, vi));
            }
            SegmentKind::Bezier => {
                let (cp1x, cp1y) = transform.apply(seg.params[0], seg.params[1]);
                let (cp2x, cp2y) = transform.apply(seg.params[2], seg.params[3]);
                let (ex, ey) = transform.apply(seg.params[4], seg.params[5]);
                vertices.last_mut().unwrap().c0 = Some([cp1x - ox, cp1y - oy]);
                let mut new_v = Vertex::new(ex - ox, ey - oy);
                new_v.c1 = Some([cp2x - ox, cp2y - oy]);
                vertices.push(new_v);
                prims.push(format!("B{} {}", vi - 1, vi));
            }
            SegmentKind::Move => {}
        }
    }

    if is_closed && vertices.len() > 1 {
        vertices.last_mut().unwrap().c0 = None;
        vertices.first_mut().unwrap().c1 = None;
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
