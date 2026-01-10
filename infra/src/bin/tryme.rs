////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Test executable for lib
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use log::info;

use infra::router::create_router;
use infra::state::{AppConfig, create_app_state};

use std::sync::Arc;

// use sqlx::MySqlPool;

#[tokio::main]
async fn main() {
    env_logger::init();

    let cfg = create_app_config();
    info!("Server Config: set to run on port {}", cfg.port);
    let port = cfg.port;

    info!("Creating application state");
    let app_state = Arc::new(create_app_state(cfg));

    let router = create_router(app_state);
    info!("Created router");

    let db_name = String::from("newsrv_app");
    let db_host = String::from("localhost");
    let db_user = String::from("newsrv");
    let db_pass = String::from("secret");
    let db_port = 3306;

    let db_url = format!("mysql://{db_user}:{db_pass}@{db_host}:{db_port}/{db_name}");
    info!("About to start connection pool on {db_url:?}");
    // app_state.set_connection_pool(&db_url);

    info!("About to start the server on port {port}");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!("About to start serving requests");
    axum::serve(listener, router).await.unwrap();

    println!("That's All Folks!");
}

fn create_app_config() -> AppConfig {
    let port = 7021;
    let login_page = String::from("res/html/login.html");

    AppConfig { port, login_page }
}
