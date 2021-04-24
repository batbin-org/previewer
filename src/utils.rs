use crate::app::AppState;
use std::ops::Deref;
use image::{png::PngEncoder, ImageBuffer, Rgba, ImageError, Pixel};
use once_cell::sync::OnceCell;
use warp::Filter;
use rusttype::Font;

static STATE: OnceCell<AppState> = OnceCell::new();

pub fn init_state(pastes: String, base_img: ImageBuffer<Rgba<u8>, Vec<u8>>, font: Font<'static>) {
    STATE.set(AppState::new(pastes, base_img, font)).ok().unwrap();
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
