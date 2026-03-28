////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldSpec, TableDef, TypeDef};

const FIELDS: [FieldSpec; 3] = [
    FieldSpec {
        name: "svc_id",
        default: None,
        type_def: TypeDef::Data(DataType::String(32)),
        nullable: false,
        unique: true,
    },
    FieldSpec {
        name: "svc_name",
        default: None,
        type_def: TypeDef::Data(DataType::String(256)),
        nullable: false,
        unique: true,
    },
    FieldSpec {
        name: "is_global",
        default: Some("'Y'"),
        type_def: TypeDef::Data(DataType::Boolean),
        nullable: false,
        unique: false,
    },
];

pub const SERVICE: TableDef = TableDef {
    name: "service",
    fields: &FIELDS,
};
