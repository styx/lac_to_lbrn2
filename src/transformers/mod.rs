pub mod ellipse;
pub mod path;
pub mod raster_image;

use crate::types::Instance;
use crate::xml_builder::XmlBuilder;

pub trait ShapeTransformer {
    fn transform(
        &mut self,
        instance: &Instance,
        xml: &mut XmlBuilder,
        cut_index: usize,
        offset: (f64, f64),
    ) -> Vec<String>;
}
