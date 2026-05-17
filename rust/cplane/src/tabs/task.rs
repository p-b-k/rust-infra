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

    pub const FKEY_REQ: FieldSpec = FieldSpec {
        name: "fkey_req",
        default: None,
        type_def: TypeDef::FKey("request"),
        nullable: false,
        unique: false,
    };

    pub const STATUS: FieldSpec = FieldSpec {
        name: "status",
        default: Some("'PENDING'"),
        type_def: TypeDef::Data(DataType::String(32)),
        nullable: false,
        unique: false,
    };
}
const FIELDS: [&FieldSpec; 2] = [&fields::FKEY_REQ, &fields::STATUS];

pub const TASK: TableDef = TableDef {
    name: "task",
    fields: &FIELDS,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Task {
    pub fkey_req: u64,
    pub status: String,
}

impl<'a> AsRecord<'a> for Task {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([
            ("fkey_req", SqlValue::Id(self.fkey_req)),
            ("status", SqlValue::String(self.status.clone())),
        ])
    }
}

pub type TaskDO<'a> = DObj<'a, Task>;
pub static TASK_FACTORY: DObjFactory<'static, Task> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &TASK,
};
