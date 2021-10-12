mod app;
mod consts;
mod handlers;
mod ipri;
mod syntax;
mod utils;

use rusttype::Font;
use warp::Filter;

use std::env;
use warp::http::header::{HeaderMap, HeaderValue};

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

    let font: Font<'static> =
        Font::try_from_bytes(include_bytes!("FiraCode-Retina.ttf") as &[u8]).unwrap();

    utils::init_state(arg, image, font);

    println!("Starting warp server on port 3030");

    let p = warp::path("p");
    let ap = utils::with_state()
        .and(warp::path::param())
        .and_then(handlers::root_handler);

    let pk = warp::path("pk");
    let apk = utils::with_state()
        .and(warp::path::param())
        .and(warp::path::param())
        .and_then(handlers::root_handler_known);

    warp::serve(
        p.and(ap)
            .or(pk.and(apk))
            .with(warp::reply::with::headers(png_header)),
    )
    .run(([127, 0, 0, 1], 3030))
    .await;
}
