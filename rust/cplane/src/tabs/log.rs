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

const FIELDS: [FieldSpec; 2] = [
    FieldSpec {
        name: "log_level",
        default: None,
        type_def: TypeDef::Data(DataType::String(32)),
        nullable: false,
        unique: true,
    },
    FieldSpec {
        name: "log_scope",
        default: None,
        type_def: TypeDef::Data(DataType::String(256)),
        nullable: false,
        unique: true,
    },
];

pub const LOG: TableDef = TableDef {
    name: "log",
    fields: &FIELDS,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Log {
    pub log_level: String,
    pub log_scope: String,
}

impl<'a> AsRecord<'a> for Log {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([
            ("log_level", SqlValue::String(self.log_level.clone())),
            ("log_scope", SqlValue::String(self.log_scope.clone())),
        ])
    }
}

pub type ProductDO<'a> = DObj<'a, Log>;
pub static LOG_FACTORY: DObjFactory<'static, Log> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &LOG,
};
