use indexmap::IndexMap;
use kintaro::color_map_from_named_colorsets;
use kintaro::gen::GenColor;
use kintaro::{colorsets_from_vec_hex_strings, ColorMap, ColorSets};

pub fn named_colorsets<'a>() -> Vec<(&'a str, Vec<&'a str>)> {
    vec![
        ("left_frontal", vec!["#dd1133", "#222222"]),
        ("right_frontal", vec!["#833903", "#222222"]),
        ("left_temporal", vec!["#744253", "#333333"]),
        ("left_parietal", vec!["#887880", "#111111"]),
        ("right_parietal", vec!["#63474D", "#111111"]),
        ("right_temporal", vec!["#683347", "#333333"]),
        ("left_occipital", vec!["#1A3A3A", "#222222"]),
        ("right_occipital", vec!["#383F51", "#222222"]),
    ]
}

pub fn color_map() -> ColorMap {
    color_map_from_named_colorsets(named_colorsets())
}

pub fn _color_map() -> ColorMap {
    let colors = vec![
        ("a", vec!["#dd1133", "#122333"]),
        ("b", vec!["#cc3344", "#122222"]),
        ("c", vec!["#ff0033", "#122111"]),
        ("d", vec!["#5a38ff", "#4a3112"]),
        ("e", vec!["#4a38dd", "#4a3123"]),
        ("f", vec!["#3a38bb", "#4a3134"]),
        ("g", vec!["#11aa88", "#11a111"]),
        ("h", vec!["#33aa77", "#22a133"]),
        ("i", vec!["#55aa66", "#33a155"]),
        ("j", vec!["#885533", "#ffaaaa"]),
        ("k", vec!["#885544", "#ffaabb"]),
        ("l", vec!["#885554", "#ffaacc"]),
    ];

    color_map_from_named_colorsets(colors)
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
