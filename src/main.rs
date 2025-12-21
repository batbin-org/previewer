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
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    eprintln!("=== Previewer starting ===");
    io::stderr().flush().unwrap();
    
    // Get API URL from environment or use default
    let api_url = env::var("PASTE_API_URL")
        .unwrap_or_else(|_| "https://umbra.batbin.me/api".to_string());
    
    // Get assets path from command line or use default
    let assets_path = if env::args().count() >= 2 {
        env::args().nth(1).unwrap()
    } else {
        "/data".to_string()
    };

    eprintln!("Assets path: {}", assets_path);
    eprintln!("API URL: {}", api_url);
    io::stderr().flush().unwrap();

    let mut png_header = HeaderMap::new();
    png_header.insert("Content-type", HeaderValue::from_static("image/png"));

    eprintln!("Loading image...");
    io::stderr().flush().unwrap();
    
    let image = image::open(format!("{}/{}", &assets_path, "editor.png"))
                        .expect("ERR No template image found at provided path")
                        .to_rgba8();

    eprintln!("Image loaded successfully");
    io::stderr().flush().unwrap();

    let font: Font<'static> = Font::try_from_bytes(include_bytes!("FiraCode-Retina.ttf") as &[u8]).unwrap();

    eprintln!("Initializing state...");
    io::stderr().flush().unwrap();
    
    utils::init_state(api_url.clone(), assets_path, image, font);

    eprintln!("State initialized, starting warp server on port 3030");
    io::stderr().flush().unwrap();

    let p = warp::path("p");
    let ap = utils::with_state()
                        .and(warp::path::param())
                        .and_then(handlers::root_handler);

    let pk = warp::path("pk");
    let apk = utils::with_state()
                        .and(warp::path::param())
                        .and(warp::path::param())
                        .and_then(handlers::root_handler_known);

    eprintln!("About to call warp::serve().run().await");
    io::stderr().flush().unwrap();

    warp::serve(p.and(ap).or(pk.and(apk)).with(warp::reply::with::headers(png_header)))
        .run(([0, 0, 0, 0], 3030))
        .await;
    
    eprintln!("Warp server exited (this should never print!)");
    io::stderr().flush().unwrap();
}
