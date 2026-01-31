////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create a sample schema and print it out and stuff
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{
    DBUser, DataType, FieldDef, FieldSpec, GrantInfo, SchemaDef, TableDef, TypeDef,
};

use serde_json::{from_str, to_string};

// ---------------------------------------------------------------------------------------------------------------------
// Define the tables
// ---------------------------------------------------------------------------------------------------------------------

fn mk_acct() -> TableDef {
    TableDef {
        name: String::from("account"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("acct_id"),
                type_def: TypeDef::Data(DataType::String(64)),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("acct_name"),
                type_def: TypeDef::Data(DataType::String(256)),
                nullable: false,
                unique: true,
            }),
        ])),
    }
}

fn mk_prod() -> TableDef {
    TableDef {
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
    }
}

fn mk_prod_ver() -> TableDef {
    TableDef {
        name: String::from("product_ver"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_prod"),
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("maj_ver"),
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("min_ver"),
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("rel_ver"),
                type_def: TypeDef::Data(DataType::Integer),
                nullable: true,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("bld_ver"),
                type_def: TypeDef::Data(DataType::Integer),
                nullable: true,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("bld_tag"),
                type_def: TypeDef::Data(DataType::String(128)),
                nullable: false,
                unique: true,
            }),
        ])),
    }
}

fn mk_svc() -> TableDef {
    TableDef {
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
    }
}

fn mk_svc_ver() -> TableDef {
    TableDef {
        name: String::from("service_ver"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_svc"),
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("maj_ver"),
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("min_ver"),
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("rel_ver"),
                type_def: TypeDef::Data(DataType::Integer),
                nullable: true,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("bld_ver"),
                type_def: TypeDef::Data(DataType::Integer),
                nullable: true,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("bld_tag"),
                type_def: TypeDef::Data(DataType::String(128)),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("schema"),
                type_def: TypeDef::Data(DataType::Clob),
                nullable: false,
                unique: true,
            }),
        ])),
    }
}

fn mk_prod_svc() -> TableDef {
    TableDef {
        name: String::from("service"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_prod_ver"),
                type_def: TypeDef::FKey(String::from("product")),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_svc_ver"),
                type_def: TypeDef::FKey(String::from("service")),
                nullable: false,
                unique: true,
            }),
        ])),
    }
}

fn mk_req() -> TableDef {
    TableDef {
        name: String::from("request"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("req_type"),
                type_def: TypeDef::Data(DataType::String(64)),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("req_start"),
                type_def: TypeDef::Data(DataType::Timestamp),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("req_status"),
                type_def: TypeDef::Data(DataType::String(64)),
                nullable: false,
                unique: true,
            }),
        ])),
    }
}

fn mk_task() -> TableDef {
    TableDef {
        name: String::from("task"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_prod_ver"),
                type_def: TypeDef::FKey(String::from("product")),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_req"),
                type_def: TypeDef::FKey(String::from("request")),
                nullable: false,
                unique: true,
            }),
        ])),
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Now create the main function
// ---------------------------------------------------------------------------------------------------------------------

fn main() {
    env_logger::init();

    let account = mk_acct();
    let product_def = mk_prod();
    let product_ver_def = mk_prod_ver();
    let service_def = mk_svc();
    let service_ver_def = mk_svc_ver();
    let product_service = mk_prod_svc();
    let request = mk_req();
    let task = mk_task();

    let schema_def = SchemaDef {
        users: Box::new(Vec::from([DBUser {
            role_id: String::from("app"),
            grants: Box::new(Vec::from([GrantInfo::All])),
        }])),
        tables: Box::new(Vec::from([
            account,
            service_def,
            service_ver_def,
            product_def,
            product_ver_def,
            product_service,
            request,
            task,
        ])),
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
