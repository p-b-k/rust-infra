////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create a sample schema and print it out and stuff
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldDef, FieldSpec, SchemaDef, TableDef, TypeDef};

fn main() {
    env_logger::init();

    let product_def = TableDef {
        name: String::from("product"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("prod_id"),
                type_def: TypeDef::Data(DataType::String(32)),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("prod_name"),
                type_def: TypeDef::Data(DataType::String(256)),
                nullable: false,
                unique: true,
            }),
        ])),
    };

    let service_def = TableDef {
        name: String::from("service"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("svc_id"),
                type_def: TypeDef::Data(DataType::String(32)),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("svc_name"),
                type_def: TypeDef::Data(DataType::String(256)),
                nullable: false,
                unique: true,
            }),
        ])),
    };

    let schema_def = SchemaDef {
        tables: Box::new(Vec::from([service_def, product_def])),
    };

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

        println!("");
        match serde_json::to_string(&schema_def) {
            Ok(json_str) => println!("{}", json_str),
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("");
    println!("==== WHOLE SCHEMA {padding}");
}
