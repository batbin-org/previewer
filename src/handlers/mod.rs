use crate::{app::AppState, utils, ipri, syntax, consts::{MARGINS, IMG_SCALE}};
use std::time::{SystemTime, UNIX_EPOCH};
use syntect::easy::HighlightLines;
use tokio::task;
use tokio::fs;

pub fn epoch_ms() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("IMPOSSIBLE thanos.png")
        .as_millis();
}

pub async fn root_handler(state: &'static AppState, uuid: String) -> Result<impl warp::Reply, warp::Rejection> {
    let f = format!("{}/{}", &state.pastes, uuid);
    let code = fs::read_to_string(f).await.map_err(|_| warp::reject())?;
    let t = epoch_ms();

    let (code, syntax) = task::spawn_blocking(move || -> Result<_, warp::Rejection> {
        let l = syntax::language_detect(&code);
        println!("lang: {}", l);
        // return the ownership to code
        Ok((code, syntax::get_syntax_lang(&l, &state.syntaxes)))
    }).await.map_err(|_| warp::reject())??;

    let mut h = HighlightLines::new(syntax, &state.highlight_theme);
    let lines = code.split_inclusive("\n").take(30);

    let mut img2 = state.base_img.clone();

    for (i, line) in lines.enumerate() {
        let highlighted = h.highlight(&line, &state.syntaxes);
        let mut w = 102;
        for (j, (style, chunk)) in highlighted.iter().enumerate() {
            let mut c_strip: &str = chunk;
            if j == highlighted.len()-1 {
                c_strip = chunk.strip_suffix("\n").unwrap_or(chunk);
            }

            w += ipri::draw_text_mut_w(&mut img2,  syntax::to_rgba8(&style.foreground), w, MARGINS[i], IMG_SCALE, &state.font, c_strip);
        }
    }

    let res = utils::encode_png(&img2).map_err(|_| warp::reject());

    println!("time {}", epoch_ms() - t);
    return res;
}
