extern crate imageproc;
extern crate image;
extern crate rusttype;

mod ipri;
mod utils;

use rusttype::{Scale, Font};
use image::{ImageBuffer, Rgba};
use std::io::{BufReader, BufRead};
use std::fs::File;
use warp::Filter;

use warp::http::header::{HeaderMap, HeaderValue};
use std::env;

static MARGINS: &'static [i32] = &[
  // 1    2    3    4    5     6     7     8     9    10
    111, 150, 188, 225, 264,  302,  341,  379,  416,  455,
    493, 532, 570, 608, 646,  684,  723,  761,  799,  837,
    875, 914, 952, 990, 1028, 1066, 1105, 1143, 1181, 1219
];

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
                ipri::draw_text_mut_w(&mut img2, Rgba([255u8, 255u8, 255u8, 100u8]), 102, MARGINS[line_num], Scale { x: 24., y: 24. }, &font, &str);
                line_num += 1;
            },
            Err(e) => println!("Something went wrong because: {}", e),
        };
    }

    let imgpng = utils::encode_png(&img2);

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

    let route = warp::path!("p" / String)
                    .and(utils::with_pastes_dir(arg))
                    .and(utils::with_img(image))
                    .and(utils::with_font(font))
                    .and_then(root_handler)
                    .with(warp::reply::with::headers(png_header));

    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
