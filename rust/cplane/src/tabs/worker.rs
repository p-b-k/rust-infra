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

    pub const NAME: FieldSpec = FieldSpec {
        name: "name",
        default: None,
        type_def: TypeDef::Data(DataType::String(32)),
        nullable: false,
        unique: true,
    };
    pub const HOST: FieldSpec = FieldSpec {
        name: "host",
        default: None,
        type_def: TypeDef::Data(DataType::String(128)),
        nullable: false,
        unique: false,
    };
    pub const PORT: FieldSpec = FieldSpec {
        name: "port",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: false,
        unique: false,
    };
    pub const STATUS: FieldSpec = FieldSpec {
        name: "status",
        default: None,
        type_def: TypeDef::Data(DataType::String(64)),
        nullable: true,
        unique: false,
    };
}

const FIELDS: [&FieldSpec; 4] = [&fields::NAME, &fields::HOST, &fields::PORT, &fields::STATUS];

pub const WORKER: TableDef = TableDef {
    name: "worker",
    fields: &FIELDS,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Worker {
    pub name: String,
    pub host: String,
    pub port: u32,
    pub status: String,
    // pub last_check: Time,
}

impl<'a> AsRecord<'a> for Worker {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([
            ("name", SqlValue::String(self.name.clone())),
            ("host", SqlValue::String(self.host.clone())),
            ("port", SqlValue::ShortU(self.port)),
            ("status", SqlValue::String(self.status.clone())),
        ])
    }
}

pub type WorkerDO<'a> = DObj<'a, Worker>;
pub static WOKER_FACTORY: DObjFactory<'static, Worker> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &WORKER,
};
