////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Worker Bee for the Control Plane
//
// This looks for jobs to be run in the cp-svr and takes then and runs. It should locally store logs on the jobs,
// but return status updates and user facing logging info to the cp-svr.
//
// On startup it should register itself with the cp-svr, and on shutdown it should unregister itself. It should
// also provide a ping type endpoint to test for availability, and probably some kinds of statistics endpoints
// as well.  While we're at it we should probably add locally stored statistics as well.
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod state;
pub mod worker_routers;

use std::env;

use log::{debug, info};

use crate::state::{AppConfig, create_app_state};

use std::sync::Arc;

use crate::worker_routers::status_router;

#[tokio::main]
async fn main() {
    env_logger::init();

    let cfg = create_app_config();

    debug!("Server Config: set to run on port {}", cfg.port);
    let port = cfg.port;

    let db_url = cfg.db.to_url();

    debug!("Creating application state");
    let app = Arc::new(create_app_state(&db_url, cfg));

    let router = status_router(app.clone());
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
        } else {
            panic!("Unknown paramater: {next}");
        }

        i = i + 1;
    }

    cfg
}
