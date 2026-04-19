use crate::constants::{get_process_type_info, process_type_keys};
use crate::scene_builder::SceneBuilder;
use crate::types::ProcessParams;
use crate::visitors::cut_setting::CutSettingVisitor;
use crate::visitors::shape::ShapeVisitor;
use crate::xml_builder::XmlBuilder;
use crate::utils::id_str;
use serde_json::Value;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

pub struct Converter {
    input: String,
    output: String,
    normalize: bool,
}

impl Converter {
    pub fn new(input: String, output: String, normalize: bool) -> Self {
        Self {
            input,
            output,
            normalize,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let tmp = tmp_dir.path();

        self.extract(tmp)?;

        let json_path = tmp.join("2D/2dmodel.json");
        let settings_path = tmp.join("Metadata2D/project_settings.json");
        let objects_path = tmp.join("2D/Objects");

        let data: Value = serde_json::from_str(&std::fs::read_to_string(&json_path)?)?;
        let canvas = &data["canvas_list"][0];

        let mut obj_map: HashMap<String, Value> = HashMap::new();
        if let Some(arr) = canvas["obj_list"].as_array() {
            for o in arr {
                if let Some(id) = id_str(&o["obj_id"]) {
                    obj_map.insert(id, o.clone());
                }
            }
        }

        self.inject_process_types(&mut obj_map, &settings_path)?;
        let process_params = self.load_process_params(tmp, &settings_path)?;

        let scene = SceneBuilder::new(canvas, &obj_map);
        let instances = scene.build();

        let mut seen = std::collections::HashSet::new();
        let process_types: Vec<String> = instances
            .iter()
            .filter_map(|i| i.obj["process_type"].as_str().map(|s| s.to_string()))
            .filter(|pt| seen.insert(pt.clone()))
            .collect();

        let process_type_to_idx = self.build_index_map(&process_types);

        let mut process_type_to_obj: HashMap<String, Value> = HashMap::new();
        for inst in &instances {
            if let Some(pt) = inst.obj["process_type"].as_str() {
                process_type_to_obj
                    .entry(pt.to_string())
                    .or_insert_with(|| inst.obj.clone());
            }
        }

        let offset = if self.normalize {
            scene.compute_offset(&instances)
        } else {
            (0.0, 0.0)
        };

        let mut xml = XmlBuilder::new();
        xml.open(
            "LightBurnProject",
            &[
                ("AppVersion", "2.1.00"),
                ("DeviceName", "LaserPecker_LX1_Rotation"),
                ("FormatVersion", "1"),
                ("MaterialHeight", "0"),
                ("MirrorX", "False"),
                ("MirrorY", "True"),
                ("AskForSendName", "True"),
            ],
        );

        emit_header(&mut xml);

        CutSettingVisitor::new(
            &process_types,
            &process_type_to_obj,
            &process_type_to_idx,
            &process_params,
        )
        .visit(&mut xml);

        let objects_path_str = objects_path.to_string_lossy().to_string();
        ShapeVisitor::new(process_type_to_idx, objects_path_str)
            .visit(&instances, &mut xml, offset);

        xml.leaf("Notes", &[("ShowOnLoad", "0"), ("Notes", "")]);
        xml.close("LightBurnProject");

        std::fs::write(&self.output, xml.to_xml())?;
        println!("Done: {} objects → {}", instances.len(), self.output);

        Ok(())
    }

    fn extract(&self, tmp: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::open(&self.input)?;
        let mut archive = ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)?;
            let name = entry.name().to_string();

            if !name.starts_with("2D/") && !name.starts_with("Metadata2D/") {
                continue;
            }

            let dest = safe_join(tmp, &name);
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }
            if !entry.is_dir() {
                let mut f = std::fs::File::create(&dest)?;
                io::copy(&mut entry, &mut f)?;
            }
        }

        Ok(())
    }

    fn inject_process_types(
        &self,
        obj_map: &mut HashMap<String, Value>,
        settings_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !settings_path.exists() {
            return Ok(());
        }

        let settings: Value = serde_json::from_str(&std::fs::read_to_string(settings_path)?)?;

        let object_settings = settings
            .pointer("/canvas_settings/0/object_settings")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        for s in &object_settings {
            if let (Some(id), Some(pt)) = (id_str(&s["obj_id"]), s["process_type"].as_str()) {
                if let Some(obj) = obj_map.get_mut(&id) {
                    obj["process_type"] = Value::String(pt.to_string());
                }
            }
        }

        Ok(())
    }

    fn load_process_params(
        &self,
        tmp: &Path,
        settings_path: &Path,
    ) -> Result<HashMap<String, ProcessParams>, Box<dyn std::error::Error>> {
        let mut result: HashMap<String, ProcessParams> = HashMap::new();

        if !settings_path.exists() {
            return Ok(result);
        }

        let settings: Value = serde_json::from_str(&std::fs::read_to_string(settings_path)?)?;

        let batch_list = settings
            .pointer("/canvas_settings/0/making_batch_list")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let metadata_dir = tmp.join("Metadata2D");

        for batch in &batch_list {
            let material = batch["material_settings_name"]
                .as_str()
                .unwrap_or("")
                .to_string();
            if material.is_empty() {
                continue;
            }

            let prefix = format!("{} Process @", material);
            let config_path = match std::fs::read_dir(&metadata_dir) {
                Ok(entries) => entries.filter_map(|e| e.ok()).map(|e| e.path()).find(|p| {
                    p.file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| n.starts_with(&prefix) && n.ends_with(".config"))
                        .unwrap_or(false)
                }),
                Err(_) => continue,
            };

            let config_path = match config_path {
                Some(p) => p,
                None => continue,
            };

            let config: Value = serde_json::from_str(&std::fs::read_to_string(&config_path)?)?;

            for pt in process_type_keys() {
                if result.contains_key(pt) {
                    continue;
                }
                if let Some(section) = config.get(pt) {
                    result.insert(
                        pt.to_string(),
                        ProcessParams {
                            max_power: section["max_power"].as_f64(),
                            speed: section["speed"].as_f64(),
                        },
                    );
                }
            }
        }

        Ok(result)
    }

    fn build_index_map(&self, process_types: &[String]) -> HashMap<String, usize> {
        let mut map = HashMap::new();
        let mut next_free = 3usize;

        for pt in process_types {
            if let Some(info) = get_process_type_info(pt) {
                map.insert(pt.clone(), info.index);
            } else {
                map.insert(pt.clone(), next_free);
                next_free += 1;
            }
        }

        map
    }
}

fn emit_header(xml: &mut XmlBuilder) {
    xml.open("VariableText", &[]);
    for (name, value) in [
        ("Start", "0"),
        ("End", "999"),
        ("Current", "0"),
        ("Increment", "1"),
        ("AutoAdvance", "0"),
    ] {
        xml.leaf(name, &[("Value", value)]);
    }
    xml.close("VariableText");

    xml.open("UIPrefs", &[]);
    for (name, value) in [
        ("Optimize_ByLayer", "0"),
        ("Optimize_ByGroup", "-1"),
        ("Optimize_ByPriority", "1"),
        ("Optimize_WhichDirection", "0"),
        ("Optimize_InnerToOuter", "1"),
        ("Optimize_ByDirection", "0"),
        ("Optimize_ReduceTravel", "1"),
        ("Optimize_HideBacklash", "0"),
        ("Optimize_ReduceDirChanges", "0"),
        ("Optimize_ChooseCorners", "0"),
        ("Optimize_AllowReverse", "1"),
        ("Optimize_RemoveOverlaps", "0"),
        ("Optimize_OptimalEntryPoint", "0"),
        ("Optimize_OverlapDist", "0.025"),
    ] {
        xml.leaf(name, &[("Value", value)]);
    }
    xml.close("UIPrefs");
}

fn safe_join(base: &Path, name: &str) -> PathBuf {
    let mut path = base.to_path_buf();
    for component in name.split('/') {
        if !component.is_empty() && component != ".." {
            path.push(component);
        }
    }
    path
}
