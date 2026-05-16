////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Support for Services pages back ends.
//
// 1. Services table on /pages/services
// 2. Service details page on /pages/services/{svc_id}
// 3. Update/Create service handlers
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{Json, Router, extract::State, routing::get};
use cplane::ro::services::{ServiceMainRecord, get_main_services};

use std::sync::Arc;

use crate::state::AppState;
use ui::table::{ColumnDef, TableDef};

use log::{debug, error};

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
) -> Json<Option<Vec<ServiceMainRecord>>> {
    let return_value = match get_main_services() {
        Ok(services) => Some(services),
        Err(e) => {
            error!("Error getting services: {}", e.to_string());
            None
        }
    };

    Json(return_value)
}
