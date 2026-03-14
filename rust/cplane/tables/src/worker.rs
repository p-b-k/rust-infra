////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldSpec, TableDef, TypeDef};

const FIELDS: [FieldSpec; 4] = [
    FieldSpec {
        name: "name",
        default: None,
        type_def: TypeDef::Data(DataType::String(32)),
        nullable: false,
        unique: true,
    },
    FieldSpec {
        name: "host",
        default: None,
        type_def: TypeDef::Data(DataType::String(128)),
        nullable: false,
        unique: false,
    },
    FieldSpec {
        name: "port",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: false,
        unique: false,
    },
    FieldSpec {
        name: "status",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: true,
        unique: false,
    },
    // FieldSpec {
    //     name: String::from("last_check"),
    //     default: None,
    //     type_def: TypeDef::Data(DataType::Timestamp),
    //     nullable: true,
    //     unique: false,
    // },
];

pub const WORKER: TableDef = TableDef {
    name: "worker",
    fields: &FIELDS,
};
