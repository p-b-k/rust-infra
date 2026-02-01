////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create a sample schema and print it out and stuff
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{SchemaDef, TableDef};
use std::env;

use cplane::schema::build_schema_def;

// ---------------------------------------------------------------------------------------------------------------------
// Now create the main function
// ---------------------------------------------------------------------------------------------------------------------

enum TableFormat {
    Display,
    SQL,
    Json,
}

struct AppConfig<'a> {
    tables: Option<Box<Vec<&'a TableDef>>>,
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

fn table_exists_in_schema<'a>(schema: &'a SchemaDef, table: &str) -> Option<&'a TableDef> {
    let mut result = None;

    let mut i = 0;
    while i < schema.tables.len() {
        let table_def = &schema.tables[i];
        if table == table_def.name {
            result = Some(table_def);
            break;
        }

        i += 1;
    }

    result
}

fn create_config(schema_def: &SchemaDef) -> AppConfig<'_> {
    let mut tab_vec = Box::new(Vec::new());
    let mut format = TableFormat::Display;

    let args: Vec<String> = env::args().collect();

    let mut i = 1;
    while i < args.len() {
        let next = &args[i];
        if next == "--fmt" {
            i += 1;
            format = string_to_table_format(&args[i]).unwrap();
        } else if next == "--table" {
            i += 1;
            let table = &args[i];
            match table_exists_in_schema(&schema_def, &table) {
                Some(table_def) => tab_vec.push(table_def),
                None => panic!("Table {table} does not exist in the schema"),
            }
        } else {
            panic!("Unknown parameter: {next}");
        }
        i += 1;
    }

    let tables = if tab_vec.is_empty() {
        None
    } else {
        Some(tab_vec)
    };

    AppConfig { tables, format }
}

fn write_table(table: &TableDef, fmt: &TableFormat) {
    match fmt {
        TableFormat::Display => {
            println!("{table}")
        }
        TableFormat::Json => match serde_json::to_string(&table) {
            Ok(json_str) => println!("{}", json_str),
            Err(e) => println!("Error: {}", e),
        },
        TableFormat::SQL => {
            println!("{};", table.create_sql())
        }
    }
    println!();
}

fn main() {
    env_logger::init();

    let schema_def = build_schema_def();
    let cfg = create_config(&schema_def);

    match cfg.tables {
        None => {
            for table in schema_def.tables() {
                write_table(table, &cfg.format);
            }
        }
        Some(vec) => {
            for table in vec.iter() {
                write_table(table, &cfg.format);
            }
        }
    }
    // schema_def.display();
    // let padding = "=============";
    // for table in schema_def.tables() {
    //     let name = table.name.to_uppercase();
    //     println!("==== {name} {padding}");
    //     println!("");
    //     println!("{table}");
    //     println!("");
    //     println!("{};", table.create_sql());
    //     println!("");
    //     match serde_json::to_string(&table) {
    //         Ok(json_str) => println!("{}", json_str),
    //         Err(e) => println!("Error: {}", e),
    //     }
    // }

    // println!("");
    // println!("==== WHOLE SCHEMA {padding}");
    // println!("");

    // match to_string(&schema_def) {
    //     Ok(json_str) => {
    //         println!("{}", json_str);
    //         let res: Result<SchemaDef, serde_json::Error> = from_str(json_str.as_str());
    //         println!("");
    //         match res {
    //             Ok(new_def) => {
    //                 println!("==== Read Schema");
    //                 println!("");
    //                 if new_def == schema_def {
    //                     println!("We have a match!");
    //                 } else {
    //                     println!("No matches here :()");
    //                 }
    //             }
    //             Err(e) => {
    //                 println!("Error: {}", e)
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         println!("Error: {}", e)
    //     }
    // }
}
