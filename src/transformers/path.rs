use crate::path_parser;
use crate::subpath_converter;
use crate::types::Instance;
use crate::xml_builder::XmlBuilder;

pub struct PathTransformer {
    vert_id: usize,
    prim_id: usize,
}

impl PathTransformer {
    pub fn new() -> Self {
        Self {
            vert_id: 0,
            prim_id: 0,
        }
    }

    pub fn transform(
        &mut self,
        instance: &Instance,
        xml: &mut XmlBuilder,
        cut_index: usize,
        offset: (f64, f64),
    ) {
        let obj = &instance.obj;
        let t = &instance.transform;
        let (ox, oy) = offset;

        let path_data = match obj["path_data"].as_str() {
            Some(s) => s,
            None => return,
        };
        let is_closed = obj["is_closed"].as_bool().unwrap_or(false);

        for segs in path_parser::parse(path_data) {
            if let Some((vert_list, prim_list)) =
                subpath_converter::convert(&segs, is_closed, t, ox, oy)
            {
                xml.open(
                    "Shape",
                    &[
                        ("Type", "Path"),
                        ("CutIndex", &cut_index.to_string()),
                        ("VertID", &self.vert_id.to_string()),
                        ("PrimID", &self.prim_id.to_string()),
                    ],
                );
                xml.inline("XForm", "1 0 0 1 0 0");
                xml.inline("VertList", &vert_list);
                xml.inline("PrimList", &prim_list);
                xml.close("Shape");
                self.vert_id += 1;
                self.prim_id += 1;
            }
        }
    }
}
