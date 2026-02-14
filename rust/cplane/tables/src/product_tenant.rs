////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{FieldDef, FieldSpec, TableDef, TypeDef};

pub fn init() -> TableDef {
    TableDef {
        name: String::from("product_tenant"),
        fields: Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_tnet"),
                default: None,
                type_def: TypeDef::FKey(String::from("product_tenant")),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_prod_ver"),
                default: None,
                type_def: TypeDef::FKey(String::from("product_ver")),
                nullable: false,
                unique: true,
            }),
        ]),
    }
}
