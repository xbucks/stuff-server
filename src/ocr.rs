use chrono::{Utc, DateTime};
use rusty_tesseract::{Args, Image};
use std::collections::HashMap;

pub fn read_screenshot() -> String {
    let temp = format!("{}temp.png", String::from_utf8_lossy(DOCUMENTS));
    let img = Image::from_path(temp).unwrap();

    // fill your own argument struct if needed
    let image_to_string_args = Args {
        lang: "eng".into(),
        config_variables: HashMap::from([(
            "tessedit_char_whitelist".into(),
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789@$./ ?,".into(),
        )]),
        dpi: Some(150),
        psm: Some(6),
        oem: Some(3),
    };

    let output = rusty_tesseract::image_to_string(&img, &image_to_string_args).unwrap();

    output
}