use indexmap::IndexMap;
use kintaro::{
    color::{RandColor, RandColorSet},
    colorset_from_hex_strings, colorsets_from_vec_hex_strings, color_map_from_named_gen_color
    gen::GenColor,
    ColorMap, ColorSet, ColorSets, NamedColorSet,
};

fn colors<'a>() -> Vec<NamedColorSet<'a>> {
    vec![
        ("a", vec!["#5a38ff", "#4a3100"]),
        ("b", vec!["#00aa88", "#00a111"]),
        ("c", vec!["#885511", "#ffaaaa"]),
        ("d", vec!["#ff0000", "#122222"]),
        ("e", vec!["#229292", "#010001"]),
        ("f", vec!["#000f88", "#010001"]),
        ("g", vec!["#333333", "#000000"]),
        ("h", vec!["#ffffff", "#790000"]),
        ("i", vec!["#334444", "#443333", "#778888"]),
    ]
}

pub fn color_map(colors: ) -> ColorMap {
    let colors: Vec<(&'a str, Box<dyn GenColor>)> = vec![
        ("a", Box::new(RandColorSet::init(8))),
        ("b", Box::new(RandColorSet::init(8))),
        ("c", Box::new(RandColorSet::init(8))),
        ("d", Box::new(RandColorSet::init(8))),
        ("e", Box::new(RandColorSet::init(8))),
        ("f", Box::new(RandColorSet::init(8))),
        ("g", Box::new(RandColorSet::init(8))),
        ("h", Box::new(RandColorSet::init(8))),
        ("i", Box::new(RandColorSet::init(8))),
    ];
    color_map_from_named_gen_color(colors)
}

#[allow(unused)]
pub fn color_sets() -> ColorSets {
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
