////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Passthrough router - passes calls to cp-svr and returns the result
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{Router, extract::Path, extract::State, http::StatusCode as SC, routing::get};

use http::Response;
// use http::response::Response;

use infra::error::{ErrorResponse, make_error};
use log::info;
use std::sync::Arc;

use crate::state::AppState;

use log::warn;

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

    // let mut request = Request::builder().uri(pt_url);
    let resp = reqwest::get(pt_url).await;
    match resp {
        Ok(r) => match r.text().await {
            Ok(body) => {
                info!("Got Body: {body}");
                let builder = Response::builder()
                    .header("Content-Type", format!("{}", mime::APPLICATION_JSON));

                let response = builder.body(body.clone()).unwrap();
                Ok(response)
            }
            Err(err) => {
                warn!("{err}");
                Err(make_error(
                    SC::INTERNAL_SERVER_ERROR,
                    format!("Error: {}", err.to_string()),
                ))
            }
        },
        Err(err) => {
            warn!("{err}");
            Err(make_error(
                SC::INTERNAL_SERVER_ERROR,
                format!("Error: {}", err.to_string()),
            ))
        }
    }
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
