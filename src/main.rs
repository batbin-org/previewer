mod app;
mod consts;
mod handlers;
mod ipri;
mod syntax;
mod utils;

use rusttype::Font;
use warp::Filter;

use warp::http::header::{HeaderMap, HeaderValue};
use std::env;

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

    utils::init_state(arg, image, font);

    println!("Starting warp server on port 3030");

    let route = warp::path("p")
                    .and(utils::with_state())
                    .and(warp::path::param())
                    .and_then(handlers::root_handler)
                    .with(warp::reply::with::headers(png_header));

    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
