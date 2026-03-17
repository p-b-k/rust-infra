////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// init-cp-db -- initialize the cplan database
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use cplane::app::DbConfig;

use std::env;

use mysql::{
     Params, Pool,
    prelude::{ Queryable},
};

use log::debug;

struct AppConfig {
    pub db: DbConfig,
    pub root_user: String,
    pub root_pass: String,
    pub sample: bool,
    pub local: bool,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            db: DbConfig {
                name: String::from("cp"),
                user: String::from("cp_app"),
                pass: String::from("secret"),
                host: String::from("localhost"),
                port: 3306,
            },
            root_user: String::from("root"),
            root_pass: String::from("secret"),
            sample: false,
            local: true,
        }
    }

    pub fn as_user_url(&self) -> String {
        let host = self.db.host.clone();
        let name = self.db.name.clone();
        let port = self.db.port;
        let user = self.db.user.clone();
        let pass = self.db.pass.clone();

        format!("mysql://{user}:{pass}@{host}:{port}/{name}")
    }

    pub fn as_root_url(&self) -> String {
        let host = self.db.host.clone();
        // let name = self.db.name.clone();
        let port = self.db.port;
        let user = self.root_user.clone();
        let pass = self.root_pass.clone();

        format!("mysql://{user}:{pass}@{host}:{port}")
    }
}

fn main() {
    env_logger::init();

    println!("Initializing ...");
    let mut cfg = AppConfig::new();

    process_parameters(&mut cfg);

    let root_url = cfg.as_root_url();
    println!("database root url = {root_url:?}");

    let pool = Pool::new(root_url.as_str()).unwrap();
    println!("Got pool ...");

    let mut conn = pool.get_conn().unwrap();
    println!("Got connection ...");

    println!("* create db = {:?}", stmt_create_db(&cfg));
    match conn.exec_drop(stmt_create_db(&cfg), Params::Empty) {
        Ok(_) => None,
        Err(_) => Some(String::from("An Error Happened"))
    };
    println!("* create user = {:?}", stmt_create_user(&cfg));
    match conn.exec_drop(stmt_create_user(&cfg), Params::Empty) {
        Ok(_) => None,
        Err(_) => Some(String::from("An Error Happened"))
    };
    println!("* grant roles to user = {:?}", stmt_grant_roles(&cfg));
    match conn.exec_drop(stmt_grant_roles(&cfg), Params::Empty) {
        Ok(_) => None,
        Err(_) => Some(String::from("An Error Happened"))
    };

    let user_url = cfg.as_user_url();
    println!("database user url = {user_url:?}");
}

// fn configure_

fn stmt_create_db(cfg: &AppConfig) -> String {
    format!("CREATE DATABASE {}", cfg.db.name)
}

fn stmt_create_user(cfg: &AppConfig) -> String {
    let user = cfg.db.user.clone();
    let host = if cfg.local {
        String::from("localhost")
    } else {
        env::var("HOSTNAME").unwrap()
    };
    let pass = cfg.db.pass.clone();

    format!("CREATE USER {user}@{host} IDENTIFIED BY '{pass}'")
}

fn stmt_grant_roles(cfg: &AppConfig) -> String {
    let user = cfg.db.user.clone();
    let host = if cfg.local {
        String::from("localhost")
    } else {
        env::var("HOSTNAME").unwrap()
    };
    let name = cfg.db.name.clone();

    format!("GRANT ALL ON {name}.* TO {user}@{host}")
}

fn process_parameters(cfg: &mut AppConfig) {
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
        } else if next == "--root-user" {
            i = i + 1;
            cfg.root_user = args[i].clone();
            debug!(target: "read_parameters", "root_user = {}", cfg.root_user);
        } else if next == "--root-user" {
            i = i + 1;
            cfg.root_pass = args[i].clone();
            debug!(target: "read_parameters", "root_pass = {}", cfg.root_pass);
        } else if next == "--sample" {
            cfg.sample = true;
            debug!(target: "read_parameters", "add sample data");
        } else {
            panic!("Unknown paramater: {next}");
        }

        i = i + 1;
    }
}
