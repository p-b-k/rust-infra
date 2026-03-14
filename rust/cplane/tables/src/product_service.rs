////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{FieldSpec, TableDef, TypeDef};
const FIELDS: [FieldSpec; 2] = [
    FieldSpec {
        name: "fkey_prod_ver",
        default: None,
        type_def: TypeDef::FKey("product_ver"),
        nullable: false,
        unique: false,
    },
    FieldSpec {
        name: "fkey_svc_ver",
        default: None,
        type_def: TypeDef::FKey("service_ver"),
        nullable: false,
        unique: false,
    },
];

pub const PRODUCT_SERVICE: TableDef = TableDef {
    name: "product_service",
    fields: &FIELDS,
};
