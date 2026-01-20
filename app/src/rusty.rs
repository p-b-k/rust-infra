////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Test executable for lib
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{Router, extract::State, http::StatusCode as SC, routing::get};

use http::response::Response;

use std::sync::Arc;

use infra::filecache::create_file_response;
use infra::state::AppState;

pub fn prepare_router(router: &mut Router<()>) {
    router.route("/login", get(login_page).post(login_action))
    // .route("/favicon.ico", get(favicon))
    // .route("/test", get(get_test))
    // .route("/icon/{name}/{context}", get(context_icon))
    // .route("/static/html/{*path}", get(static_html_get))
    // .route("/static/json/{*path}", get(static_json_get))
    // .route("/static/css/{*path}", get(static_css_get))
    // .route("/static/js/{*path}", get(static_js_get))
    // .route("/static/svg/{*path}", get(static_svg_get))
    // .route("/live/accounts", get(try_json_get))
    // .route("/test/prod/table/head", get(get_prod_test_head))
    // .route("/test/prod/table/body", get(get_prod_test_body))
    // .route("/test/svc/table/head", get(get_svc_test_head))
    // .route("/test/svc/table/body", get(get_svc_test_body));
}

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
