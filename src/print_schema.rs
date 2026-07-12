use std::env;

use crate::schema::{SchemaDef, SchemaDefFactory, TableDef};

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create a sample schema and print it out and stuff
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
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

fn table_exists_in_schema<'a>(schema: &'a SchemaDef, table: &str) -> Option<&'static TableDef> {
    match schema.tables.get(table) {
        None => None,
        Some(v) => Some(v),
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
        TableFormat::Json => match serde_json::to_string(&table) {
            Ok(json_str) => println!("{}", json_str),
            Err(e) => println!("Error: {}", e),
        },
        TableFormat::SQL => {
            println!("{};", table.create_sql())
        }
    }
}

fn write_head(fmt: &TableFormat) {
    match fmt {
        TableFormat::Json => {
            println!("[");
        }
        _ => {}
    }
}

fn write_sep(fmt: &TableFormat) {
    match fmt {
        TableFormat::Json => {
            println!(",");
        }
        _ => {
            println!();
        }
    }
}

fn write_tail(fmt: &TableFormat) {
    match fmt {
        TableFormat::Json => {
            println!("]");
        }
        _ => {}
    }
}

pub fn write_schema_def<T: SchemaDefFactory>(fact: &T) {
    let schema_def = fact.build_schema_def();
    let cfg = create_config(&schema_def);

    write_head(&cfg.format);
    let mut first = true;

    match cfg.tables {
        None => {
            for (_, table) in schema_def.tables.iter() {
                if first {
                    first = false;
                } else {
                    write_sep(&cfg.format);
                }
                write_table(table, &cfg.format);
            }
        }
        Some(vec) => {
            for table in vec.iter() {
                write_table(table, &cfg.format);
            }
        }
    }
    write_tail(&cfg.format);
}
