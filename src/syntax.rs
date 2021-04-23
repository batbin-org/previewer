use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::highlighting::Color;
use hyperpolyglot::Language;
use image::Rgba;
use std::path::Path;
use std::convert::TryInto;

fn get_language_extension(path: &Path) -> Option<&'static str> {
    let lang_info: Language = hyperpolyglot::detect(path).ok()??.language().try_into().ok()?;
    return lang_info.ext;
}

pub fn get_syntax_lang<'a>(fpath: &Path, syntaxes: &'a SyntaxSet) -> &'a SyntaxReference {
    let lang_ext = get_language_extension(fpath);
    lang_ext
        .map(|ex| syntaxes.find_syntax_by_extension(ex)).flatten()
        .unwrap_or_else(|| syntaxes.find_syntax_plain_text())
}

pub fn to_rgba8(color: &Color) -> Rgba<u8> {
    Rgba::<u8>([color.r, color.g, color.b, color.a])
}
