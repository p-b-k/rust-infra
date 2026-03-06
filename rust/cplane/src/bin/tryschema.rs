////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create a sample schema and print it out and stuff
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{SchemaDef, TableDef};
use std::env;

use cplane::schema::build_datasource;

// ---------------------------------------------------------------------------------------------------------------------
// Now create the main function
// ---------------------------------------------------------------------------------------------------------------------

enum TableFormat {
    Display,
    SQL,
}

struct AppConfig<'a> {
    tables: Option<Box<Vec<&'a TableDef>>>,
    format: TableFormat,
}

fn string_to_table_format(fmt: &str) -> Result<TableFormat, String> {
    if fmt == "sql" {
        Ok(TableFormat::SQL)
    } else if fmt == "display" {
        Ok(TableFormat::Display)
    } else {
        Err(format!("Unkonwn format: {fmt}"))
    }
}

fn table_exists_in_schema<'a>(schema: &'a SchemaDef, table: &str) -> Option<&'static TableDef> {
    match schema.tables.get(table) {
        None => None,
        Some(v) => Some(&v),
    }
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
            table.print();
        }
        TableFormat::SQL => {
            println!("{};", table.create_sql())
        }
    }
    println!();
}

fn main() {
    env_logger::init();

    let ds = build_datasource();
    let cfg = create_config(&ds.schema_def);

    match cfg.tables {
        None => {
            for (_, table) in ds.schema_def.tables.iter() {
                write_table(table, &cfg.format);
            }
        }
        Some(vec) => {
            for table in vec.iter() {
                write_table(table, &cfg.format);
            }
        }
    }
}
