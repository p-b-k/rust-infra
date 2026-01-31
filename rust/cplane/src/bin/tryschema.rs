////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create a sample schema and print it out and stuff
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::SchemaDef;
use std::env;

use cplane::schema::build_schema_def;

use serde_json::{from_str, to_string};

// ---------------------------------------------------------------------------------------------------------------------
// Now create the main function
// ---------------------------------------------------------------------------------------------------------------------

enum TableFormat {
    Display,
    SQL,
    Json,
}

struct AppConfig {
    tables: Option<Box<Vec<String>>>,
    format: TableFormat,
}

fn string_to_table_format(fmt: &str) -> Result<TableFormat, String> {
    if fmt == "json" {
        Ok(TableFormat::Json)
    } else if fmt == "sql" {
        Ok(TableFormat::SQL)
    } else if fmt == "display" {
        Ok(TableFormat::Display)
    } else {
        Err(format!("Unkonwn format: {fmt}"))
    }
}

fn create_config(schema_def: &SchemaDef) -> AppConfig {
    let mut tab_vec = Box::new(Vec::new());
    let mut format = TableFormat::Display;

    let args: Vec<String> = env::args().collect();

    let mut i = 0;
    while i < args.len() {
        i = i + 1;
        let next = &args[i];
        if next == "--fmt" {
            i = i + i;
            format = string_to_table_format(&args[i]).unwrap();
        } else if next == "--table" {
            i = i + i;
            let table = &args[i];
            tab_vec.push(String::from(table));
        } else {
            panic!("Unknown parameter: {next}");
        }
    }

    let tables = if tab_vec.is_empty() {
        None
    } else {
        Some(tab_vec)
    };

    AppConfig { tables, format }
}

fn main() {
    env_logger::init();

    let schema_def = build_schema_def();
    let cfg = create_config(&schema_def);

    // schema_def.display();
    let padding = "=============";
    for table in schema_def.tables() {
        let name = table.name.to_uppercase();
        println!("==== {name} {padding}");
        println!("");
        println!("{table}");
        println!("");
        println!("{};", table.create_sql());
        println!("");
        match serde_json::to_string(&table) {
            Ok(json_str) => println!("{}", json_str),
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("");
    println!("==== WHOLE SCHEMA {padding}");
    println!("");

    match to_string(&schema_def) {
        Ok(json_str) => {
            println!("{}", json_str);
            let res: Result<SchemaDef, serde_json::Error> = from_str(json_str.as_str());
            println!("");
            match res {
                Ok(new_def) => {
                    println!("==== Read Schema");
                    println!("");
                    if new_def == schema_def {
                        println!("We have a match!");
                    } else {
                        println!("No matches here :()");
                    }
                }
                Err(e) => {
                    println!("Error: {}", e)
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e)
        }
    }
}
