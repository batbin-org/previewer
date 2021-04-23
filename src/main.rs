extern crate imageproc;
extern crate image;
extern crate rusttype;

use imageproc::drawing::draw_text_mut;
use rusttype::{Scale, Font};
use image::{png::PngEncoder, ImageBuffer, ImageError, Pixel, Rgba};
use std::io::{BufReader, BufRead};
use std::fs::File;
use warp::Filter;
use std::ops::Deref;
use warp::filters::BoxedFilter;
use warp::http::header::{HeaderMap, HeaderValue};
use std::env;

static MARGINS: &'static [u32] = &[
  // 1    2    3    4    5     6     7     8     9    10
    111, 150, 188, 225, 264,  302,  341,  379,  416,  455,
    493, 532, 570, 608, 646,  684,  723,  761,  799,  837,
    875, 914, 952, 990, 1028, 1066, 1105, 1143, 1181, 1219
];

fn encode_png<P, Container>(img: &ImageBuffer<P, Container>) -> Result<Vec<u8>, ImageError>
where
    P: Pixel<Subpixel = u8> + 'static,
    Container: Deref<Target = [P::Subpixel]>,
{
    let mut buf = Vec::new();
    let encoder = PngEncoder::new(&mut buf);
    encoder.encode(img, img.width(), img.height(), P::COLOR_TYPE)?;
    Ok(buf)
}

pub fn root_route() -> BoxedFilter<()> {
    warp::get().and(warp::path::end()).boxed()
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

pub async fn root_handler(uuid: String, pastes_dir: String, image: ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>, font: rusttype::Font<'_>) -> Result<impl warp::Reply, warp::Rejection> {
    let file = match File::open(format!("{}/{}", pastes_dir, uuid)) {
        Ok(content) => content,
        Err(_) => return Err(warp::reject()) 
    };
    let reader = BufReader::new(file);

    let mut line_num = 0;

    let mut img2 = image.clone();

    for line in reader.lines() {
        if line_num >= 30 { break }
        match line {
            Ok(str) => {
                // x -> 102
                draw_text_mut(&mut img2, Rgba([255u8, 255u8, 255u8, 100u8]), 102, MARGINS[line_num], Scale { x: 24., y: 24. }, &font, &str);
                line_num += 1;
            },
            Err(e) => println!("Something went wrong because: {}", e),
        };
    }

    let imgpng = encode_png(&img2);

    match imgpng {
        Ok(final_img) => {
            return Ok(final_img)
        }

        Err(_) => return Err(warp::reject())
    }
}

#[tokio::main]
async fn main() {
    let arg = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Absolute path of pastes directory *without* trailing slash was not passed!")
    };

    let mut png_header = HeaderMap::new();
    png_header.insert("Content-type", HeaderValue::from_static("image/png"));

    let image = image::open(format!("{}/{}", arg, "editor.png"))
                        .expect("ERR No template image found at provided path")
                        .to_rgba8();

    let font: Font<'static> = Font::try_from_bytes(include_bytes!("FiraCode-Retina.ttf") as &[u8]).unwrap();

    println!("Starting warp server on port 3030");

    warp::serve(warp::path!("p" / String).and(with_pastes_dir(arg)).and(with_img(image)).and(with_font(font)).and_then(root_handler).with(warp::reply::with::headers(png_header)))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
