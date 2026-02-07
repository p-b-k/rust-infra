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
use ui::filecache::create_file_response;

pub fn basic_router(app: Arc<AppState>) -> Router<()> {
    Router::new()
        .route("/login", get(login_page).post(login_action))
        .route("/favicon.ico", get(favicon))
        .with_state(app)
}

// Basic Handlers
async fn login_page(State(state): State<Arc<AppState>>) -> Result<Response<String>, ErrorResponse> {
    let login_page = state.config.login_page.clone();
    // debug!(target: "login_page", "called with login page set to {login_page:?}");
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

async fn login_action() -> Result<Response<String>, ErrorResponse> {
    // debug!(target: "login_action", "called with some data, presumably");
    Err(make_error(
        SC::NOT_IMPLEMENTED,
        String::from("favicon not yet implemented"),
    ))
}

async fn favicon() -> Result<Response<String>, ErrorResponse> {
    let favicon = String::from("res/svg/icon.svg");
    let mimetype = format!("{}", mime::SVG);
    match read_to_string(&favicon) {
        Ok(contents) => Ok(create_file_response(&contents, &mimetype)),
        Err(err) => {
            error!(target: "favicon", "error getting favicon: {}", err.kind());
            Err(make_error(
                SC::INTERNAL_SERVER_ERROR,
                format!("error getting login page: {}", err.kind()),
            ))
        }
    }
}
