////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{FieldSpec, TableDef, TypeDef};

const FIELDS: [FieldSpec; 2] = [
    FieldSpec {
        name: "fkey_tnet",
        default: None,
        type_def: TypeDef::FKey("product_tenant"),
        nullable: false,
        unique: true,
    },
    FieldSpec {
        name: "fkey_prod_ver",
        default: None,
        type_def: TypeDef::FKey("product_ver"),
        nullable: false,
        unique: true,
    },
];

pub const PRODUCT_TENANT: TableDef = TableDef {
    name: "product_tenant",
    fields: &FIELDS,
};
