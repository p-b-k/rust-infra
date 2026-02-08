////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::datasource::DS;
use infra::schema::{DataType, FieldDef, FieldSpec, TableDef, TypeDef};
use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Account {
    pub pkey: u64,
    pub acct_id: String,
    pub acct_name: String,
}

pub fn init() -> TableDef {
    TableDef {
        name: String::from("account"),
        fields: Box::new(Vec::from([
            FieldDef::Field(FieldSpec {
                name: String::from("acct_id"),
                default: None,
                type_def: TypeDef::Data(DataType::String(64)),
                nullable: false,
                unique: true,
            }),
            FieldDef::Field(FieldSpec {
                name: String::from("acct_name"),
                default: None,
                type_def: TypeDef::Data(DataType::String(256)),
                nullable: false,
                unique: true,
            }),
        ])),
    }
}
