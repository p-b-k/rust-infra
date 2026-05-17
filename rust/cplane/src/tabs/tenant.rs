////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::{
    record::{AsRecord, DObj, DObjFactory},
    schema::{FieldSpec, TableDef},
    sql::SqlValue,
};
use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};

pub mod fields {
    use infra::schema::{FieldSpec, TypeDef};

    pub const FKEY_CUST: FieldSpec = FieldSpec {
        name: "fkey_cust",
        default: None,
        type_def: TypeDef::FKey("customer"),
        nullable: false,
        unique: false,
    };
}

const FIELDS: [&FieldSpec; 1] = [&fields::FKEY_CUST];

pub const TENANT: TableDef = TableDef {
    name: "tenant",
    fields: &FIELDS,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Tenant {
    pub fkey_acct: u64,
}

impl<'a> AsRecord<'a> for Tenant {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([("fkey_acct", SqlValue::Id(self.fkey_acct))])
    }
}

pub type TenantDO<'a> = DObj<'a, Tenant>;
pub static TENANT_FACTORY: DObjFactory<'static, Tenant> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &TENANT,
};
