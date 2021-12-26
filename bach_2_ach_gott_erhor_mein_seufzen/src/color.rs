use kintaro::color_map_from_named_colorsets;
use kintaro::gen::GenColor;
use kintaro::{
    color_map_from_named_gen_color, ColorMap, ColorSet, ColorSets, RandColor, RandColorSet,
};

pub fn color_map() -> ColorMap {
    let colors = vec![
        ("s", ["#aa22ff", "#ffaaaa", "#331122", "#222222"]),
        ("a", ["#1f338f", "#553333", "#310011", "#122221"]),
        ("t", ["#032200", "#4a31aa", "#331133", "#124421"]),
        ("b", ["#3322ff", "#773355", "#ddaaaa", "#221121"]),
        ("reverb", ["#aaaaaa", "#333333", "#777777", "#111111"]),
    ];

    color_map_from_named_colorsets(colors)
}
