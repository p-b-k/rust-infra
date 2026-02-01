////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Define the Data Objects for the control plane
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};

// use infra::schema::{
//     DBUser, DataType, FieldDef, FieldSpec, GrantInfo, SchemaDef, TableDef, TypeDef,
// };

// use infra::data_object::DObj;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Service {
    pub pkey: u64,
    pub svc_id: String,
    pub svc_name: String,
}
