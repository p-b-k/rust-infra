////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Handle for dynamic Json requests
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{Json, Router, extract::State, routing::get};

use mysql::prelude::{FromRow, Queryable};
use std::sync::Arc;

use infra::state::AppState;
use ui::table::{ColumnDef, TableDef};

use crate::data::{Product, Service};

use log::info;

// use log::{info, warn};

pub fn json_router(app: Arc<AppState>) -> Router<()> {
    Router::new()
        .route("/test/prod/table/head", get(get_prod_test_head))
        .route("/test/prod/table/body", get(get_prod_test_body))
        .route("/test/prod/table/refresh", get(get_prod_test_refresh))
        .route("/test/svc/table/head", get(get_svc_test_head))
        .route("/test/svc/table/body", get(get_svc_test_body))
        .route("/test/svc/table/search", get(get_svc_test_search))
        .with_state(app)
}

pub async fn get_table_body<T, S>(state: Arc<AppState>, query: &String, proc: fn(T) -> S) -> Vec<S>
where
    T: FromRow,
    S: Clone,
{
    info!(target: "get_table_body", "calling: {query}");
    let mut pool = state.pool.lock().unwrap();

    let mut_pool = pool.as_mut();
    let mut conn = mut_pool.unwrap().get_conn().unwrap();

    let items = &conn.query_map(query, proc).unwrap();

    items.to_vec()
}

async fn get_prod_test_head() -> Json<Box<TableDef>> {
    Json(Box::new(TableDef {
        title: String::from("Products"),
        search_url: None,
        refresh_url: Some(String::from("/test/prod/table/refresh")),
        columns: Box::new(Vec::from([
            ColumnDef {
                column: String::from("prod_id"),
                class: Some(String::from("test_id")),
                text: String::from("Product Id"),
            },
            ColumnDef {
                column: String::from("prod_name"),
                class: Some(String::from("test_name")),
                text: String::from("Product Name"),
            },
        ])),
    }))
}

async fn get_prod_test_body(State(state): State<Arc<AppState>>) -> Json<Vec<Product>> {
    let products = get_table_body(
        state,
        &String::from("SELECT pkey, prod_id, prod_name FROM product"),
        |(pkey, prod_id, prod_name)| Product {
            pkey,
            prod_id,
            prod_name,
        },
    )
    .await;

    Json(products)
}

async fn get_prod_test_refresh(State(state): State<Arc<AppState>>) -> Json<Vec<Product>> {
    let products = get_table_body(
        state,
        &String::from("SELECT pkey, prod_id, prod_name FROM product"),
        |(pkey, prod_id, prod_name)| Product {
            pkey,
            prod_id,
            prod_name,
        },
    )
    .await;

    Json(products)
}

async fn get_svc_test_head() -> Json<Box<TableDef>> {
    Json(Box::new(TableDef {
        title: String::from("Services"),
        search_url: Some(String::from("/test/svc/table/search")),
        refresh_url: None,
        columns: Box::new(Vec::from([
            ColumnDef {
                column: String::from("svc_id"),
                class: Some(String::from("svc_id")),
                text: String::from("Service Id"),
            },
            ColumnDef {
                column: String::from("svc_name"),
                class: Some(String::from("svc_name")),
                text: String::from("Service Name"),
            },
        ])),
    }))
}

async fn get_svc_test_search(State(_state): State<Arc<AppState>>) -> Json<Vec<Service>> {
    Json(Vec::from([]))
}

async fn get_svc_test_body(State(state): State<Arc<AppState>>) -> Json<Vec<Service>> {
    let services = get_table_body(
        state,
        &String::from("SELECT pkey, svc_id, svc_name FROM service"),
        |(pkey, svc_id, svc_name)| Service {
            pkey,
            svc_id,
            svc_name,
        },
    )
    .await;

    Json(services)
}
