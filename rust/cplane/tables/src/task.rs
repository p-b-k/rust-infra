////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldDef, FieldSpec, TableDef, TypeDef};

const FIELDS: [FieldDef; 2] = [
    FieldDef::Field(FieldSpec {
        name: "fkey_req",
        default: None,
        type_def: TypeDef::FKey("request"),
        nullable: false,
        unique: false,
    }),
    FieldDef::Field(FieldSpec {
        name: "status",
        default: Some("'PENDING'"),
        type_def: TypeDef::Data(DataType::String(32)),
        nullable: false,
        unique: false,
    }),
];

pub const TASK: TableDef = TableDef {
    name: "task",
    fields: &FIELDS,
};
