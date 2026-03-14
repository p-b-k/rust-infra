////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{DataType, FieldSpec, TableDef, TypeDef};

const FIELDS: [FieldSpec; 2] = [
    FieldSpec {
        name: "cust_id",
        default: None,
        type_def: TypeDef::Data(DataType::String(64)),
        nullable: false,
        unique: true,
    },
    FieldSpec {
        name: "cust_name",
        default: None,
        type_def: TypeDef::Data(DataType::String(256)),
        nullable: false,
        unique: true,
    },
];

pub const CUSTOMER: TableDef = TableDef {
    name: "customer",
    fields: &FIELDS,
};
