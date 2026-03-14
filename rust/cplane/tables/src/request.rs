////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldSpec, TableDef, TypeDef};
const FIELDS: [FieldSpec; 3] = [
    FieldSpec {
        name: "req_type",
        default: None,
        type_def: TypeDef::Data(DataType::String(64)),
        nullable: false,
        unique: true,
    },
    FieldSpec {
        name: "req_start",
        default: None,
        type_def: TypeDef::Data(DataType::Timestamp),
        nullable: false,
        unique: true,
    },
    FieldSpec {
        name: "req_status",
        default: None,
        type_def: TypeDef::Data(DataType::String(64)),
        nullable: false,
        unique: true,
    },
];

pub const REQUEST: TableDef = TableDef {
    name: "request",
    fields: &FIELDS,
};
