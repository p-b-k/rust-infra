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

    pub const FKEY_PROD_VER: FieldSpec = FieldSpec {
        name: "fkey_prod_ver",
        default: None,
        type_def: TypeDef::FKey("product_ver"),
        nullable: false,
        unique: false,
    };

    pub const FKEY_SVC_VER: FieldSpec = FieldSpec {
        name: "fkey_svc_ver",
        default: None,
        type_def: TypeDef::FKey("service_ver"),
        nullable: false,
        unique: false,
    };
}

const FIELDS: [&FieldSpec; 2] = [&fields::FKEY_PROD_VER, &fields::FKEY_SVC_VER];

pub const PRODUCT_SERVICE: TableDef = TableDef {
    name: "product_service",
    fields: &FIELDS,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductService {
    pub fkey_prod_ver: u64,
    pub fkey_svc_ver: u64,
}

impl<'a> AsRecord<'a> for ProductService {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([
            ("fkey_prod_ver", SqlValue::Id(self.fkey_prod_ver)),
            ("fkey_svc_ver", SqlValue::Id(self.fkey_svc_ver)),
        ])
    }
}

pub type ProductServiceDO<'a> = DObj<'a, ProductService>;
pub static PRODUCT_SERVICE_FACTORY: DObjFactory<'static, ProductService> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &PRODUCT_SERVICE,
};
