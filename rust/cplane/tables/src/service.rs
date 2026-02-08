////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldDef, FieldSpec, TableDef, TypeDef};

pub fn init() -> TableDef {
    TableDef {
        name: String::from("service"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("svc_id"),
                default: None,
                type_def: TypeDef::Data(DataType::String(32)),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("svc_name"),
                default: None,
                type_def: TypeDef::Data(DataType::String(256)),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("is_global"),
                default: Some(String::from("'Y'")),
                type_def: TypeDef::Data(DataType::Boolean),
                nullable: false,
                unique: false,
            }),
        ])),
    }
}
