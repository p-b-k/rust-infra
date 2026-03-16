////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// init-cp-db -- initialize the cplan database
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use cplane::app::DbConfig;

use std::env;

use log::debug;

struct AppConfig {
    pub db : DbConfig,
    pub sample : bool
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            db : DbConfig {
                name : String::from("cp"),
                user : String::from("cp_app"),
                pass : String::from("secret"),
                host : String::from("localhost"),
                port : 1521
            },
            sample : false
        }
    }

    pub fn as_url (&self) -> String {
        let host = self.db.host.clone();
        let name = self.db.name.clone();
        let port = self.db.port;
        let user = self.db.user.clone();
        let pass = self.db.pass.clone();

        format!("mysql://{user}:{pass}@{host}:{port}/{name}")
    }
}

fn main () {
    env_logger::init();

    println!("Initializing ...");
    let mut cfg = AppConfig::new();

    process_parameters(&mut cfg);

    let url = cfg.as_url();
    println!("database url = {url:?}");
}

fn process_parameters(cfg : &mut AppConfig) {
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    
    while i < args.len() {
        let next = &args[i];
        // debug!("arg = {next:?}");
        if next == "--port" {
            i = i + 1;
            let port_str = &args[i];
            debug!(target: "read_parameters", "port_str = {port_str}");
            cfg.db.port = port_str.parse().unwrap();
        } else if next == "--host" {
            i = i + 1;
            cfg.db.host = args[i].clone();
            debug!(target: "read_parameters", "host = {}", cfg.db.host);
        } else if next == "--name" {
            i = i + 1;
            cfg.db.name = args[i].clone();
            debug!(target: "read_parameters", "name{}", cfg.db.name);
        } else if next == "--user" {
            i = i + 1;
            cfg.db.user = args[i].clone();
            debug!(target: "read_parameters", "user = {}", cfg.db.user);
        } else if next == "--pass" {
            i = i + 1;
            cfg.db.pass = args[i].clone();
            debug!(target: "read_parameters", "pass = {}", cfg.db.pass);
        } else if next == "--sample" {
            cfg.sample = true;
            debug!(target: "read_parameters", "add sample data");
        } else {
            panic!("Unknown paramater: {next}");
        }

        i = i + 1;
    }
}

