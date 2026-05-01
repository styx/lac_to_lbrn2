use crate::transform::Transform;

#[derive(Debug, Clone, PartialEq)]
pub enum SegmentKind {
    Move,
    Line,
    Bezier,
}

#[derive(Debug, Clone)]
pub struct Segment {
    pub kind: SegmentKind,
    pub params: Vec<f64>,
}

impl Segment {
    pub fn new(kind: SegmentKind, params: Vec<f64>) -> Self {
        Self { kind, params }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub c0: Option<[f64; 2]>,
    pub c1: Option<[f64; 2]>,
}

impl Vertex {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct Instance {
    pub obj: serde_json::Value,
    pub transform: Transform,
}

#[derive(Debug)]
pub struct ProcessParams {
    pub max_power: Option<f64>,
    pub speed: Option<f64>,
}
