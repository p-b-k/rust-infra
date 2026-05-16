////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Support for Services pages back ends.
//
// 1. Services table on /pages/services
// 2. Service details page on /pages/services/{svc_id}
// 3. Update/Create service handlers
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{Json, Router, extract::State, routing::get};
use cplane::tabs::product::ProductDO;

use std::sync::Arc;

use crate::state::AppState;
use ui::table::{ColumnDef, TableDef};

use log::debug;

pub fn services_router(app: Arc<AppState>) -> Router<()> {
    {
        let json_root = &app.config.pt.root;
        Router::new()
            // These are only for testing and practice
            .route(
                format!("/{json_root}/page/services/service-table/head").as_str(),
                get(get_services_head),
            )
            .route(
                format!("/{json_root}/page/services/service-table/body").as_str(),
                get(get_services_body),
            )
            .route(
                format!("/{json_root}/test/page/services/service-table/refresh").as_str(),
                get(get_services_body),
            )
            .with_state(app)
    }
}

async fn get_services_head() -> Json<Box<TableDef>> {
    Json(Box::new(TableDef {
        title: String::from("Services"),
        search_url: None,
        refresh_url: Some(String::from("/test/prod/table/refresh")),
        columns: Box::new(Vec::from([
            ColumnDef {
                column: String::from("svc_id"),
                class: None,
                text: String::from("Service Id"),
                width: None,
            },
            ColumnDef {
                column: String::from("svc_name"),
                class: None,
                text: String::from("Service Name"),
                width: None,
            },
            ColumnDef {
                column: String::from("version"),
                class: None,
                text: String::from("Current Version"),
                width: None,
            },
        ])),
    }))
}

async fn get_services_body<'a>(
    State(_state): State<Arc<AppState>>,
) -> Json<Option<Vec<ProductDO<'a>>>> {
    // let mut pool = state.pool.lock().unwrap();
    // let mut_pool = pool.as_mut();
    // let mut conn: PooledConn = mut_pool.unwrap().get_conn().unwrap();

    // let products = PRODUCT_FACTORY.all(&mut conn).unwrap();
    // let products = Vec::new();

    debug!(target: "get_services_body", "Not returning any data yet");

    Json(None)
}
