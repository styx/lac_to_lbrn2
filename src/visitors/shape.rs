use crate::transformers::ellipse::EllipseTransformer;
use crate::transformers::path::PathTransformer;
use crate::transformers::raster_image::RasterImageTransformer;
use crate::transformers::ShapeTransformer;
use crate::types::Instance;
use crate::xml_builder::XmlBuilder;
use std::collections::HashMap;

pub struct ShapeVisitor {
    process_type_to_idx: HashMap<String, usize>,
    transformers: HashMap<&'static str, Box<dyn ShapeTransformer>>,
}

impl ShapeVisitor {
    pub fn new(process_type_to_idx: HashMap<String, usize>, objects_path: String) -> Self {
        let mut transformers: HashMap<&'static str, Box<dyn ShapeTransformer>> = HashMap::new();
        transformers.insert("PathObject", Box::new(PathTransformer::new()));
        transformers.insert("EllipseObject", Box::new(EllipseTransformer::new()));
        transformers.insert(
            "RasterImage",
            Box::new(RasterImageTransformer::new(objects_path)),
        );
        Self {
            process_type_to_idx,
            transformers,
        }
    }

    pub fn visit(
        &mut self,
        instances: &[Instance],
        xml: &mut XmlBuilder,
        offset: (f64, f64),
    ) -> Vec<String> {
        let mut warnings = Vec::new();
        for inst in instances {
            let cut_index = inst.obj["process_type"]
                .as_str()
                .and_then(|pt| self.process_type_to_idx.get(pt).copied())
                .unwrap_or(0);

            if let Some(obj_type) = inst.obj["type"].as_str() {
                if let Some(transformer) = self.transformers.get_mut(obj_type) {
                    warnings.extend(transformer.transform(inst, xml, cut_index, offset));
                }
            }
        }
        warnings
    }
}
