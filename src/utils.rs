use crate::{app::AppState, consts};
use std::ops::Deref;
use image::{png::PngEncoder, ImageBuffer, Rgba, ImageError, Pixel};
use once_cell::sync::OnceCell;
use warp::Filter;
use rusttype::{point, Font};

static STATE: OnceCell<AppState> = OnceCell::new();
static SPACE_WIDTH: OnceCell<i32> = OnceCell::new();


// rusttype can't rasterize spaces :|
fn init_space_width(font: &Font<'static>) {
    let offset = point(0.0, font.v_metrics(consts::FONT_SCALE).ascent);
    let layout = font.layout(" r", consts::FONT_SCALE, offset);
    let r = font.glyph('r').scaled(consts::FONT_SCALE).positioned(offset).pixel_bounding_box().unwrap().max.x;
    let width = layout.last().unwrap().pixel_bounding_box().unwrap().max.x - r;
    SPACE_WIDTH.set(width).ok().expect("INVALID INIT STATE!!");
}

pub fn init_state(api_url: String, assets_path: String, base_img: ImageBuffer<Rgba<u8>, Vec<u8>>, font: Font<'static>) {
    init_space_width(&font);
    STATE.set(AppState::new(api_url, assets_path, base_img, font)).ok().expect("INVALID INIT STATE!!");
}

pub fn space_width() -> i32 {
    *SPACE_WIDTH.get().unwrap()
}

pub fn with_state() -> impl Filter<Extract = (&'static AppState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(|| STATE.get().expect("AppState not initialized!!!"))
}

pub fn encode_png<P, Container>(img: &ImageBuffer<P, Container>) -> Result<Vec<u8>, ImageError>
where
    P: Pixel<Subpixel = u8> + 'static,
    Container: Deref<Target = [P::Subpixel]>,
{
    let mut buf = Vec::new();
    let encoder = PngEncoder::new(&mut buf);
    encoder.encode(img, img.width(), img.height(), P::COLOR_TYPE)?;
    Ok(buf)
}
