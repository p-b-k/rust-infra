////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldDef, FieldSpec, TableDef, TypeDef};

pub fn init() -> TableDef {
    TableDef {
        name: String::from("request"),
        fields: Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("req_type"),
                default: None,
                type_def: TypeDef::Data(DataType::String(64)),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("req_start"),
                default: None,
                type_def: TypeDef::Data(DataType::Timestamp),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("req_status"),
                default: None,
                type_def: TypeDef::Data(DataType::String(64)),
                nullable: false,
                unique: true,
            }),
        ]),
    }
}
