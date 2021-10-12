use crate::{app::AppState, syntax};
use tokio::fs;
use tokio::task;

async fn get_preview(
    state: &'static AppState,
    uuid: String,
    ext: Option<String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let f = format!("{}/{}", &state.pastes, uuid);
    let code = fs::read_to_string(f).await.map_err(|_| warp::reject())?;

    let rendered = task::spawn_blocking(move || -> Result<Vec<u8>, warp::Rejection> {
        let res = syntax::render_preview(state, &code, ext);
        return if let Err(e) = res {
            eprintln!("err while trying to fetch preview for {}: {}", uuid, e);
            Err(warp::reject())
        } else {
            Ok(res.unwrap())
        };
    })
    .await
    .map_err(|_| warp::reject())?;

    return rendered;
}

pub async fn root_handler_known(
    state: &'static AppState,
    uuid: String,
    ext: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    get_preview(state, uuid, Some(ext)).await
}

pub async fn root_handler(
    state: &'static AppState,
    uuid: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    get_preview(state, uuid, None).await
}
