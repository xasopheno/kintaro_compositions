use indexmap::IndexMap;
use kintaro::color_map_from_named_colorsets;
use kintaro::gen::GenColor;
use kintaro::{colorsets_from_vec_hex_strings, ColorMap, ColorSets};

pub fn named_colorsets<'a>() -> Vec<(&'a str, Vec<&'a str>)> {
    vec![
        ("a", vec!["#da1173", "#032333"]),
        ("b", vec!["#233933", "#313030"]),
        ("c", vec!["#743253", "#c1ddaa"]),
        ("d", vec!["#883880", "#fac8a2"]),
        ("e", vec!["#63477D", "#afcf7a"]),
        ("f", vec!["#683387", "#ffed00"]),
        ("g", vec!["#1A3A2A", "#f72323"]),
        ("h", vec!["#338e51", "#34df38"]),
        ("i", vec!["#cc2233", "#232813"]),
        ("j", vec!["#ee4399", "#382427"]),
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
