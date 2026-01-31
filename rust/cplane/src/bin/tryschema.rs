////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create a sample schema and print it out and stuff
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::SchemaDef;

use cplane::schema::build_schema_def;

use serde_json::{from_str, to_string};

// ---------------------------------------------------------------------------------------------------------------------
// Now create the main function
// ---------------------------------------------------------------------------------------------------------------------

fn main() {
    env_logger::init();

    let schema_def = build_schema_def();

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
