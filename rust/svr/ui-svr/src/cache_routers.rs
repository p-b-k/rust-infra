////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Test executable for lib
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{Router, extract::Path, extract::State, routing::get};

use http::response::Response;

use std::sync::Arc;

use infra::error::ErrorResponse;

use crate::state::AppState;

use log::info;

pub fn static_router(app: Arc<AppState>) -> Router<()> {
    Router::new()
        .route("/static/html/{*path}", get(static_html_get))
        .route("/static/json/{*path}", get(static_json_get))
        .route("/static/css/{*path}", get(static_css_get))
        .route("/static/js/{*path}", get(static_js_get))
        .route("/static/svg/{*path}", get(static_svg_get))
        .route("/page/{*path}", get(static_page_get))
        .with_state(app)
}

async fn static_html_get(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    let html_cache = &mut state.html_cache.lock().unwrap();

    html_cache.get_result(&path)
}

async fn static_json_get(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    let json_cache = &mut state.json_cache.lock().unwrap();

    json_cache.get_result(&path)
}

async fn static_css_get(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    let css_cache = &mut state.css_cache.lock().unwrap();

    css_cache.get_result(&path)
}

async fn static_js_get(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    let js_cache = &mut state.js_cache.lock().unwrap();

    js_cache.get_result(&path)
}

async fn static_svg_get(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    let svg_cache = &mut state.svg_cache.lock().unwrap();

    svg_cache.get_result(&path)
}

async fn static_page_get(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    info!("Got page request for {path:?}");
    let page_cache = &mut state.page_cache.lock().unwrap();

    page_cache.get_result(&path)
}
