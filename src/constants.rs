pub struct ProcessTypeInfo {
    pub kind: &'static str,
    pub name: &'static str,
    pub index: usize,
}

static PROCESS_TYPE_MAP: &[(&str, ProcessTypeInfo)] = &[
    (
        "LaserLineEngrave",
        ProcessTypeInfo {
            kind: "Cut",
            name: "Line",
            index: 0,
        },
    ),
    (
        "LaserFillEngrave",
        ProcessTypeInfo {
            kind: "Scan",
            name: "Fill",
            index: 1,
        },
    ),
    (
        "LaserLineCut",
        ProcessTypeInfo {
            kind: "Cut",
            name: "Cut",
            index: 2,
        },
    ),
];

static DITHER_MAP: &[(&str, &str)] = &[
    ("IF_Relief", "stucki"),
    ("IF_Threshold", "threshold"),
    ("IF_Ordered", "ordered"),
    ("IF_Dither", "floyd"),
];

pub fn get_process_type_info(pt: &str) -> Option<&'static ProcessTypeInfo> {
    PROCESS_TYPE_MAP
        .iter()
        .find(|(k, _)| *k == pt)
        .map(|(_, v)| v)
}

pub fn get_dither(filtering_type: &str) -> &'static str {
    DITHER_MAP
        .iter()
        .find(|(k, _)| *k == filtering_type)
        .map(|(_, v)| *v)
        .unwrap_or("stucki")
}

pub fn process_type_keys() -> impl Iterator<Item = &'static str> {
    PROCESS_TYPE_MAP.iter().map(|(k, _)| *k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn laser_line_engrave_has_correct_info() {
        let info = get_process_type_info("LaserLineEngrave").expect("should be present");
        assert_eq!(info.kind, "Cut");
        assert_eq!(info.name, "Line");
        assert_eq!(info.index, 0);
    }

    #[test]
    fn laser_fill_engrave_has_correct_info() {
        let info = get_process_type_info("LaserFillEngrave").expect("should be present");
        assert_eq!(info.kind, "Scan");
        assert_eq!(info.name, "Fill");
        assert_eq!(info.index, 1);
    }

    #[test]
    fn laser_line_cut_has_correct_info() {
        let info = get_process_type_info("LaserLineCut").expect("should be present");
        assert_eq!(info.kind, "Cut");
        assert_eq!(info.name, "Cut");
        assert_eq!(info.index, 2);
    }

    #[test]
    fn unknown_process_type_returns_none() {
        assert!(get_process_type_info("UnknownType").is_none());
    }

    #[test]
    fn dither_if_relief_maps_to_stucki() {
        assert_eq!(get_dither("IF_Relief"), "stucki");
    }

    #[test]
    fn dither_if_threshold_maps_correctly() {
        assert_eq!(get_dither("IF_Threshold"), "threshold");
    }

    #[test]
    fn dither_if_ordered_maps_correctly() {
        assert_eq!(get_dither("IF_Ordered"), "ordered");
    }

    #[test]
    fn dither_if_dither_maps_to_floyd() {
        assert_eq!(get_dither("IF_Dither"), "floyd");
    }

    #[test]
    fn unknown_dither_type_defaults_to_stucki() {
        assert_eq!(get_dither("UnknownDither"), "stucki");
    }

    #[test]
    fn process_type_keys_contains_all_three_variants() {
        let keys: Vec<&str> = process_type_keys().collect();
        assert!(keys.contains(&"LaserLineEngrave"));
        assert!(keys.contains(&"LaserFillEngrave"));
        assert!(keys.contains(&"LaserLineCut"));
        assert_eq!(keys.len(), 3);
    }
}
