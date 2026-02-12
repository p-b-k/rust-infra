////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldDef, FieldSpec, TableDef, TypeDef};

pub fn init() -> TableDef {
    TableDef {
        name: String::from("task"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_req"),
                default: None,
                type_def: TypeDef::FKey(String::from("request")),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("status"),
                default: Some(String::from("'PENDING'")),
                type_def: TypeDef::Data(DataType::String(32)),
                nullable: false,
                unique: false,
            }),
        ])),
    }
}
