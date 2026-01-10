////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Configure the the router
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{
    Router, body::Body, extract::Path, extract::State, http::StatusCode as SC,
    response::IntoResponse, routing::get,
};

use http::response::Response;

use crate::{
    filecache::{FileCache, StaticFileCacheLogic, create_file_response},
    state::AppState,
};

use log::{error, info, warn};

use std::{
    fs::{exists, read_to_string},
    sync::Arc,
};

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Error Response: contains both an error code and some kind of helpful message

struct ErrorResponse {
    status_code: SC,
    error_msg: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(self.status_code)
            .body(Body::from(self.error_msg))
            .unwrap()
    }
}

fn make_error(status_code: SC, error_msg: String) -> ErrorResponse {
    ErrorResponse {
        status_code,
        error_msg,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Create Router Object
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login", get(login_page).post(login_action))
        .route("/favicon.ico", get(favicon))
        .route("/test", get(get_test))
        .route("/icon/{name}/{context}", get(context_icon))
        .route("/static/html/{*path}", get(static_html_get))
        .route("/static/json/{*path}", get(static_json_get))
        .route("/static/css/{*path}", get(static_css_get))
        .route("/static/js/{*path}", get(static_js_get))
        .route("/static/svg/{*path}", get(static_svg_get))
        .route("live/accounts", get(try_json_get))
        .with_state(app_state)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Define the handlers
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

async fn login_page(State(state): State<Arc<AppState>>) -> Result<Response<String>, ErrorResponse> {
    let login_page = state.config.login_page.clone();
    // info!(target: "login_page", "called with login page set to {login_page:?}");
    let mimetype = format!("{}", mime::TEXT_HTML);
    match read_to_string(&login_page) {
        Ok(contents) => Ok(create_file_response(&contents, &mimetype)),
        Err(err) => {
            error!(target: "login_page", "error getting login page: {}", err.kind());
            Err(make_error(
                SC::INTERNAL_SERVER_ERROR,
                format!("error getting login page: {}", err.kind()),
            ))
        }
    }
}

async fn login_action() {
    // info!(target: "login_action", "called with some data, presumably");
}

async fn favicon() {}

async fn get_test() -> String {
    String::from("Hello World!")
}

async fn context_icon(
    Path(name): Path<String>,
    Path(context): Path<String>,
) -> Result<String, ErrorResponse> {
    info!(target: "context_icon", "called with name = {name:?} and context = {context:?}");
    Err(make_error(
        SC::NOT_IMPLEMENTED,
        String::from("context_icon handler not implemented yet"),
    ))
}

async fn static_html_get(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    // info!(target: "static_html_get", "called with path {path:?}");

    let html_cache = &mut state.html_cache.lock().unwrap();
    let static_root = html_cache.root.clone();

    let full_path = format!("{}/{}", static_root, path);

    // info!(target: "static_html_get", "full static path is {full_path:?}");

    static_get(html_cache, &full_path)
}

async fn static_json_get(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    // info!(target: "static_json_get", "called with path {path:?}");

    let json_cache = &mut state.json_cache.lock().unwrap();
    let static_root = json_cache.root.clone();

    let full_path = format!("{}/{}", static_root, path);

    // info!(target: "static_json_get", "full static path is {full_path:?}");

    static_get(json_cache, &full_path)
}

async fn static_css_get(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    // info!(target: "static_css_get", "called with path {path:?}");

    let css_cache = &mut state.css_cache.lock().unwrap();
    let static_root = css_cache.root.clone();

    let full_path = format!("{}/{}", static_root, path);

    // info!(target: "static_css_get", "full static path is {full_path:?}");

    static_get(css_cache, &full_path)
}

async fn static_js_get(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    // info!(target: "static_js_get", "called with path {path:?}");

    let js_cache = &mut state.js_cache.lock().unwrap();
    let static_root = js_cache.root.clone();

    let full_path = format!("{}/{}", static_root, path);

    // info!(target: "static_js_get", "full static path is {full_path:?}");

    static_get(js_cache, &full_path)
}

async fn static_svg_get(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    // info!(target: "static_svg_get", "called with path {path:?}");

    let svg_cache = &mut state.svg_cache.lock().unwrap();
    let static_root = svg_cache.root.clone();

    let full_path = format!("{}/{}", static_root, path);

    // info!(target: "static_svg_get", "full static path is {full_path:?}");

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

async fn try_json_get() -> Result<Response<String>, ErrorResponse> {
    Err(make_error(
        SC::NOT_IMPLEMENTED,
        String::from("The service is not implemented yet"),
    ))
}
