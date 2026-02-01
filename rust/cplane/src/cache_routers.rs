////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Test executable for lib
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{Router, extract::Path, extract::State, http::StatusCode as SC, routing::get};

use http::response::Response;

use std::fs::exists;
use std::sync::Arc;

use log::{error, warn};

use infra::error::{ErrorResponse, make_error};

use infra::filecache::{FileCache, StaticFileCacheLogic};
use infra::state::AppState;

pub fn static_router(app: Arc<AppState>) -> Router<()> {
    Router::new()
        .route("/static/html/{*path}", get(static_html_get))
        .route("/static/json/{*path}", get(static_json_get))
        .route("/static/css/{*path}", get(static_css_get))
        .route("/static/js/{*path}", get(static_js_get))
        .route("/static/svg/{*path}", get(static_svg_get))
        .with_state(app)
}
async fn static_html_get(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    // debug!(target: "static_html_get", "called with path {path:?}");

    let html_cache = &mut state.html_cache.lock().unwrap();
    let static_root = html_cache.root.clone();

    let full_path = format!("{}/{}", static_root, path);

    // debug!(target: "static_html_get", "full static path is {full_path:?}");

    static_get(html_cache, &full_path)
}

async fn static_json_get(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    // debug!(target: "static_json_get", "called with path {path:?}");

    let json_cache = &mut state.json_cache.lock().unwrap();
    let static_root = json_cache.root.clone();

    let full_path = format!("{}/{}", static_root, path);

    // debug!(target: "static_json_get", "full static path is {full_path:?}");

    static_get(json_cache, &full_path)
}

async fn static_css_get(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    // debug!(target: "static_css_get", "called with path {path:?}");

    let css_cache = &mut state.css_cache.lock().unwrap();
    let static_root = css_cache.root.clone();

    let full_path = format!("{}/{}", static_root, path);

    // debug!(target: "static_css_get", "full static path is {full_path:?}");

    static_get(css_cache, &full_path)
}

async fn static_js_get(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    // debug!(target: "static_js_get", "called with path {path:?}");

    let js_cache = &mut state.js_cache.lock().unwrap();
    let static_root = js_cache.root.clone();

    let full_path = format!("{}/{}", static_root, path);

    // debug!(target: "static_js_get", "full static path is {full_path:?}");

    static_get(js_cache, &full_path)
}

async fn static_svg_get(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    // debug!(target: "static_svg_get", "called with path {path:?}");

    let svg_cache = &mut state.svg_cache.lock().unwrap();
    let static_root = svg_cache.root.clone();

    let full_path = format!("{}/{}", static_root, path);

    // debug!(target: "static_svg_get", "full static path is {full_path:?}");

    static_get(svg_cache, &full_path)
}

fn static_get(
    cache: &mut FileCache<StaticFileCacheLogic>,
    path: &String,
) -> Result<Response<String>, ErrorResponse> {
    match exists(&path) {
        Ok(true) => match cache.lookup_file(&path) {
            Ok(content) => Ok(content),
            Err(msg) => {
                error!("{msg}");
                Err(make_error(
                    SC::INTERNAL_SERVER_ERROR,
                    format!("Error returning {path} from the cache -- {}", msg),
                ))
            }
        },
        Ok(false) => {
            warn!(target: "static_get", "No file found at {path}");
            Err(make_error(
                SC::NOT_FOUND,
                format!("Unable to find file {path}"),
            ))
        }
        Err(err) => {
            warn!(target: "static_get", "Error checking {path:?}: {}", err.kind());
            Err(make_error(
                SC::INTERNAL_SERVER_ERROR,
                String::from("Error checking if {path} exists"),
            ))
        }
    }
}
