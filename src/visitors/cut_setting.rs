use crate::constants::{get_dither, get_process_type_info};
use crate::types::ProcessParams;
use crate::xml_builder::XmlBuilder;
use serde_json::Value;
use std::collections::HashMap;

pub struct CutSettingVisitor<'a> {
    process_types: &'a [String],
    process_type_to_obj: &'a HashMap<String, Value>,
    process_type_to_idx: &'a HashMap<String, usize>,
    process_params: &'a HashMap<String, ProcessParams>,
}

impl<'a> CutSettingVisitor<'a> {
    pub fn new(
        process_types: &'a [String],
        process_type_to_obj: &'a HashMap<String, Value>,
        process_type_to_idx: &'a HashMap<String, usize>,
        process_params: &'a HashMap<String, ProcessParams>,
    ) -> Self {
        Self {
            process_types,
            process_type_to_obj,
            process_type_to_idx,
            process_params,
        }
    }

    pub fn visit(&self, xml: &mut XmlBuilder) {
        let mut sorted = self.process_types.to_vec();
        sorted.sort_by_key(|pt| {
            self.process_type_to_idx
                .get(pt.as_str())
                .copied()
                .unwrap_or(99)
        });

        for process_type in &sorted {
            let obj = self.process_type_to_obj.get(process_type.as_str());
            let idx = self
                .process_type_to_idx
                .get(process_type.as_str())
                .copied()
                .unwrap_or(0);

            let is_raster = obj
                .map(|o| o["type"].as_str() == Some("RasterImage"))
                .unwrap_or(false);

            if is_raster {
                if let Some(o) = obj {
                    self.emit_image(xml, o, idx, process_type);
                }
            } else {
                self.emit_cut(xml, process_type, idx);
            }
        }
    }

    fn emit_image(&self, xml: &mut XmlBuilder, obj: &Value, idx: usize, process_type: &str) {
        let settings = &obj["image_settings"];
        let filtering = settings["filtering_type"].as_str().unwrap_or("");
        let dither = get_dither(filtering);
        let name = format!("C{:02}", idx);
        let params = self.process_params.get(process_type);
        let max_power = fmt_param(params.and_then(|p| p.max_power));
        let speed = fmt_param(params.and_then(|p| p.speed));
        let idx_s = idx.to_string();

        xml.open("CutSetting_Img", &[("type", "Image")]);
        xml.leaf("index", &[("Value", &idx_s)]);
        xml.leaf("name", &[("Value", &name)]);
        xml.leaf("maxPower", &[("Value", &max_power)]);
        xml.leaf("maxPower2", &[("Value", &max_power)]);
        xml.leaf("speed", &[("Value", &speed)]);
        xml.leaf("priority", &[("Value", &idx_s)]);
        xml.leaf("ditherMode", &[("Value", dither)]);
        xml.close("CutSetting_Img");
    }

    fn emit_cut(&self, xml: &mut XmlBuilder, process_type: &str, idx: usize) {
        let (kind, name) = if let Some(info) = get_process_type_info(process_type) {
            (info.kind, info.name.to_string())
        } else {
            ("Cut", format!("C{:02}", idx))
        };

        let params = self.process_params.get(process_type);
        let max_power = fmt_param(params.and_then(|p| p.max_power));
        let speed = fmt_param(params.and_then(|p| p.speed));
        let idx_s = idx.to_string();

        xml.open("CutSetting", &[("type", kind)]);
        xml.leaf("index", &[("Value", &idx_s)]);
        xml.leaf("name", &[("Value", &name)]);
        xml.leaf("maxPower", &[("Value", &max_power)]);
        xml.leaf("maxPower2", &[("Value", &max_power)]);
        xml.leaf("speed", &[("Value", &speed)]);
        xml.leaf("priority", &[("Value", &idx_s)]);
        xml.close("CutSetting");
    }
}

fn fmt_param(v: Option<f64>) -> String {
    v.map(|n| {
        if n == n.floor() && n.abs() < 1e15 {
            format!("{}", n as i64)
        } else {
            n.to_string()
        }
    })
    .unwrap_or_else(|| "0".to_string())
}
