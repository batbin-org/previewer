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
    // Get API URL from environment or use default
    let api_url = env::var("PASTE_API_URL")
        .unwrap_or_else(|_| "https://api-umbra.batbin.me".to_string());
    
    // Get assets path from command line or use default
    let assets_path = if env::args().count() >= 2 {
        env::args().nth(1).unwrap()
    } else {
        "/data".to_string()
    };

    let mut png_header = HeaderMap::new();
    png_header.insert("Content-type", HeaderValue::from_static("image/png"));

    let image = image::open(format!("{}/{}", &assets_path, "editor.png"))
                        .expect("ERR No template image found at provided path")
                        .to_rgba8();

    let font: Font<'static> = Font::try_from_bytes(include_bytes!("FiraCode-Retina.ttf") as &[u8]).unwrap();

    utils::init_state(api_url.clone(), assets_path, image, font);

    println!("Starting warp server on port 3030");
    println!("Using paste API: {}", api_url);

    let p = warp::path("p");
    let ap = utils::with_state()
                        .and(warp::path::param())
                        .and_then(handlers::root_handler);

    let pk = warp::path("pk");
    let apk = utils::with_state()
                        .and(warp::path::param())
                        .and(warp::path::param())
                        .and_then(handlers::root_handler_known);

    warp::serve(p.and(ap).or(pk.and(apk)).with(warp::reply::with::headers(png_header)))
        .run(([0, 0, 0, 0], 3030))
        .await;
}
