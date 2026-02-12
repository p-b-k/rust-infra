////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{FieldDef, FieldSpec, TableDef, TypeDef};

pub fn init() -> TableDef {
    TableDef {
        name: String::from("product_service"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_prod_ver"),
                default: None,
                type_def: TypeDef::FKey(String::from("product_ver")),
                nullable: false,
                unique: false,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("fkey_svc_ver"),
                default: None,
                type_def: TypeDef::FKey(String::from("service_ver")),
                nullable: false,
                unique: false,
            }),
        ])),
    }
}
