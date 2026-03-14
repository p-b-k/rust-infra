////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldDef, FieldSpec, TableDef, TypeDef};

const FIELDS: [FieldDef; 4] = [
    FieldDef::Field(FieldSpec {
        name: "name",
        default: None,
        type_def: TypeDef::Data(DataType::String(32)),
        nullable: false,
        unique: true,
    }),
    FieldDef::Field(FieldSpec {
        name: "host",
        default: None,
        type_def: TypeDef::Data(DataType::String(128)),
        nullable: false,
        unique: false,
    }),
    FieldDef::Field(FieldSpec {
        name: "port",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: false,
        unique: false,
    }),
    FieldDef::Field(FieldSpec {
        name: "status",
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
];

pub const WORKER: TableDef = TableDef {
    name: "worker",
    fields: &FIELDS,
};
