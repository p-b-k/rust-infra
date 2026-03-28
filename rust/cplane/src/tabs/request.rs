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

const FIELDS: [FieldSpec; 3] = [
    FieldSpec {
        name: "req_type",
        default: None,
        type_def: TypeDef::Data(DataType::String(64)),
        nullable: false,
        unique: true,
    },
    FieldSpec {
        name: "req_start",
        default: None,
        type_def: TypeDef::Data(DataType::Timestamp),
        nullable: false,
        unique: true,
    },
    FieldSpec {
        name: "req_status",
        default: None,
        type_def: TypeDef::Data(DataType::String(64)),
        nullable: false,
        unique: true,
    },
];

pub const REQUEST: TableDef = TableDef {
    name: "request",
    fields: &FIELDS,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Request {
    pub req_type: String,
    pub req_start: u64,
    pub req_status: String,
}

impl<'a> AsRecord<'a> for Request {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([
            ("req_type", SqlValue::String(self.req_type.clone())),
            ("req_start", SqlValue::Id(self.req_start)),
            ("req_status", SqlValue::String(self.req_status.clone())),
        ])
    }
}

pub type RequestDO<'a> = DObj<'a, Request>;
pub static REQUEST_FACTORY: DObjFactory<'static, Request> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &REQUEST,
};
