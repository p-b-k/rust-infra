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

    pub const CUST_ID: FieldSpec = FieldSpec {
        name: "cust_id",
        default: None,
        type_def: TypeDef::Data(DataType::String(64)),
        nullable: false,
        unique: true,
    };
    pub const CUST_NAME: FieldSpec = FieldSpec {
        name: "cust_name",
        default: None,
        type_def: TypeDef::Data(DataType::String(256)),
        nullable: false,
        unique: true,
    };
}

const FIELDS: [&FieldSpec; 2] = [&fields::CUST_ID, &fields::CUST_NAME];

pub const CUSTOMER: TableDef = TableDef {
    name: "customer",
    fields: &FIELDS,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Customer {
    pub cust_id: String,
    pub cust_name: String,
}

impl<'a> AsRecord<'a> for Customer {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([
            ("cust_id", SqlValue::String(self.cust_id.clone())),
            ("cust_name", SqlValue::String(self.cust_name.clone())),
        ])
    }
}

pub type CustomerDO<'a> = DObj<'a, Customer>;
pub static CUSTOMER_FACTORY: DObjFactory<'static, Customer> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &CUSTOMER,
};
