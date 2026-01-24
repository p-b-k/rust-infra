////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Handle for dynamic Json requests
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{Json, Router, extract::State, routing::get};

use mysql::prelude::{FromRow, Queryable};
use std::sync::Arc;

use infra::state::AppState;
use infra::table::ColumnDef;

use crate::data::{Product, Service};

use log::info;

// use log::{info, warn};

pub fn json_router(app: Arc<AppState>) -> Router<()> {
    Router::new()
        .route("/test/prod/table/head", get(get_prod_test_head))
        .route("/test/prod/table/body", get(get_prod_test_body))
        .route("/test/svc/table/head", get(get_svc_test_head))
        .route("/test/svc/table/body", get(get_svc_test_body))
        .with_state(app)
}

pub async fn get_table_body<T, S>(state: Arc<AppState>, query: &String, proc: fn(T) -> S) -> Vec<S>
where
    T: FromRow,
    S: Clone,
{
    info!("get_prod_test_body: calling");
    let mut pool = state.pool.lock().unwrap();

    info!("get_prod_test_body: at a");

    let mut_pool = pool.as_mut();
    info!("get_prod_test_body: at a 1");
    let mut conn = mut_pool.unwrap().get_conn().unwrap();
    info!("get_prod_test_body: at c");

    let items = &conn.query_map(query, proc).unwrap();
    info!("get_prod_test_body: at d");

    items.to_vec()
}

async fn get_prod_test_head() -> Json<Vec<ColumnDef>> {
    let head = [
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
    ];

    Json(Vec::from(head))
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

async fn get_svc_test_head() -> Json<Vec<ColumnDef>> {
    let head = [
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
    ];

    Json(Vec::from(head))
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
