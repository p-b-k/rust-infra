////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create a sample schema and print it out and stuff
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// use infra::schema::{SchemaDef, TableDef};
use std::env;
use std::str::FromStr;

use mysql::{Conn, OptsBuilder, prelude::Queryable};

use cplane::schema::build_datasource;
use infra::schema::SchemaDef;

// ---------------------------------------------------------------------------------------------------------------------
// Now create the main function
// ---------------------------------------------------------------------------------------------------------------------

struct DbConn {
    host: String,
    port: u16,
    name: Option<String>,
    user: String,
    pass: String,
}

impl DbConn {
    fn connect(&self) -> Conn {
        let opts = OptsBuilder::new()
            .user(Some(self.user.clone()))
            .pass(Some(self.pass.clone()))
            .ip_or_hostname(Some(self.host.clone()))
            .tcp_port(self.port);

        let opts = match &self.name {
            Some(name) => opts.db_name(Some(name.as_str())),
            None => opts,
        };

        Conn::new(opts).unwrap()
    }

    fn url(&self) -> String {
        match &self.name {
            Some(name) => format!(
                "mysql://{}:{}@{}:{}/{name}",
                self.user, self.pass, self.host, self.port
            ),
            None => format!(
                "mysql://{}:{}@{}:{}",
                self.user, self.pass, self.host, self.port
            ),
        }
    }
}

fn create_conn() -> DbConn {
    let mut host = String::from("localhost");
    let mut port = 3306;
    let mut app_name = String::from("cplane");
    let mut app_user = String::from("cplane_app");
    let mut app_pass = String::from("secret");

    let args: Vec<String> = env::args().collect();

    let mut i = 1;
    while i < args.len() {
        let next = &args[i];
        if next == "--host" {
            i += 1;
            host = String::from(&args[i]);
        } else if next == "--port" {
            i += 1;
            port = u16::from_str(&args[i]).unwrap();
        } else if next == "--name" {
            i += 1;
            app_name = String::from(&args[i]);
        } else if next == "--user" {
            i += 1;
            app_user = String::from(&args[i]);
        } else if next == "--pass" {
            i += 1;
            app_pass = String::from(&args[i]);
        } else {
            panic!("Unknown parameter: {next}");
        }
        i += 1;
    }

    DbConn {
        host,
        port,
        name: Some(app_name),
        user: app_user,
        pass: app_pass,
    }
}

fn create_db(conn: &mut Conn, schema: &SchemaDef) {
    for (_, table) in schema.tables.iter() {
        let sql = table.create_sql();
        println!("sql = {sql}");
        conn.exec_drop(sql, ()).unwrap();
    }
}

fn main() {
    env_logger::init();
    let ds = build_datasource();

    let db_conn = create_conn();

    let url = db_conn.url();
    println!("url = {url}");

    let mut conn = db_conn.connect();
    create_db(&mut conn, &ds.schema_def);
}
