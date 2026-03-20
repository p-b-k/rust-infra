////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// init-cp-db -- initialize the cplan database
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::env;

use infra::schema::{SchemaDef, TableDef};
use log::{info, debug, error};

use mysql::{Params, Pool, PooledConn, prelude::Queryable};

use cplane::{app::DbConfig, sample::load_sample_data, schema::build_schema_def};

#[derive(Copy, Clone, Debug)]
enum Op {
    Create(bool, bool, bool),
    Drop,
    Help,
}

struct AppConfig {
    pub op: Op,
    pub db: DbConfig,
    pub root_user: String,
    pub root_pass: String,
    pub local: bool,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            op: Op::Create(true, true, false),
            db: DbConfig {
                name: String::from("cp"),
                user: String::from("cp_app"),
                pass: String::from("secret"),
                host: String::from("localhost"),
                port: 3306,
            },
            root_user: String::from("root"),
            root_pass: String::from("secret"),
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

    let mut cfg = AppConfig::new();

    process_parameters(&mut cfg);

    match cfg.op {
        Op::Create(i, c, s) => {
            initialize(&cfg, (i, c, s));
        }
        Op::Drop => {
            drop_db(&cfg);
        }
        Op::Help => {
            write_help();
        }
    }
}

fn drop_db(cfg: &AppConfig) {
    let root_url = cfg.as_root_url();
    info!(target: "drop db", "database root url = {root_url:?}");

    let root_pool = Pool::new(root_url.as_str()).unwrap();

    let mut root_conn = root_pool.get_conn().unwrap();

    let drop_db = stmt_drop_db(&cfg);
    debug!(target: "drop_db", "{:?}", drop_db);
    match root_conn.exec_drop(drop_db, Params::Empty) {
        Ok(_) => None,
        Err(e) => Some(format!("{}", e.to_string())),
    };

    let drop_user = stmt_drop_user(&cfg);
    debug!(target: "drop_db", "{:?}", drop_user);
    match root_conn.exec_drop(drop_user, Params::Empty) {
        Ok(_) => None,
        Err(e) => Some(format!("{}", e.to_string())),
    };
}

fn initialize(cfg: &AppConfig, (init_db, init_schema, load_sample_data) : (bool, bool, bool)) {
    if init_db {
        initialize_db(cfg);
    }

    if init_schema {
        let def = build_schema_def();
        initialize_schema(cfg, &def);
    }

    if load_sample_data {
        initialize_data(cfg);
    }
}

fn initialize_data(cfg: &AppConfig) {
    let user_url = cfg.as_user_url();
    info!(target: "sample data", "database user url = {user_url:?}");

    let pool = Pool::new(user_url.as_str()).unwrap();
    debug!(target : "initalize data", "Got pool ...");

    let mut conn = pool.get_conn().unwrap();
    debug!(target : "initalize data", "Got connection ...");
    
    load_sample_data(&mut conn);
}

fn initialize_schema(cfg: &AppConfig, def: &SchemaDef) {
    let user_url = cfg.as_user_url();
    info!(target: "init schema", "database user url = {user_url:?}");

    let user_pool = Pool::new(user_url.as_str()).unwrap();
    debug!(target: "initalize_schema", "Got pool ...");

    let mut user_conn = user_pool.get_conn().unwrap();
    debug!(target: "initalize_schema", "Got connection ...");

    match init_schema(def, &mut user_conn) {
        Some(msgs) => {
            println!("Completed with errors:");
            msgs.iter().for_each(|msg| println!("{}", msg));
        }
        _ => {        }
    };
}

fn initialize_db(cfg: &AppConfig) {
    let root_url = cfg.as_root_url();
    info!(target: "create db", "database root url = {root_url:?}");

    let root_pool = Pool::new(root_url.as_str()).unwrap();
    debug!(target: "initalize_db", "Got pool ...");

    let mut root_conn = root_pool.get_conn().unwrap();
    debug!(target: "initalize_db", "Got connection ...");

    let create_db = stmt_create_db(&cfg);
    debug!(target: "initalize_db", "{:?}", create_db);
    match root_conn.exec_drop(create_db, Params::Empty) {
        Ok(_) => None,
        Err(e) => Some(format!("{}", e.to_string())),
    };

    let create_user = stmt_create_user(&cfg);
    debug!(target: "initalize_db", "{:?}", create_user);
    match root_conn.exec_drop(create_user, Params::Empty) {
        Ok(_) => None,
        Err(e) => Some(format!("{}", e.to_string())),
    };

    let grant_roles = stmt_grant_roles(&cfg);
    debug!(target: "initalize_db", "{:?}", grant_roles);
    match root_conn.exec_drop(grant_roles, Params::Empty) {
        Ok(_) => None,
        Err(e) => Some(format!("{}", e.to_string())),
    };
}

fn init_schema(def: &SchemaDef, conn: &mut PooledConn) -> Option<Vec<String>> {
    let mut vec: Vec<String> = Vec::from([]);

    def.tables
        .iter()
        .for_each(|(_, tdef)| match create_table(conn, tdef) {
            Some(err_msg) => {
                error!("Error creating table {}: {err_msg}", tdef.name);
                vec.push(format!("{}, {err_msg}", tdef.name))
            }
            _ => (),
        });

    None
}

fn create_table(conn: &mut PooledConn, tdef: &TableDef) -> Option<String> {
    let sql = tdef.create_sql();
    debug!(target: "create table", "{sql}");
    match conn.exec_drop(sql, Params::Empty) {
        Ok(_) => None,
        Err(_) => Some(String::from("An Error Happened")),
    }
}
fn stmt_create_db(cfg: &AppConfig) -> String {
    format!("CREATE DATABASE {}", cfg.db.name)
}

fn stmt_drop_db(cfg: &AppConfig) -> String {
    format!("DROP DATABASE {}", cfg.db.name)
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

fn stmt_drop_user(cfg: &AppConfig) -> String {
    let user = cfg.db.user.clone();
    let host = if cfg.local {
        String::from("localhost")
    } else {
        env::var("HOSTNAME").unwrap()
    };

    format!("DROP USER {user}@{host}")
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

fn process_parameters(cfg: &mut AppConfig) -> bool {
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    let mut valid = true;

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
        } else if next == "-c" {
            cfg.op = match cfg.op {
                Op::Create(_, c, s) => Op::Create(false, c, s),
                o => o,
            };
            debug!(target: "read_parameters", "skip database creation");
        } else if next == "-i" {
            cfg.op = match cfg.op {
                Op::Create(i, _, s) => Op::Create(i, false, s),
                o => o,
            };
            debug!(target: "read_parameters", "skip schema initalization");
        } else if next == "-s" {
            cfg.op = match cfg.op {
                Op::Create(i, c, _) => Op::Create(i, c, true),
                o => o,
            };
            debug!(target: "read_parameters", "load sample data");
        } else if next == "-h" {
            cfg.op = match cfg.op {
                Op::Create(_, _, _) => Op::Help,
                o => panic!("Cannot set operation to Help when it's already {o:?}"),
            };
            debug!(target: "read_parameters", "printing help");
        } else if next == "-d" {
            cfg.op = match cfg.op {
                Op::Create(_, _, _) => Op::Drop,
                o => panic!("Cannot set operation to Drop when it's already {o:?}"),
            };
            debug!(target: "read_parameters", "printing help");
        } else {
            let prog_path = env::current_exe().unwrap();
            let prog_name = prog_path.file_name().unwrap().to_str().unwrap();
            println!("Unknown paramater: \"{next}\", run \"{prog_name} -h\" for help");
            valid = false;
        }

        i = i + 1;
    }

    valid
}

fn write_help() {
    let prog_path = env::current_exe().unwrap();
    let prog_name = prog_path.file_name().unwrap().to_str().unwrap();
    println!(
        "{prog_name}: Prepare a database for connecting cplane to, optionally with sample data"
    );
    println!("With no options or flags it will create the database and the user and then initialize the schema");
    println!();
    println!("Options:");
    println!("--name         : cp        : The name of the database (defaults to \"cp\")");
    println!("--host         : localhost : The host that the db is running on");
    println!("--port         : 3306      : The db's port number");
    println!(
        "--user         : cp_app    : The name of the apps db connection user id (defaults to \"cp_app\")"
    );
    println!("--pass         : secret    : The name of the apps db connection user id");
    println!(
        "--root-name    : root      : The system user of the database (to create the table and users)"
    );
    println!("--root-pass    : secret    : The system user's password");
    println!();
    println!("Flags:");
    println!("-h : Show this message");
    println!("-d : Drop the database");
    println!("-c : Do not create the database or the user");
    println!("-i : Do not create the schema (tables and such)");
    println!("-s : Load the sample data");
}
