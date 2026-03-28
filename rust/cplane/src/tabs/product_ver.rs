////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::{
    record::{AsRecord, DObj, DObjFactory},
    schema::{DataType, FieldSpec, TableDef, TypeDef},
    sql::SqlValue,
};
use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};

const FIELDS: [FieldSpec; 6] = [
    FieldSpec {
        name: "fkey_prod",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: false,
        unique: false,
    },
    FieldSpec {
        name: "maj_ver",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: false,
        unique: false,
    },
    FieldSpec {
        name: "min_ver",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: false,
        unique: false,
    },
    FieldSpec {
        name: "rel_ver",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: true,
        unique: false,
    },
    FieldSpec {
        name: "bld_ver",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: true,
        unique: false,
    },
    FieldSpec {
        name: "bld_tag",
        default: None,
        type_def: TypeDef::Data(DataType::String(128)),
        nullable: true,
        unique: true,
    },
];

pub const PRODUCT_VERSION: TableDef = TableDef {
    name: "product_ver",
    fields: &FIELDS,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductVer {
    pub fkey_prod: u64,
    pub maj_ver: u64,
    pub min_ver: u64,
    pub rel_ver: Option<u64>,
    pub bld_ver: Option<u64>,
    pub bld_tag: Option<String>,
}

impl<'a> AsRecord<'a> for ProductVer {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        let rel_ver = match self.rel_ver {
            Some(i) => SqlValue::Nullable(Some(Box::new(SqlValue::Id(i)))),
            None => SqlValue::Nullable(None),
        };

        let bld_ver = match self.bld_ver {
            Some(i) => SqlValue::Nullable(Some(Box::new(SqlValue::Id(i)))),
            None => SqlValue::Nullable(None),
        };

        let bld_tag = match self.bld_tag.as_ref() {
            Some(s) => SqlValue::Nullable(Some(Box::new(SqlValue::String(s.clone())))),
            None => SqlValue::Nullable(None),
        };

        Vec::from([
            ("fkey_prod", SqlValue::Id(self.fkey_prod)),
            ("maj_ver", SqlValue::Id(self.maj_ver)),
            ("min_ver", SqlValue::Id(self.maj_ver)),
            ("rel_ver", rel_ver),
            ("bld_ver", bld_ver),
            ("bld_tag", bld_tag),
        ])
    }
}

pub type ProductVerDO<'a> = DObj<'a, ProductVer>;
pub static PRODUCT_VER_FACTORY: DObjFactory<'static, ProductVer> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &PRODUCT_VERSION,
};
