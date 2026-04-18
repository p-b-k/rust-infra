////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Test executable for lib
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{Router, extract::State, http::StatusCode as SC, routing::get};

use http::response::Response;

use std::fs::read_to_string;
use std::sync::Arc;

use log::error;

use infra::error::{ErrorResponse, make_error};

use crate::state::AppState;
pub fn basic_router(app: Arc<AppState>) -> Router<()> {
    Router::new()
        .route(format!("/health").as_str(), get(get_health))
        .route("/login", get(login_page).post(login_action))
        .route("/favicon.ico", get(favicon))
        .route("/pages", get(pagelist))
        .with_state(app)
}

// Just return a status 200
pub async fn get_health() {}

// Basic Handlers
async fn login_page(State(state): State<Arc<AppState>>) -> Result<Response<String>, ErrorResponse> {
    let login_page = state.config.login_page.clone();

    let mimetype = format!("{}", mime::TEXT_HTML);
    match read_to_string(&login_page) {
        Ok(contents) => {
            let builder = Response::builder().header("Content-Type", format!("{mimetype}"));
            Ok(builder.body(contents.clone()).unwrap())
        }
        Err(err) => {
            error!(target: "login_page", "error getting login page: {}", err.kind());
            Err(make_error(
                SC::INTERNAL_SERVER_ERROR,
                format!("error getting login page: {}", err.kind()),
            ))
        }
    }
}

async fn login_action() -> Result<Response<String>, ErrorResponse> {
    Err(make_error(
        SC::NOT_IMPLEMENTED,
        String::from("favicon not yet implemented"),
    ))
}

async fn favicon() -> Result<Response<String>, ErrorResponse> {
    let favicon = String::from("res/svg/icon.svg");
    let mimetype = format!("{}", mime::SVG);
    match read_to_string(&favicon) {
        Ok(contents) => {
            let builder = Response::builder().header("Content-Type", format!("{mimetype}"));
            Ok(builder.body(contents.clone()).unwrap())
        }
        Err(err) => {
            error!(target: "favicon", "error getting favicon ({favicon}): {}", err.kind());
            Err(make_error(
                SC::INTERNAL_SERVER_ERROR,
                format!("error getting favicon from {favicon}: {}", err.kind()),
            ))
        }
    }
}

async fn pagelist(State(state): State<Arc<AppState>>) -> Result<Response<String>, ErrorResponse> {
    let cache_lock = state.page_cache.lock().unwrap();

    cache_lock.map.iter().for_each(|_a| {
        println!("Got Page");
    });

    Err(make_error(
        SC::NOT_IMPLEMENTED,
        format!("Page List is not implemented yet"),
    ))
}
