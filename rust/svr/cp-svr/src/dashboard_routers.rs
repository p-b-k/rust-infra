////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create json responses for dashboard tables
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{Json, Router, extract::State, routing::get};
use cplane::{
    ro::requests::{RequestError, RequestRO},
    tabs::request::REQUEST_FACTORY,
};
use mysql::PooledConn;

use std::sync::Arc;

use crate::state::AppState;
use ui::table::{ColumnDef, TableDef};

pub fn dashboard_router(app: Arc<AppState>) -> Router<()> {
    let json_root = &app.config.pt.root;

    Router::new()
        // These are only for testing and practice
        .route(
            format!("/{json_root}/dash/jobs/head").as_str(),
            get(get_request_head),
        )
        .route(
            format!("/{json_root}/dash/jobs/body").as_str(),
            get(get_request_body),
        )
        .with_state(app)
}

async fn get_request_head(State(state): State<Arc<AppState>>) -> Json<Box<TableDef>> {
    let json_root = &state.config.pt.root;
    Json(Box::new(TableDef {
        title: String::from("Request Monitor"),
        search_url: None,
        refresh_url: Some(format!("/{json_root}/test/prod/table/refresh")),
        action: None,
        columns: Box::new(Vec::from([
            ColumnDef {
                column: String::from("req_type"),
                class: None,
                text: String::from("Request Type"),
                width: Some(200),
            },
            ColumnDef {
                column: String::from("req_status"),
                class: None,
                text: String::from("Request Status"),
                width: Some(800),
            },
        ])),
    }))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// BEGIN // This will eventuall by moved to some other module
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// END // This will eventuall by moved to some other module
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// This is the get request call that should probably stay in this module for now
async fn get_request_body<'a>(State(state): State<Arc<AppState>>) -> Json<Option<Vec<RequestRO>>> {
    let mut pool = state.pool.lock().unwrap();
    let mut_pool = pool.as_mut();
    let mut conn: PooledConn = mut_pool.unwrap().get_conn().unwrap();

    match get_notable_requests(&mut conn) {
        Ok(_vec) => {}
        Err(_e) => {}
    }

    // debug!(target: "get_request_body", "requests = {requests:?}");

    Json(None)
}

fn get_notable_requests(conn: &mut PooledConn) -> Result<Vec<RequestRO>, RequestError> {
    let _requests = REQUEST_FACTORY.all(conn).unwrap();

    Err(RequestError::new("Not Implemented"))
}
