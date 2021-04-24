use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::highlighting::Color;
use highlightjs_rs::JSExecutor;
use image::Rgba;


pub fn language_detect(src: &str) -> String {
    let mut ex = JSExecutor::new().unwrap();
    let mut l = ex.detect_language(src).unwrap();
    l.get_mut(0..1).unwrap().make_ascii_uppercase();
    l
}

pub fn get_syntax_lang<'a>(lang: &str, syntaxes: &'a SyntaxSet) -> &'a SyntaxReference {
    syntaxes.find_syntax_by_name(lang)
        .unwrap_or_else(|| syntaxes.find_syntax_plain_text())
}

pub fn to_rgba8(color: &Color) -> Rgba<u8> {
    Rgba::<u8>([color.r, color.g, color.b, color.a])
}
