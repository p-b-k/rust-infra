////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Log table and DObj
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

    pub const LOG_LEVEL: FieldSpec = FieldSpec {
        name: "log_level",
        default: None,
        type_def: TypeDef::Data(DataType::String(32)),
        nullable: false,
        unique: false,
    };
    pub const FKEY_REQ: FieldSpec = FieldSpec {
        name: "fkey_req",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: true,
        unique: false,
    };
    pub const FKEY_STEP: FieldSpec = FieldSpec {
        name: "fkey_step",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: true,
        unique: false,
    };
    pub const MSG: FieldSpec = FieldSpec {
        name: "msg",
        default: None,
        type_def: TypeDef::Data(DataType::String(512)),
        nullable: true,
        unique: false,
    };
}

const FIELDS: [&FieldSpec; 4] = [
    &fields::LOG_LEVEL,
    &fields::FKEY_REQ,
    &fields::FKEY_STEP,
    &fields::MSG,
];

pub const LOG: TableDef = TableDef {
    name: "log",
    fields: &FIELDS,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Log {
    pub log_level: String,
    pub fkey_req: Option<u64>,
    pub fkey_step: Option<u64>,
    pub msg: String,
}

impl<'a> AsRecord<'a> for Log {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        let fkey_req = match self.fkey_req {
            Some(i) => SqlValue::Nullable(Some(Box::new(SqlValue::Id(i)))),
            None => SqlValue::Nullable(None),
        };
        let fkey_step = match self.fkey_step {
            Some(i) => SqlValue::Nullable(Some(Box::new(SqlValue::Id(i)))),
            None => SqlValue::Nullable(None),
        };
        Vec::from([
            ("log_level", SqlValue::String(self.log_level.clone())),
            ("fkey_req", fkey_req),
            ("fkey_step", fkey_step),
            ("msg", SqlValue::String(self.msg.clone())),
        ])
    }
}

pub type ProductDO<'a> = DObj<'a, Log>;
pub static LOG_FACTORY: DObjFactory<'static, Log> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &LOG,
};
