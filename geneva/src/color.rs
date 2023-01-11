use indexmap::IndexMap;
use kintaro::color_map_from_named_colorsets;
use kintaro::gen::GenColor;
use kintaro::{colorsets_from_vec_hex_strings, ColorMap, ColorSets};

pub fn named_colorsets<'a>() -> Vec<(&'a str, Vec<&'a str>)> {
    vec![
        ("sendMessageMutation", vec!["#dd1133", "#030303"]),
        // ("b", vec!["#293923", "#303030"]),
        // ("c", vec!["#744253", "#ccddaa"]),
        ("createRoom", vec!["#447880", "#fa3355"]),
        ("acceptFriendRequest", vec!["#63474D", "#adc37a"]),
        ("createHome", vec!["#323341", "#2a0d10"]),
        // ("g", vec!["#1A3c3A", "#fadf23"]),
        // ("h", vec!["#aa4411", "#4e3636"]),
        // ("i", vec!["#aa2244", "#232423"]),
        // ("j", vec!["#aa3300", "#3f2527"]),
    ]
}

pub fn color_map() -> ColorMap {
    color_map_from_named_colorsets(named_colorsets())
}
