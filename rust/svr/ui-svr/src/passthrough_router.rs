////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Passthrough router - passes calls to cp-svr and returns the result
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{Router, extract::Path, extract::State, http::StatusCode as SC, routing::get};

use http::{Request, Response};
use infra::error::{ErrorResponse, make_error};
use log::info;
use std::sync::Arc;

use crate::state::AppState;

// use log::debug;

pub fn pt_router(app: Arc<AppState>) -> Router<()> {
    Router::new().route(
        "/pass/{*path}",
        get(get_pass_through)
            .post(post_pass_through)
            .put(put_pass_through)
            .with_state(app),
    )
}

async fn get_pass_through(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, ErrorResponse> {
    let pt = &state.config.pt;
    let pt_url = pt.get_passthrough_url(path.as_str());
    info!("Request URL = {pt_url}");

    let mut _request = Request::builder().uri(pt_url);

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
