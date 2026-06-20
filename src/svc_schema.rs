////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Defines a services schema
// TODO: This should be in the cplane crate and a generic object serialization format should be implemented here
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use log::info;
use mysql::{FromValueError, prelude::FromValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TableDef {}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct ViewDef {}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct SchemaDef {}

impl From<String> for SchemaDef {
    fn from(value: String) -> SchemaDef {
        serde_json::from_str(value.as_str())
            .expect(format!("Unable to read SchemeDev object from {value}").as_str())
    }
}

impl FromValue for SchemaDef {
    type Intermediate = String;

    fn from_value_opt(d: mysql::Value) -> Result<Self, FromValueError> {
        info!("d = {d:?}");
        // match Version::from_string(d.as_sql(true).as_str()) {
        match serde_json::from_str(d.as_sql(true).as_str()) {
            Err(_) => Err(FromValueError(d)),
            Ok(r) => Ok(r),
        }
    }
}
