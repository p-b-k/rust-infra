////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Define the Control Plane schema
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

use infra::schema::{DBUser, GrantInfo, SchemaDef, TableDef};

use crate::tabs::customer::CUSTOMER;
use crate::tabs::log::LOG;
use crate::tabs::product::PRODUCT;
use crate::tabs::product_service::PRODUCT_SERVICE;
use crate::tabs::product_tenant::PRODUCT_TENANT;
use crate::tabs::product_ver::PRODUCT_VERSION;
use crate::tabs::request::REQUEST;
use crate::tabs::service::SERVICE;
use crate::tabs::service_ver::SERVICE_VERSION;
use crate::tabs::task::TASK;
use crate::tabs::tenant::TENANT;
use crate::tabs::worker::WORKER;

// use time::Time;

// ---------------------------------------------------------------------------------------------------------------------
// Create a datasource object
// ---------------------------------------------------------------------------------------------------------------------

pub fn fields_from_table(def: &TableDef) -> String {
    let mut fields = String::from("pkey");
    for field in def.fields() {
        fields.push_str(", ");
        fields.push_str(field.name);
    }

    fields
}

// ---------------------------------------------------------------------------------------------------------------------
// Define the tables
// ---------------------------------------------------------------------------------------------------------------------

// moved to tabs module

// ---------------------------------------------------------------------------------------------------------------------
// Now create the main function
// ---------------------------------------------------------------------------------------------------------------------

pub fn build_schema_def() -> SchemaDef {
    SchemaDef {
        users: Box::new(HashMap::from([(
            String::from("app"),
            DBUser {
                role_id: String::from("app"),
                grants: Box::new(Vec::from([GrantInfo::All])),
            },
        )])),

        tables: Box::new(HashMap::from([
            (String::from("log"), &LOG),
            (String::from("customer"), &CUSTOMER),
            (String::from("service"), &SERVICE),
            (String::from("service_ver"), &SERVICE_VERSION),
            (String::from("product"), &PRODUCT),
            (String::from("product_ver"), &PRODUCT_VERSION),
            (String::from("product_service"), &PRODUCT_SERVICE),
            (String::from("request"), &REQUEST),
            (String::from("task"), &TASK),
            (String::from("tenant"), &TENANT),
            (String::from("product_tenant"), &PRODUCT_TENANT),
            (String::from("worker"), &WORKER),
        ])),
    }
}
