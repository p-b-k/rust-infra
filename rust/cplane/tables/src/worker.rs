////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldDef, FieldSpec, TableDef, TypeDef};

pub fn init() -> TableDef {
    TableDef {
        name: String::from("worker"),
        fields: Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("name"),
                default: None,
                type_def: TypeDef::Data(DataType::String(32)),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("host"),
                default: None,
                type_def: TypeDef::Data(DataType::String(128)),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("port"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("status"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: true,
                unique: false,
            }),
            // FieldDef::Field(FieldSpec {
            //     name: String::from("last_check"),
            //     default: None,
            //     type_def: TypeDef::Data(DataType::Timestamp),
            //     nullable: true,
            //     unique: false,
            // }),
        ]),
    }
}
