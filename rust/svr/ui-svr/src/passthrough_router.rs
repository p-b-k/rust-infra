////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Passthrough router
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{
    Router,
    http::StatusCode as SC,
    routing::{get, post, put},
};

use http::Response;
use infra::error::{ErrorResponse, make_error};
use std::sync::Arc;

use crate::state::AppState;

// use log::debug;

pub fn pass_through_router(app: Arc<AppState>) -> Router<()> {
    Router::new().route(
        "pass/*",
        get(get_pass_through)
            .post(post_pass_through)
            .put(put_pass_through)
            .with_state(app),
    )
}

async fn get_pass_through() -> Result<Response<String>, ErrorResponse> {
    Err(make_error(
        SC::NOT_IMPLEMENTED,
        String::from("GET Pass Through not implimented"),
    ))
}

async fn post_pass_through() -> Result<Response<String>, ErrorResponse> {
    Err(make_error(
        SC::NOT_IMPLEMENTED,
        String::from("POST Pass Through not implimented"),
    ))
}

async fn put_pass_through() -> Result<Response<String>, ErrorResponse> {
    Err(make_error(
        SC::NOT_IMPLEMENTED,
        String::from("PUT Pass Through not implimented"),
    ))
}
