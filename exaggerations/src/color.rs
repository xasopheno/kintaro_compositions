use indexmap::IndexMap;
use kintaro::color_map_from_named_colorsets;
use kintaro::gen::GenColor;
use kintaro::{colorsets_from_vec_hex_strings, ColorMap, ColorSets};

pub fn named_colorsets<'a>() -> Vec<(&'a str, Vec<&'a str>)> {
    vec![
        ("a", vec!["#aa1133", "#030303"]),
        ("b", vec!["#2339e3", "#303030"]),
        ("c", vec!["#744253", "#ccddaa"]),
        ("d", vec!["#887880", "#facba2"]),
        ("e", vec!["#63474D", "#adc37a"]),
        ("f", vec!["#683347", "#faed00"]),
        ("g", vec!["#1A3A3A", "#fadf23"]),
        ("h", vec!["#383F51", "#dadfea"]),
        ("i", vec!["#cc2277", "#232423"]),
        ("j", vec!["#ee4499", "#3f2527"]),
    ]
}

pub fn color_map() -> ColorMap {
    color_map_from_named_colorsets(named_colorsets())
}

#[allow(unused)]
pub fn colorsets() -> ColorSets {
    colorsets_from_vec_hex_strings(vec![
        vec!["#6655aa", "#222222"],
        vec!["#eeaC88", "#121312", "#333333"],
        vec![
            "#213CFB", "#310CFA", "#6688aa", "#111111", "#121212", "#101010",
        ],
        vec!["#660000", "#100101", "#300002"],
        vec!["#473859", "#222222"],
        vec!["#300300", "#333333"],
        vec!["#001931", "#000000", "#222200"],
        vec!["#a000a0", "#000000", "#2303aa", "#333333"],
        vec!["#473859", "#222222"],
        vec!["#348348", "#112312"],
        vec!["#0000ee", "#0e000e"],
        //
        vec!["#333333", "#111111", "#777777"],
        vec!["#660000", "#100101", "#300002", "#100001", "#010210"],
        vec!["#473850", "#222222", "#001001"],
        vec!["#112112", "#000033"],
        vec!["#ff00ff", "#000000"],
        vec!["#38881a", "#333333"],
        vec!["#aa10e4", "#333333"],
    ])
}
