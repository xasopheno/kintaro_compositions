use kintaro::color_map_from_named_colorsets;
use kintaro::gen::GenColor;
use kintaro::{
    color_map_from_named_gen_color, ColorMap, ColorSet, ColorSets, RandColor, RandColorSet,
};

pub fn color_map() -> ColorMap {
    let colors = vec![
        ("s", ["#ff0000", "#122222", "#553333", "#300000"]),
        ("a", ["#885511", "#ffaaaa", "#222222", "#ddaaaa"]),
        ("t", ["#332233", "#773355", "#331122", "#221121"]),
        ("b", ["#5a38ff", "#4a3100", "#111133", "#5838aa"]),
    ];

    color_map_from_named_colorsets(colors)
}
