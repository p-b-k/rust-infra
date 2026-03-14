////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{FieldDef, FieldSpec, TableDef, TypeDef};
const FIELDS: [FieldDef; 2] = [
    FieldDef::Field(FieldSpec {
        name: "fkey_prod_ver",
        default: None,
        type_def: TypeDef::FKey("product_ver"),
        nullable: false,
        unique: false,
    }),
    FieldDef::Field(FieldSpec {
        name: "fkey_svc_ver",
        default: None,
        type_def: TypeDef::FKey("service_ver"),
        nullable: false,
        unique: false,
    }),
];

pub const PRODUCT_SERVICE: TableDef = TableDef {
    name: "product_service",
    fields: &FIELDS,
};
