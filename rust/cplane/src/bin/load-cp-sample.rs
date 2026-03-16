////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// load-cp-sample -- load the sample data into an empty database
// User requires INSERT, SELECT, DELETE, UPDATE rights on the tables
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use cplane::{app::DbConfig, schema::build_schema_def};
use infra::schema::TableDef;
use mysql::{
    Conn, OptsBuilder, Params, Pool, PooledConn,
    prelude::{FromRow, Queryable},
};

use std::env;

use log::debug;

struct AppConfig {
    pub db: DbConfig,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            db: DbConfig {
                name: String::from("cp"),
                user: String::from("cp_app"),
                pass: String::from("secret"),
                host: String::from("localhost"),
                port: 1521,
            },
        }
    }

    pub fn as_url(&self) -> String {
        let host = self.db.host.clone();
        let name = self.db.name.clone();
        let port = self.db.port;
        let user = self.db.user.clone();
        let pass = self.db.pass.clone();

        format!("mysql://{user}:{pass}@{host}:{port}/{name}")
    }
}

fn main() {
    env_logger::init();

    println!("Initializing ...");
    let mut cfg = AppConfig::new();

    process_parameters(&mut cfg);

    let url = cfg.as_url();
    println!("database url = {url:?}");

    let def = build_schema_def();

    let conn = Pool::new(url.as_str()).unwrap();

    def.tables
        .iter()
        .for_each(|(_, tdef)| match create_table(&conn, tdef) {
            Some(err_msg) => panic!("Failed to create: {err_msg}"),
            _ => (),
        });
}

/// Return error message, or none
fn create_table<T>(conn: &PooledConn, tdef: &TableDef) -> Option<String>
where
    T: FromRow,
{
    let sql = tdef.create_sql();
    println!("{sql}");
    conn.exec_opt(sql, Params::Empty);

    None
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
        } else {
            panic!("Unknown paramater: {next}");
        }

        i = i + 1;
    }
}
