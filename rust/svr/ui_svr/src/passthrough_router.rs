////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Passthrough router
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{Json, Router, extract::State, http::StatusCode as SC, routing::{post, put, get}};

use mysql::prelude::{FromRow, Queryable};
use http::Response;
use std::sync::Arc;
use infra::error::{make_error, ErrorResponse};

use crate::state::AppState;
use ui::table::{ColumnDef, TableDef};

// use log::debug;

pub fn pass_through_router (app : Arc<AppState>) -> Router<()> {
    Router::new()
        .route("pass/*", get(get_pass_through)
            .post(post_pass_through)
            .put(put_pass_through)
            .with_state(app))
}

fn get_pass_through () -> Result<Response<String>, ErrorResponse>{
    Err(make_error(SC::NOT_IMPLEMENTED, String::from("GET Pass Through not implimented")))
}
fn post_pass_through ()-> Result<Response<String>, ErrorResponse>{
    Err(make_error(SC::NOT_IMPLEMENTED, String::from("POST Pass Through not implimented")))
}
fn put_pass_through ()-> Result<Response<String>, ErrorResponse>{
    Err(make_error(SC::NOT_IMPLEMENTED, String::from("PUT Pass Through not implimented")))
}
