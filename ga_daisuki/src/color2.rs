use indexmap::IndexMap;
use kintaro::color_map_from_named_colorsets;
use kintaro::gen::GenColor;
use kintaro::{colorsets_from_vec_hex_strings, ColorMap, ColorSets};

pub fn named_colorsets<'a>() -> Vec<(&'a str, Vec<&'a str>)> {
    vec![
        ("a", vec!["#001122", "#110022"]),
        ("b", vec!["#331122", "#113322"]),
        ("c", vec!["#331122", "#113322"]),
        ("d", vec!["#baba33", "#fafa22"]),
        ("e", vec!["#3333ab", "#ad5522"]),
        ("f", vec!["#883333", "#885554"]),
        ("g", vec!["#2233bb", "#3355dd"]),
    ]
}

pub fn color_map() -> ColorMap {
    color_map_from_named_colorsets(named_colorsets())
}
