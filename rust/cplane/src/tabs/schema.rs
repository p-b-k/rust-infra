////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This holds a schema object( but not the def)
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

    pub const SCHEMA_NAME: FieldSpec = FieldSpec {
        name: "schema_name",
        default: None,
        type_def: TypeDef::Data(DataType::String(256)),
        nullable: false,
        unique: true,
    };
    pub const SCHEMA_DESC: FieldSpec = FieldSpec {
        name: "prod_desc",
        default: None,
        type_def: TypeDef::Data(DataType::String(1024)),
        nullable: true,
        unique: false,
    };
}
const FIELDS: [&FieldSpec; 2] = [&fields::SCHEMA_NAME, &fields::SCHEMA_DESC];

pub const SCHEMA: TableDef = TableDef {
    name: "schema",
    fields: &FIELDS,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Schema {
    pub schema_name: String,
    pub schema_desc: String,
}

impl<'a> AsRecord<'a> for Schema {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([
            ("schema_name", SqlValue::String(self.schema_name.clone())),
            ("schema_dist", SqlValue::String(self.schema_desc.clone())),
        ])
    }
}

pub type SchemaDO<'a> = DObj<'a, Schema>;
pub static SCHEMA_FACTORY: DObjFactory<'static, Schema> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &SCHEMA,
};
