////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Handle to test Json requests -- not for final product, just as an example of how it works
//
// Will be removed when no longer needed because real examples are already in place
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{Json, Router, extract::State, routing::get};

use cplane::{
    tabs::product::{PRODUCT_FACTORY, ProductDO},
    tabs::service::{SERVICE_FACTORY, ServiceDO},
};

use mysql::{
    PooledConn,
    prelude::{FromRow, Queryable},
};
use std::sync::Arc;

use crate::state::AppState;
use ui::table::{ColumnDef, TableDef};

use log::debug;

pub fn json_router(app: Arc<AppState>) -> Router<()> {
    let json_root = &app.config.pt.root;
    Router::new()
        // These are only for testing and practice
        .route(
            format!("/{json_root}/test/prod/table/head").as_str(),
            get(get_prod_test_head),
        )
        .route(
            format!("/{json_root}/test/prod/table/body").as_str(),
            get(get_prod_test_body),
        )
        .route(
            format!("/{json_root}/test/prod/table/refresh").as_str(),
            get(get_prod_test_refresh),
        )
        .route(
            format!("/{json_root}/test/svc/table/head").as_str(),
            get(get_svc_test_head),
        )
        .route(
            format!("/{json_root}/test/svc/table/body").as_str(),
            get(get_svc_test_body),
        )
        .route(
            format!("/{json_root}/test/svc/table/search").as_str(),
            get(get_svc_test_search),
        )
        .with_state(app)
}

pub async fn get_table_body<T, S>(state: Arc<AppState>, query: &String, proc: fn(T) -> S) -> Vec<S>
where
    T: FromRow,
    S: Clone,
{
    debug!(target: "get_table_body", "calling: {query}");
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
                class: Some(String::from("prod_id")),
                text: String::from("Product Id"),
                width: None,
            },
            ColumnDef {
                column: String::from("prod_name"),
                class: Some(String::from("test_name")),
                text: String::from("Product Name"),
                width: None,
            },
        ])),
    }))
}

async fn get_prod_test_body<'a>(State(state): State<Arc<AppState>>) -> Json<Vec<ProductDO<'a>>> {
    let mut pool = state.pool.lock().unwrap();
    let mut_pool = pool.as_mut();
    let mut conn: PooledConn = mut_pool.unwrap().get_conn().unwrap();

    let products = PRODUCT_FACTORY.all(&mut conn).unwrap();

    debug!(target: "get_prod_test_body", "products = {products:?}");

    Json(products)
}

async fn get_prod_test_refresh<'a>(State(state): State<Arc<AppState>>) -> Json<Vec<ProductDO<'a>>> {
    let mut pool = state.pool.lock().unwrap();
    let mut_pool = pool.as_mut();
    let mut conn: PooledConn = mut_pool.unwrap().get_conn().unwrap();

    let products = PRODUCT_FACTORY.all(&mut conn).unwrap();

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
                width: None,
            },
            ColumnDef {
                column: String::from("svc_name"),
                class: Some(String::from("svc_name")),
                text: String::from("Service Name"),
                width: None,
            },
        ])),
    }))
}

async fn get_svc_test_search<'a>(State(_state): State<Arc<AppState>>) -> Json<Vec<ServiceDO<'a>>> {
    Json(Vec::from([]))
}

async fn get_svc_test_body<'a>(State(state): State<Arc<AppState>>) -> Json<Vec<ServiceDO<'a>>> {
    let mut pool = state.pool.lock().unwrap();
    let mut_pool = pool.as_mut();
    let mut conn: PooledConn = mut_pool.unwrap().get_conn().unwrap();

    let services = SERVICE_FACTORY.all(&mut conn).unwrap();

    debug!(target: "get_svc_test_body", "services = {services:?}");

    Json(services)
}
