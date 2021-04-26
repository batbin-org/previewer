use crate::{ipri, utils, app::AppState, consts::{MARGINS, FONT_SCALE}};
use syntect::easy::HighlightLines;
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::highlighting::Color;
use highlightjs_rs::JSExecutor;
use image::Rgba;


fn language_detect(src: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut ex = JSExecutor::new()?;
    let mut l = ex.detect_language(src)?;
    l.get_mut(0..1).unwrap().make_ascii_uppercase();
    Ok(l)
}

fn get_syntax_lang<'a>(lang: &str, syntaxes: &'a SyntaxSet, ext: Option<String>) -> &'a SyntaxReference {
    match ext {
        Some(e) => {
            syntaxes.find_syntax_by_extension(&e)
                .unwrap_or_else(|| syntaxes.find_syntax_plain_text())
        },

        None => {
            syntaxes.find_syntax_by_name(lang)
                .unwrap_or_else(|| syntaxes.find_syntax_plain_text())
        }
    }
}

fn to_rgba8(color: &Color) -> Rgba<u8> {
    Rgba::<u8>([color.r, color.g, color.b, color.a])
}

pub fn render_preview(state: &AppState, src: &str, ext: Option<String>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let lang = language_detect(src)?;
    let syntax = get_syntax_lang(&lang, &state.syntaxes, ext);

    let mut h = HighlightLines::new(syntax, &state.highlight_theme);
    let lines = src.split_inclusive("\n").take(18);

    let mut img2 = state.base_img.clone();

    for (i, line) in lines.enumerate() {
        let highlighted = h.highlight(&line, &state.syntaxes);
        let mut w = 64;
        for (style, chunk) in highlighted.iter() {
            let mut isspace = true; // whether the chunk is purely whitespace
            let mut sc = 0; // number of spaces in chunk
            let mut cindx = 0; // last non-space character in chunk
            let mut c_s = 0; // length of the chunk

            let c_string: String = chunk.chars().enumerate().filter_map(|(i, c)| match c {
                '\n' => None, // doesn't contribute to length
                ' ' => {
                    sc += 1;
                    c_s += 1;
                    Some(' ') 
                },
                c => {
                    isspace = false; // chunk has non whitespace characters too!
                    cindx = i;
                    c_s += 1;
                    Some(c) 
                }
            }).collect();

            if isspace {
                w += utils::space_width()*sc;
            } else {
                w += ipri::draw_text_mut_w(&mut img2,  to_rgba8(&style.foreground), w, MARGINS[i], FONT_SCALE, &state.font, &c_string);
                let trailing_spaces = c_s - cindx - 1;
                w += utils::space_width()*trailing_spaces as i32; // rusttype doesn't handle trailing spaces :|
            }
        }
    }

    let res = utils::encode_png(&img2)?;
    Ok(res)
}
