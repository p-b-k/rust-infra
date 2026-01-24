////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Test executable for lib
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::env;

use log::info;

// use infra::router::create_router;
use infra::state::{AppConfig, create_app_state};

use std::sync::Arc;

use app::cache_routers::static_router;
use app::json_routers::json_router;
use app::rusty::basic_router;

use axum::Router;

// use sqlx::MySqlPool;

#[tokio::main]
async fn main() {
    env_logger::init();

    let cfg = create_app_config();
    info!("Server Config: set to run on port {}", cfg.port);
    let port = cfg.port;

    let db_url = cfg.db.to_url();

    info!("Creating application state");
    let app = Arc::new(create_app_state(&db_url, cfg));

    info!("About to start the server on port {port}");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    // let router = create_router(app_state);
    let router = Router::merge(static_router(app.clone()), basic_router(app.clone()))
        .merge(json_router(app.clone()));
    info!("Created router");

    // prepare_router(&mut router);

    info!("About to start serving requests");
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
        // info!("arg = {next:?}");
        if next == "--port" {
            i = i + 1;
            let port_str = &args[i];
            info!(target: "read_parameters", "port_str = {port_str:?}");
            cfg.port = port_str.parse().unwrap();
        } else if next == "--login-page" {
            i = i + 1;
            cfg.login_page = args[i].clone();
            info!(target: "read_parameters", "login_page = {}", cfg.login_page);
        } else {
            panic!("Unknown paramater: {next}");
        }

        i = i + 1;
    }

    cfg
}
