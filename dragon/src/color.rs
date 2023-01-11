use indexmap::IndexMap;
use kintaro::color_map_from_named_colorsets;
use kintaro::gen::GenColor;
use kintaro::{colorsets_from_vec_hex_strings, ColorMap, ColorSets};

pub fn named_colorsets<'a>() -> Vec<(&'a str, Vec<&'a str>)> {
    vec![
        ("a", vec!["#bd1313", "#433844", "#030303", "#001100"]),
        (
            "c",
            vec!["#224433", "#fadf23", "#dd7723", "#223355", "#101200"],
        ),
        (
            "e",
            vec![
                "#000330", "#220000", "#323030", "#001100", "#101132", "#302232", "#190201",
            ],
        ),
        (
            "d",
            vec![
                "#56343e", "#29373a", "#212121", "#001100", "#440000", "#000000",
            ],
        ),
        (
            "b",
            vec![
                "#774820", "#932230", "#934748", "#ba83ba", "#033333", "#303030", "#001100",
                "#300303", "#382042",
            ],
        ),
    ]
}

pub fn color_map() -> ColorMap {
    color_map_from_named_colorsets(named_colorsets())
}
