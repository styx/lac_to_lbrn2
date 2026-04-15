use crate::path_parser;
use crate::transform::Transform;
use crate::types::{Instance, SegmentKind};
use serde_json::Value;
use std::collections::HashMap;

fn id_str(v: &Value) -> Option<String> {
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

pub struct SceneBuilder<'a> {
    canvas: &'a Value,
    obj_map: &'a HashMap<String, Value>,
}

impl<'a> SceneBuilder<'a> {
    pub fn new(canvas: &'a Value, obj_map: &'a HashMap<String, Value>) -> Self {
        Self { canvas, obj_map }
    }

    pub fn build(&self) -> Vec<Instance> {
        let mut instances = Vec::new();
        if let Some(components) = self.canvas["components"].as_array() {
            for comp in components {
                self.walk(comp, &Transform::identity(), &mut instances);
            }
        }
        instances
    }

    pub fn compute_offset(&self, instances: &[Instance]) -> (f64, f64) {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;

        for inst in instances {
            let obj = &inst.obj;
            let t = &inst.transform;

            match obj["type"].as_str() {
                Some("PathObject") => {
                    if let Some(path_data) = obj["path_data"].as_str() {
                        for segs in path_parser::parse(path_data) {
                            for seg in &segs {
                                match seg.kind {
                                    SegmentKind::Move | SegmentKind::Line => {
                                        let (wx, wy) = t.apply(seg.params[0], seg.params[1]);
                                        min_x = min_x.min(wx);
                                        min_y = min_y.min(wy);
                                    }
                                    SegmentKind::Bezier => {
                                        for (xi, yi) in [(0usize, 1usize), (2, 3), (4, 5)] {
                                            let (wx, wy) = t.apply(seg.params[xi], seg.params[yi]);
                                            min_x = min_x.min(wx);
                                            min_y = min_y.min(wy);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Some("EllipseObject") => {
                    let cx = obj["center_x"].as_f64().unwrap_or(0.0);
                    let cy = obj["center_y"].as_f64().unwrap_or(0.0);
                    let rx = obj["radius_x"].as_f64().unwrap_or(0.0);
                    let ry = obj["radius_y"].as_f64().unwrap_or(0.0);
                    let (wx, wy) = t.apply(cx, cy);
                    min_x = min_x.min(wx - rx);
                    min_y = min_y.min(wy - ry);
                }
                Some("RasterImage") => {
                    let width = obj["width"].as_f64().unwrap_or(0.0);
                    let height = obj["height"].as_f64().unwrap_or(0.0);
                    let w_mm = width * t.scale_x().abs();
                    let h_mm = height * t.scale_y().abs();
                    min_x = min_x.min(t.tx() - w_mm / 2.0);
                    min_y = min_y.min(t.ty() - h_mm / 2.0);
                }
                _ => {}
            }
        }

        (
            if min_x.is_finite() { min_x } else { 0.0 },
            if min_y.is_finite() { min_y } else { 0.0 },
        )
    }

    fn walk(&self, comp: &Value, parent_transform: &Transform, instances: &mut Vec<Instance>) {
        let obj_id = match id_str(&comp["obj_id"]) {
            Some(id) => id,
            None => return,
        };
        let obj = match self.obj_map.get(&obj_id) {
            Some(o) => o,
            None => return,
        };

        let comp_transform = Transform::parse(comp["transform"].as_str());
        let total = parent_transform.compose(&comp_transform);

        const LEAF_TYPES: &[&str] = &["PathObject", "EllipseObject", "RasterImage"];

        match obj["type"].as_str() {
            Some("AttachedGroup") => {
                if let Some(children) = obj["components"].as_array() {
                    for child in children {
                        self.walk(child, &total, instances);
                    }
                }
            }
            Some(t) if LEAF_TYPES.contains(&t) => {
                if !obj["color"].is_null() {
                    instances.push(Instance {
                        obj: obj.clone(),
                        transform: total,
                    });
                }
            }
            _ => {}
        }
    }
}
