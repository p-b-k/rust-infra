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

    pub const PROD_ID: FieldSpec = FieldSpec {
        name: "prod_id",
        default: None,
        type_def: TypeDef::Data(DataType::String(32)),
        nullable: false,
        unique: true,
    };
    pub const PROD_NAME: FieldSpec = FieldSpec {
        name: "prod_name",
        default: None,
        type_def: TypeDef::Data(DataType::String(256)),
        nullable: false,
        unique: true,
    };
}
const FIELDS: [&FieldSpec; 2] = [&fields::PROD_ID, &fields::PROD_NAME];

pub const PRODUCT: TableDef = TableDef {
    name: "product",
    fields: &FIELDS,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Product {
    pub prod_id: String,
    pub prod_name: String,
}

impl<'a> AsRecord<'a> for Product {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([
            ("prod_id", SqlValue::String(self.prod_id.clone())),
            ("prod_name", SqlValue::String(self.prod_name.clone())),
        ])
    }
}

pub type ProductDO<'a> = DObj<'a, Product>;
pub static PRODUCT_FACTORY: DObjFactory<'static, Product> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &PRODUCT,
};
