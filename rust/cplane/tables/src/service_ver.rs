////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldDef, FieldSpec, TableDef, TypeDef};

pub fn init() -> TableDef {
    TableDef {
        name: String::from("service_ver"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_svc"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("maj_ver"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("min_ver"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("rel_ver"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: true,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("bld_ver"),
                default: None,
                type_def: TypeDef::Data(DataType::Integer),
                nullable: true,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("bld_tag"),
                default: None,
                type_def: TypeDef::Data(DataType::String(128)),
                nullable: true,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("schema_def"),
                default: None,
                type_def: TypeDef::Data(DataType::Clob),
                nullable: true,
                unique: false,
            }),
        ])),
    }
}
