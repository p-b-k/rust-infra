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

const FIELDS: [FieldSpec; 3] = [
    FieldSpec {
        name: "fkey_svc",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: false,
        unique: false,
    },
    FieldSpec {
        name: "svc_ver",
        default: None,
        type_def: TypeDef::Data(DataType::Version),
        nullable: false,
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
    pub svc_ver: Version,
    // pub schema_def: Option<String>,
}

impl<'a> AsRecord<'a> for ServiceVer {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([
            ("fkey_svc", SqlValue::Id(self.fkey_svc)),
            ("svc_ver", SqlValue::Version(self.svc_ver.clone())),
        ])
    }
}

pub type ServiceVerDO<'a> = DObj<'a, ServiceVer>;
pub static SERVICE_VER_FACTORY: DObjFactory<'static, ServiceVer> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &SERVICE_VERSION,
};
