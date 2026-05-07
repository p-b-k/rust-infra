////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Log table and DObj
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::{
    record::{AsRecord, DObj, DObjFactory},
    schema::{DataType, FieldSpec, TableDef, TypeDef},
    sql::SqlValue,
};

use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};

const FIELDS: [FieldSpec; 4] = [
    FieldSpec {
        name: "log_level",
        default: None,
        type_def: TypeDef::Data(DataType::String(32)),
        nullable: false,
        unique: false,
    },
    FieldSpec {
        name: "fkey_req",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: true,
        unique: false,
    },
    FieldSpec {
        name: "fkey_step",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: true,
        unique: false,
    },
    FieldSpec {
        name: "msg",
        default: None,
        type_def: TypeDef::Data(DataType::String(512)),
        nullable: true,
        unique: false,
    },
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
