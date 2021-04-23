use rusttype::Font;
use image::{ImageBuffer, Rgba};
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{Theme, ThemeSet};

#[derive(Debug)]
pub struct AppState {
    pub pastes: String,
    pub base_img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub font: Font<'static>,
    pub syntaxes: SyntaxSet,
    pub highlight_theme: Theme
}

impl AppState {
    pub fn new(pastes: String, base_img: ImageBuffer<Rgba<u8>, Vec<u8>>, font: Font<'static>) -> Self {
        let ts = ThemeSet::load_defaults();
        Self {
            pastes,
            base_img,
            font,
            syntaxes: SyntaxSet::load_defaults_newlines(),
            highlight_theme: ts.themes["base16-ocean.dark"].clone()
        }
    }
}
