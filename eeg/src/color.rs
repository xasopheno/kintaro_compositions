use kintaro::color_map_from_named_colorsets;
use kintaro::gen::GenColor;
use kintaro::{
    color_map_from_named_gen_color, ColorMap, ColorSet, ColorSets, RandColor, RandColorSet,
};

pub fn _color_map() -> ColorMap {
    let colors = vec![
        ("a", ["#ff0000", "#122222"]),
        ("b", ["#ff0000", "#122222"]),
        ("c", ["#ff0000", "#122222"]),
        ("d", ["#885511", "#ffaaaa"]),
        ("e", ["#885511", "#ffaaaa"]),
        ("f", ["#885511", "#ffaaaa"]),
        ("g", ["#00aa88", "#00a111"]),
        ("h", ["#00aa88", "#00a111"]),
        ("i", ["#00aa88", "#00a111"]),
        ("j", ["#5a38ff", "#4a3100"]),
        ("k", ["#5a38ff", "#4a3100"]),
        ("l", ["#5a38ff", "#4a3100"]),
    ];

    color_map_from_named_colorsets(colors)
}

const LOWER: &[&'static str] = &["#411055"];
const LOW: &[&'static str] = &["#554381"];
const HIGH: &[&'static str] = &["#145745"];
const HIGHER: &[&'static str] = &["#927677"];

pub fn _color_map_low_high<'a>() -> ColorMap {
    let colors: Vec<(&'a str, Box<dyn GenColor>)> = vec![
        ("a", Box::new(ColorSet::init(LOWER))),
        ("b", Box::new(ColorSet::init(LOW))),
        ("c", Box::new(ColorSet::init(HIGH))),
        ("d", Box::new(ColorSet::init(HIGHER))),
        ("e", Box::new(ColorSet::init(HIGHER))),
        ("f", Box::new(ColorSet::init(LOW))),
        ("g", Box::new(ColorSet::init(LOW))),
        ("h", Box::new(ColorSet::init(HIGHER))),
        ("i", Box::new(ColorSet::init(HIGHER))),
        ("j", Box::new(ColorSet::init(HIGH))),
        ("k", Box::new(ColorSet::init(LOW))),
        ("l", Box::new(ColorSet::init(LOWER))),
    ];
    color_map_from_named_gen_color(colors)
}

pub fn color_map<'a>() -> ColorMap {
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
        ("j", Box::new(RandColorSet::init(8))),
        ("k", Box::new(RandColorSet::init(8))),
        ("l", Box::new(RandColorSet::init(8))),
    ];
    color_map_from_named_gen_color(colors)
}

#[allow(unused)]
pub fn colorsets() -> ColorSets {
    ColorSets::init(vec![
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
