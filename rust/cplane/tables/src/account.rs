////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldDef, FieldSpec, TableDef, TypeDef};

const FIELDS: [FieldDef; 2] = [
    FieldDef::Field(FieldSpec {
        name: "acct_id",
        default: None,
        type_def: TypeDef::Data(DataType::String(64)),
        nullable: false,
        unique: true,
    }),
    FieldDef::Field(FieldSpec {
        name: "acct_name",
        default: None,
        type_def: TypeDef::Data(DataType::String(256)),
        nullable: false,
        unique: true,
    }),
];

pub const ACCOUNT: TableDef = TableDef {
    name: "account",
    fields: &FIELDS,
};
