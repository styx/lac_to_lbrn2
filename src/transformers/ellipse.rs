use crate::transformers::ShapeTransformer;
use crate::types::Instance;
use crate::utils::fnum;
use crate::xml_builder::XmlBuilder;

pub struct EllipseTransformer;

impl EllipseTransformer {
    pub fn new() -> Self {
        Self
    }
}

impl ShapeTransformer for EllipseTransformer {
    fn transform(
        &mut self,
        instance: &Instance,
        xml: &mut XmlBuilder,
        cut_index: usize,
        offset: (f64, f64),
    ) -> Vec<String> {
        let obj = &instance.obj;
        let t = &instance.transform;
        let (ox, oy) = offset;

        let center_x = obj["center_x"].as_f64().unwrap_or(0.0);
        let center_y = obj["center_y"].as_f64().unwrap_or(0.0);
        let radius_x = obj["radius_x"].as_f64().unwrap_or(0.0);
        let radius_y = obj["radius_y"].as_f64().unwrap_or(0.0);

        let (cx, cy) = t.apply(center_x, center_y);

        let cut_index_s = cut_index.to_string();
        let rx_s = fnum(radius_x);
        let ry_s = fnum(radius_y);

        xml.open(
            "Shape",
            &[
                ("Type", "Ellipse"),
                ("CutIndex", &cut_index_s),
                ("Rx", &rx_s),
                ("Ry", &ry_s),
            ],
        );
        xml.inline(
            "XForm",
            &format!("1 0 0 1 {} {}", fnum(cx - ox), fnum(cy - oy)),
        );
        xml.close("Shape");
        vec![]
    }
}
