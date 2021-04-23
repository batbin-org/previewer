use crate::{app::AppState, utils, ipri, syntax, consts::{MARGINS, IMG_SCALE}};
use std::path::Path;
use syntect::easy::HighlightLines;
use tokio::task;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs::File;

pub async fn root_handler(state: &'static AppState, uuid: String) -> Result<impl warp::Reply, warp::Rejection> {
    let f = format!("{}/{}", &state.pastes, uuid);
    let f2 = f.clone();

    let syntax = task::spawn_blocking(move || {
        syntax::get_syntax_lang(Path::new(&f2), &state.syntaxes)
    }).await.map_err(|_| warp::reject())?;
    let mut h = HighlightLines::new(syntax, &state.highlight_theme);

    let file = File::open(f).await.map_err(|_| warp::reject())?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut img2 = state.base_img.clone();

    let mut i: usize = 0;
    while let Some(line) = lines.next_line().await.map_err(|_| warp::reject())? {
        if i >= 30 { break }
        let highlighted = h.highlight(&line, &state.syntaxes);
        let mut w = 102;
        for (j, (style, chunk)) in highlighted.iter().enumerate() {
            let mut c_strip: &str = chunk;
            if j == highlighted.len()-1 {
                c_strip = chunk.strip_suffix("\n").unwrap_or(chunk);
            }

            w += ipri::draw_text_mut_w(&mut img2,  syntax::to_rgba8(&style.foreground), w, MARGINS[i], IMG_SCALE, &state.font, c_strip);
        }
        i += 1;
    }

    utils::encode_png(&img2).map_err(|_| warp::reject())
}
