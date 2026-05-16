////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// UI Server for the Control Plane.
//
// User facing interface, including the following
// 1. Static Cached files (html, css, js, svg)
// 2. Login page and action, with user preferences and session history
// 3. Passes json requestes through to cp-svr for processing
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod cache_routers;
pub mod passthrough_router;
pub mod state;
pub mod ui_routers;

use std::env;

use log::{debug, info};

use crate::state::{AppConfig, create_app_state};
use cache_routers::static_router;
use passthrough_router::pt_router;
use ui_routers::basic_router;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    env_logger::init();

    let cfg = create_app_config();
    debug!("Server Config: set to run on port {}", cfg.port);
    let port = cfg.port;

    debug!("Creating application state");
    let app = Arc::new(create_app_state(cfg));

    let router = basic_router(app.clone())
        .merge(static_router(app.clone()))
        .merge(pt_router(app));
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
        } else if next == "--dev-mode" {
            cfg.dev_mode = true;
            debug!(target: "read_parameters", "login_page = {}", cfg.login_page);
        } else if next == "--login-page" {
            i = i + 1;
            cfg.login_page = args[i].clone();
            debug!(target: "read_parameters", "login_page = {}", cfg.login_page);
        } else if next == "--root" {
            i = i + 1;
            cfg.cache_root = args[i].clone();
            debug!(target: "read_parameters", "cache_root = {}", cfg.cache_root);
        } else if next == "--pt-port" {
            i = i + 1;
            let port_str = &args[i];
            debug!(target: "read_parameters", "port_str = {port_str:?}");
            cfg.pt.port = port_str.parse().unwrap();
        } else {
            panic!("Unknown paramater: {next}");
        }

        i = i + 1;
    }

    cfg
}
