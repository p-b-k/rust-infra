////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create json responses for dashboard tables
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{Json, Router, extract::State, routing::get};

use cplane::schema::{REQUEST_FACTORY, RequestDO};
use log::debug;
use mysql::PooledConn;
use std::sync::Arc;

use crate::state::AppState;
use ui::table::{ColumnDef, TableDef};

pub fn json_router(app: Arc<AppState>) -> Router<()> {
    let json_root = &app.config.pt.root;

    Router::new()
        // These are only for testing and practice
        .route(
            format!("/{json_root}/dash/jobs/table/head").as_str(),
            get(get_jobs_head),
        )
        .route(
            format!("/{json_root}/test/prod/table/body").as_str(),
            get(get_jobs_body),
        )
        .with_state(app)
}

async fn get_jobs_head(State(state): State<Arc<AppState>>) -> Json<Box<TableDef>> {
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

async fn get_jobs_body<'a>(State(state): State<Arc<AppState>>) -> Json<Vec<RequestDO<'a>>> {
    let mut pool = state.pool.lock().unwrap();
    let mut_pool = pool.as_mut();
    let mut conn: PooledConn = mut_pool.unwrap().get_conn().unwrap();

    let services = REQUEST_FACTORY.all(&mut conn).unwrap();

    debug!(target: "get_jobs_body", "services = {services:?}");

    Json(services)
}
