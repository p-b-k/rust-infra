////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::{
    record::{AsRecord, DObj, DObjFactory},
    schema::{DataType, FieldSpec, TableDef, TypeDef},
    sql::SqlValue,
    version::Version,
};
use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};

const FIELDS: [FieldSpec; 2] = [
    FieldSpec {
        name: "fkey_prod",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: false,
        unique: false,
    },
    FieldSpec {
        name: "prod_ver",
        default: None,
        type_def: TypeDef::Data(DataType::Version),
        nullable: false,
        unique: false,
    },
];

pub const PRODUCT_VERSION: TableDef = TableDef {
    name: "product_ver",
    fields: &FIELDS,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductVer {
    pub fkey_prod: u64,
    pub prod_ver: Version,
}

impl<'a> AsRecord<'a> for ProductVer {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([
            ("fkey_prod", SqlValue::Id(self.fkey_prod)),
            ("maj_ver", SqlValue::Version(self.prod_ver.clone())),
        ])
    }
}

pub type ProductVerDO<'a> = DObj<'a, ProductVer>;
pub static PRODUCT_VER_FACTORY: DObjFactory<'static, ProductVer> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &PRODUCT_VERSION,
};
