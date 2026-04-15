use crate::transformers::ellipse::EllipseTransformer;
use crate::transformers::path::PathTransformer;
use crate::transformers::raster_image::RasterImageTransformer;
use crate::types::Instance;
use crate::xml_builder::XmlBuilder;
use std::collections::HashMap;

pub struct ShapeVisitor {
    process_type_to_idx: HashMap<String, usize>,
    path_transformer: PathTransformer,
    ellipse_transformer: EllipseTransformer,
    raster_image_transformer: RasterImageTransformer,
}

impl ShapeVisitor {
    pub fn new(process_type_to_idx: HashMap<String, usize>, objects_path: String) -> Self {
        Self {
            process_type_to_idx,
            path_transformer: PathTransformer::new(),
            ellipse_transformer: EllipseTransformer::new(),
            raster_image_transformer: RasterImageTransformer::new(objects_path),
        }
    }

    pub fn visit(&mut self, instances: &[Instance], xml: &mut XmlBuilder, offset: (f64, f64)) {
        for inst in instances {
            let cut_index = inst.obj["process_type"]
                .as_str()
                .and_then(|pt| self.process_type_to_idx.get(pt).copied())
                .unwrap_or(0);

            match inst.obj["type"].as_str() {
                Some("PathObject") => {
                    self.path_transformer.transform(inst, xml, cut_index, offset);
                }
                Some("EllipseObject") => {
                    self.ellipse_transformer.transform(inst, xml, cut_index, offset);
                }
                Some("RasterImage") => {
                    self.raster_image_transformer
                        .transform(inst, xml, cut_index, offset);
                }
                _ => {}
            }
        }
    }
}
