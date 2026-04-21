use crate::types::Instance;
use crate::utils::fnum;
use crate::xml_builder::XmlBuilder;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::path::Path;

pub struct RasterImageTransformer {
    objects_path: String,
}

impl RasterImageTransformer {
    pub fn new(objects_path: String) -> Self {
        Self { objects_path }
    }

    pub fn transform(
        &self,
        instance: &Instance,
        xml: &mut XmlBuilder,
        cut_index: usize,
        offset: (f64, f64),
    ) {
        let obj = &instance.obj;
        let t = &instance.transform;
        let (ox, oy) = offset;

        let width = obj["width"].as_f64().unwrap_or(0.0);
        let height = obj["height"].as_f64().unwrap_or(0.0);
        let w_mm = width * t.scale_x().abs();
        let h_mm = height * t.scale_y().abs();
        let cx = t.tx() - ox;
        let cy = t.ty() - oy;

        let file_name = obj["file_name"].as_str().unwrap_or("");
        let bare = Path::new(file_name)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if bare.is_empty() {
            eprintln!("Warning: invalid file_name in object, skipping");
            return;
        }
        let png_path = Path::new(&self.objects_path).join(bare);

        let bytes = match std::fs::read(&png_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Warning: could not read {}: {}", png_path.display(), e);
                return;
            }
        };
        let b64 = STANDARD.encode(&bytes);

        let settings = &obj["image_settings"];
        let contrast = settings["contrast_adjust"].as_f64().unwrap_or(0.0) / 100.0;
        let bright = settings["brightness_adjust"].as_f64().unwrap_or(0.0) / 100.0;
        let enhance = settings["sharpness_adjust"].as_f64().unwrap_or(0.0);

        let cut_index_s = cut_index.to_string();
        let w_s = fnum(w_mm);
        let h_s = fnum(h_mm);
        let contrast_s = fnum(contrast);
        let bright_s = fnum(bright);
        let enhance_s = fnum(enhance);
        let file_s = png_path.to_string_lossy().to_string();

        xml.open(
            "Shape",
            &[
                ("Type", "Bitmap"),
                ("CutIndex", &cut_index_s),
                ("W", &w_s),
                ("H", &h_s),
                ("Gamma", "1"),
                ("Contrast", &contrast_s),
                ("Brightness", &bright_s),
                ("EnhanceAmount", &enhance_s),
                ("EnhanceRadius", "0"),
                ("EnhanceDenoise", "0"),
                ("File", &file_s),
                ("SourceHash", "0"),
                ("Data", &b64),
            ],
        );
        xml.inline("XForm", &format!("1 0 0 1 {} {}", fnum(cx), fnum(cy)));
        xml.close("Shape");
    }
}
