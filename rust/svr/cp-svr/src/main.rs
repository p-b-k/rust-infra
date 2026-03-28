////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Main Server for the Control Plane.
//
// This is the main manager and controller, accessable through the ui-svr and interacting with wb_svrs
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod json_routers;
pub mod state;

use std::env;

use infra::status_router::status_router;
use log::{debug, info};

use crate::state::{AppConfig, create_app_state};

use std::sync::Arc;

use crate::json_routers::json_router;

#[tokio::main]
async fn main() {
    env_logger::init();

    let cfg = create_app_config();
    debug!("Server Config: set to run on port {}", cfg.port);
    let port = cfg.port;

    let db_url = cfg.db.to_url();

    debug!("Creating application state");
    let app = Arc::new(create_app_state(&db_url, cfg));

    let router = status_router().merge(json_router(app.clone()));
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
            debug!(target: "read_parameters", "port str = {port_str:?}");
            cfg.port = port_str.parse().unwrap();
        } else if next == "--login-page" {
            i = i + 1;
            cfg.login_page = args[i].clone();
            debug!(target: "read_parameters", "login page = {}", cfg.login_page);
        } else if next == "--db-name" {
            i = i + 1;
            cfg.db.name = args[i].clone();
            debug!(target: "read_parameters", "db name = {}", cfg.db.name);
        } else if next == "--db-host" {
            i = i + 1;
            cfg.db.host = args[i].clone();
            debug!(target: "read_parameters", "db host = {}", cfg.db.host);
        } else if next == "--db-user" {
            i = i + 1;
            cfg.db.user = args[i].clone();
            debug!(target: "read_parameters", "db user = {}", cfg.db.user);
        } else if next == "--db-pass" {
            i = i + 1;
            cfg.db.pass = args[i].clone();
            debug!(target: "read-parameters", "db-pass = {}", cfg.db.pass);
        } else if next == "--dev-mode" {
            cfg.dev_mode = true;
            debug!(target: "read-parameters", "db-pass = {}", cfg.db.pass);
        } else if next == "--db-port" {
            i = i + 1;
            let port_str = &args[i];
            debug!(target: "read_parameters", "db port str = {port_str:?}");
            cfg.db.port = port_str.parse().unwrap();
        } else {
            panic!("Unknown paramater: {next}");
        }

        i = i + 1;
    }

    cfg
}
