////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Test executable for lib
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::env;

use log::{debug, info};

// use infra::router::create_router;
use infra::state::{AppConfig, create_app_state};

use std::sync::Arc;

use cplane::cache_routers::static_router;
use cplane::json_routers::json_router;
use cplane::rusty::basic_router;

use axum::Router;

// use sqlx::MySqlPool;

#[tokio::main]
async fn main() {
    env_logger::init();

    let cfg = create_app_config();
    debug!("Server Config: set to run on port {}", cfg.port);
    let port = cfg.port;

    let db_url = cfg.db.to_url();

    debug!("Creating application state");
    let app = Arc::new(create_app_state(&db_url, cfg));

    let router = Router::merge(static_router(app.clone()), basic_router(app.clone()))
        .merge(json_router(app.clone()));
    debug!("Created router");

    debug!("About to start the server on port {port}");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!("Now serving requests on port {port}");
    axum::serve(listener, router).await.unwrap();

    println!("That's All Folks!");
}

fn create_app_config() -> AppConfig {
    let mut cfg = AppConfig::new();

    // Process argument values and put them in the appropriate places
    let args: Vec<String> = env::args().collect();
    let mut i = 1;

    while i < args.len() {
        let next = &args[i];
        // debug!("arg = {next:?}");
        if next == "--port" {
            i = i + 1;
            let port_str = &args[i];
            debug!(target: "read_parameters", "port_str = {port_str:?}");
            cfg.port = port_str.parse().unwrap();
        } else if next == "--login-page" {
            i = i + 1;
            cfg.login_page = args[i].clone();
            debug!(target: "read_parameters", "login_page = {}", cfg.login_page);
        } else {
            panic!("Unknown paramater: {next}");
        }

        i = i + 1;
    }

    cfg
}
