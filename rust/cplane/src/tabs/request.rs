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

    pub const REQ_TYPE: FieldSpec = FieldSpec {
        name: "req_type",
        default: None,
        type_def: TypeDef::Data(DataType::String(64)),
        nullable: false,
        unique: true,
    };

    pub const REQ_START: FieldSpec = FieldSpec {
        name: "req_start",
        default: None,
        type_def: TypeDef::Data(DataType::Timestamp),
        nullable: false,
        unique: true,
    };

    pub const REQ_STATUS: FieldSpec = FieldSpec {
        name: "req_status",
        default: None,
        type_def: TypeDef::Data(DataType::String(64)),
        nullable: false,
        unique: true,
    };
}

const FIELDS: [&FieldSpec; 3] = [&fields::REQ_TYPE, &fields::REQ_START, &fields::REQ_STATUS];

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
