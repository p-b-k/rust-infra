////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::schema::{FieldDef, FieldSpec, TableDef, TypeDef};

pub fn init() -> TableDef {
    TableDef {
        name: String::from("tenant"),
        fields: Box::new(Vec::from([FieldDef::Field(FieldSpec {
            name: String::from("fkey_acct"),
            default: None,
            type_def: TypeDef::FKey(String::from("tenant")),
            nullable: false,
            unique: false,
        })])),
    }
}
