////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Test executable for lib
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::env;

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

    let db_name = String::from("rusty");
    let db_host = String::from("localhost");
    let db_user = String::from("rusty_app");
    let db_pass = String::from("secret");
    let db_port = 3306;

    let db_url = format!("mysql://{db_user}:{db_pass}@{db_host}:{db_port}/{db_name}");

    info!("Creating application state");
    let app_state = Arc::new(create_app_state(&db_url, cfg));

    // info!("About to start connection pool on {db_url:?}");
    // app_state.set_connection_pool(&db_url);

    info!("About to start the server on port {port}");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    let router = create_router(app_state);
    info!("Created router");

    info!("About to start serving requests");
    axum::serve(listener, router).await.unwrap();

    println!("That's All Folks!");
}

fn create_app_config() -> AppConfig {
    let mut port = 7021;
    let mut login_page = String::from("res/html/login.html");

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
            port = port_str.parse().unwrap();
        } else if next == "--login-page" {
            i = i + 1;
            login_page = args[i].clone();
            info!(target: "read_parameters", "login_page = {}", login_page);
        } else {
            panic!("Unknown paramater: {next}");
        }

        i = i + 1;
    }

    AppConfig { port, login_page }
}
