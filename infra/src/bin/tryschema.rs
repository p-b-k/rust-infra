////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Create a sample schema and print it out and stuff
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldDef, FieldSpec, SchemaDef, TableDef, TypeDef};

fn main() {
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

    schema_def.display();
}
