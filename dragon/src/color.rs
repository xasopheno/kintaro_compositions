use indexmap::IndexMap;
use kintaro::color_map_from_named_colorsets;
use kintaro::gen::GenColor;
use kintaro::{colorsets_from_vec_hex_strings, ColorMap, ColorSets};

pub fn named_colorsets<'a>() -> Vec<(&'a str, Vec<&'a str>)> {
    vec![
        ("a", vec!["#bd0044", "#ad11aa", "#030303", "#001100"]),
        (
            "c",
            vec!["#ee44fe", "#dd11de", "#223355", "#3233ac", "#101200"],
        ),
        ("e", vec!["#0045dd", "#29377b", "#323030", "#001100"]),
        (
            "d",
            vec![
                "#0034ee", "#29378a", "#212121", "#001100", "#440000", "#000000",
            ],
        ),
        // ("c", vec!["#744253", "#ccddaa"]),
        // ("d", vec!["#447880", "#facba2"]),
        (
            "b",
            vec!["#934748", "#ba83ba", "#033333", "#303030", "#001100"],
        ),
        // ("f", vec!["#323341", "#2a0d10"]),
        // ("g", vec!["#1A3c3A", "#fadf23"]),
        // ("h", vec!["#aa4411", "#4e3636"]),
        // ("i", vec!["#aa2244", "#232423"]),
        // ("j", vec!["#aa3300", "#3f2527"]),
    ]
}

pub fn color_map() -> ColorMap {
    color_map_from_named_colorsets(named_colorsets())
}
