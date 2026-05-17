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
    use infra::schema::{DataType, FieldSpec, TypeDef};

    pub const SVC_ID: FieldSpec = FieldSpec {
        name: "svc_id",
        default: None,
        type_def: TypeDef::Data(DataType::String(32)),
        nullable: false,
        unique: true,
    };

    pub const SVC_NAME: FieldSpec = FieldSpec {
        name: "svc_name",
        default: None,
        type_def: TypeDef::Data(DataType::String(256)),
        nullable: false,
        unique: true,
    };

    pub const IS_GLOBAL: FieldSpec = FieldSpec {
        name: "is_global",
        default: Some("'Y'"),
        type_def: TypeDef::Data(DataType::Boolean),
        nullable: false,
        unique: false,
    };
}
const FIELDS: [&FieldSpec; 3] = [&fields::SVC_ID, &fields::SVC_NAME, &fields::IS_GLOBAL];

pub const SERVICE: TableDef = TableDef {
    name: "service",
    fields: &FIELDS,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Service {
    pub svc_id: String,
    pub svc_name: String,
    pub is_global: String,
}

impl<'a> AsRecord<'a> for Service {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([
            ("svc_id", SqlValue::String(self.svc_id.clone())),
            ("svc_name", SqlValue::String(self.svc_name.clone())),
            ("is_global", SqlValue::String(self.is_global.clone())),
        ])
    }
}

pub type ServiceDO<'a> = DObj<'a, Service>;
pub static SERVICE_FACTORY: DObjFactory<'static, Service> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &SERVICE,
};
