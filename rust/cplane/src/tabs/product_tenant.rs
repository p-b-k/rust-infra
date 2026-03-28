////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::{
    record::{AsRecord, DObj, DObjFactory},
    schema::{FieldSpec, TableDef, TypeDef},
    sql::SqlValue,
};
use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductTenant {
    pub fkey_tnet: u64,
    pub fkey_prod_ver: u64,
}

impl<'a> AsRecord<'a> for ProductTenant {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([
            ("fkey_tnet", SqlValue::Id(self.fkey_tnet)),
            ("fkey_prod_ver", SqlValue::Id(self.fkey_prod_ver)),
        ])
    }
}

pub type ProductTenantDO<'a> = DObj<'a, ProductTenant>;
pub static PRODUCT_TENANT_FACTORY: DObjFactory<'static, ProductTenant> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &PRODUCT_TENANT,
};
