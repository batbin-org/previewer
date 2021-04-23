use std::ops::Deref;
use image::{png::PngEncoder, ImageBuffer, ImageError, Pixel};
use warp::Filter;

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

pub fn with_img(img: ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>) -> impl Filter<Extract = (ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || img.clone())
}

pub fn with_font(font: rusttype::Font) -> impl Filter<Extract = (rusttype::Font,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || font.clone())
}

pub fn with_pastes_dir(pastes_dir: String) -> impl Filter<Extract = (String,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pastes_dir.clone())
}
