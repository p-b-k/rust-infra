////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This holds the actual, versioned schema def
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::{
    record::{AsRecord, DObj, DObjFactory},
    schema::{FieldSpec, TableDef},
    sql::SqlValue,
    svc_schema::SchemaDef as SSD,
    version::Version,
};
use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};

pub mod fields {
    use infra::schema::{DataType, FieldSpec, TypeDef};

    pub const FKEY_SCHEMA: FieldSpec = FieldSpec {
        name: "fkey_schema",
        default: None,
        type_def: TypeDef::Data(DataType::Integer),
        nullable: false,
        unique: false,
    };

    pub const SCHEMA_VER: FieldSpec = FieldSpec {
        name: "schema_ver",
        default: None,
        type_def: TypeDef::Data(DataType::Version),
        nullable: false,
        unique: false,
    };

    pub const SCHEMA_DEF: FieldSpec = FieldSpec {
        name: "schema_def",
        default: None,
        type_def: TypeDef::Data(DataType::Clob),
        nullable: true,
        unique: false,
    };
}

const FIELDS: [&FieldSpec; 3] = [
    &fields::FKEY_SCHEMA,
    &fields::SCHEMA_VER,
    &fields::SCHEMA_DEF,
];

pub const SCHEMA_DEF: TableDef = TableDef {
    name: "schema_def",
    fields: &FIELDS,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct SchemaDef {
    pub fkey_schema: u64,
    pub schema_ver: Version,
    pub schema_def: Option<SSD>,
}

impl<'a> AsRecord<'a> for SchemaDef {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([
            ("fkey_schema", SqlValue::Id(self.fkey_schema)),
            ("schema_ver", SqlValue::Version(self.schema_ver.clone())),
            (
                "schema_def",
                match &self.schema_def {
                    None => SqlValue::Nullable(None),
                    Some(d) => SqlValue::String(
                        serde_json::to_string(&d).expect("Unable to serialize schema def"),
                    ),
                },
            ),
        ])
    }
}

pub type SchemaDefDO<'a> = DObj<'a, SchemaDef>;
pub static SCHEMA_DEF_FACTORY: DObjFactory<'static, SchemaDef> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &SCHEMA_DEF,
};
