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
