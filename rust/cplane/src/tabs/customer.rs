////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Mange the account table
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
        name: "cust_id",
        default: None,
        type_def: TypeDef::Data(DataType::String(64)),
        nullable: false,
        unique: true,
    },
    FieldSpec {
        name: "cust_name",
        default: None,
        type_def: TypeDef::Data(DataType::String(256)),
        nullable: false,
        unique: true,
    },
];

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
