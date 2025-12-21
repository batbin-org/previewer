use crate::{app::AppState, syntax};
use tokio::task;

async fn get_preview(state: &'static AppState, uuid: String, ext: Option<String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Fetch paste content from API
    let url = format!("{}/api/v2/paste/{}", &state.api_url, uuid);
    
    let response = state.http_client
        .get(&url)
        .send()
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch paste {}: {}", uuid, e);
            warp::reject()
        })?;
    
    if !response.status().is_success() {
        eprintln!("API returned {} for paste {}", response.status(), uuid);
        return Err(warp::reject());
    }
    
    let code = response
        .text()
        .await
        .map_err(|e| {
            eprintln!("Failed to read response body for {}: {}", uuid, e);
            warp::reject()
        })?;

    let rendered = task::spawn_blocking(move || -> Result<Vec<u8>, warp::Rejection> {
        let res = syntax::render_preview(state, &code, ext);
        return if let Err(e) = res {
            eprintln!("err while trying to render preview for {}: {}", uuid, e);
            Err(warp::reject())
        } else {
            Ok(res.unwrap())
        }
    }).await.map_err(|_| warp::reject())?;

    return rendered;
}

pub async fn root_handler_known(state: &'static AppState, uuid: String, ext: String) -> Result<impl warp::Reply, warp::Rejection> {
    get_preview(state, uuid, Some(ext)).await
}

pub async fn root_handler(state: &'static AppState, uuid: String) -> Result<impl warp::Reply, warp::Rejection> {
    get_preview(state, uuid, None).await
}
