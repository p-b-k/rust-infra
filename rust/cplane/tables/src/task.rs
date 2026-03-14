////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldSpec, TableDef, TypeDef};

const FIELDS: [FieldSpec; 2] = [
    FieldSpec {
        name: "fkey_req",
        default: None,
        type_def: TypeDef::FKey("request"),
        nullable: false,
        unique: false,
    },
    FieldSpec {
        name: "status",
        default: Some("'PENDING'"),
        type_def: TypeDef::Data(DataType::String(32)),
        nullable: false,
        unique: false,
    },
];

pub const TASK: TableDef = TableDef {
    name: "task",
    fields: &FIELDS,
};
