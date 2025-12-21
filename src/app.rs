use rusttype::Font;
use image::{ImageBuffer, Rgba};
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{Theme, ThemeSet};
use std::path::Path;

use reqwest::Client;

pub struct AppState {
    pub api_url: String,
    pub base_img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub font: Font<'static>,
    pub syntaxes: SyntaxSet,
    pub highlight_theme: Theme,
    pub http_client: Client,
}

impl AppState {
    pub fn new(api_url: String, assets_path: String, base_img: ImageBuffer<Rgba<u8>, Vec<u8>>, font: Font<'static>) -> Self {
        let theme_path = format!("{}/{}", &assets_path, "TwoDark.tmTheme");
        Self {
            api_url,
            base_img,
            font,
            syntaxes: SyntaxSet::load_defaults_newlines(),
            highlight_theme: ThemeSet::get_theme(Path::new(&theme_path)).unwrap_or_else(|_| panic!("Couldn't load the TwoDark theme!")),
            http_client: Client::new(),
        }
    }
}
