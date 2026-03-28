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

const FIELDS: [FieldSpec; 7] = [
    FieldSpec {
        name: "fkey_svc",
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
        unique: false,
    },
    FieldSpec {
        name: "schema_def",
        default: None,
        type_def: TypeDef::Data(DataType::Clob),
        nullable: true,
        unique: false,
    },
];

pub const SERVICE_VERSION: TableDef = TableDef {
    name: "service_ver",
    fields: &FIELDS,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ServiceVer {
    pub fkey_svc: u64,
    pub maj_ver: u64,
    pub min_ver: u64,
    pub rel_ver: Option<u64>,
    pub bld_ver: Option<u64>,
    pub bld_tag: Option<String>,
    // pub schema_def: Option<String>,
}

impl<'a> AsRecord<'a> for ServiceVer {
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
            ("fkey_svc", SqlValue::Id(self.fkey_svc)),
            ("maj_ver", SqlValue::Id(self.maj_ver)),
            ("min_ver", SqlValue::Id(self.maj_ver)),
            ("rel_ver", rel_ver),
            ("bld_ver", bld_ver),
            ("bld_tag", bld_tag),
        ])
    }
}

pub type ServiceVerDO<'a> = DObj<'a, ServiceVer>;
pub static SERVICE_VER_FACTORY: DObjFactory<'static, ServiceVer> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &SERVICE_VERSION,
};
