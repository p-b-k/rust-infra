////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create json responses for dashboard tables
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{Json, Router, extract::State, routing::get};
use cplane::ro::requests::RequestRO;

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
        columns: Box::new(Vec::from([
            ColumnDef {
                column: String::from("req_type"),
                class: None,
                text: String::from("Request Type"),
            },
            ColumnDef {
                column: String::from("req_status"),
                class: None,
                text: String::from("Request Status"),
            },
        ])),
    }))
}

async fn get_request_body<'a>(State(_state): State<Arc<AppState>>) -> Json<Option<Vec<RequestRO>>> {
    // let mut pool = state.pool.lock().unwrap();
    // let mut_pool = pool.as_mut();
    // let mut conn: PooledConn = mut_pool.unwrap().get_conn().unwrap();

    // let requests = REQUEST_FACTORY.all(&mut conn).unwrap();

    // debug!(target: "get_request_body", "services = {requests:?}");

    Json(None)
}
