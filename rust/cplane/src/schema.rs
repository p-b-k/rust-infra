////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Define the Control Plane schema
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{collections::HashMap, sync::Arc};

use infra::schema::{DBUser, GrantInfo, SchemaDef, TableDef};

use infra::datasource::DS;

use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};
use tables::customer::CUSTOMER;
use tables::product::PRODUCT;
use tables::product_service::PRODUCT_SERVICE;
use tables::product_tenant::PRODUCT_TENANT;
use tables::product_ver::PRODUCT_VERSION;
use tables::request::REQUEST;
use tables::service::SERVICE;
use tables::service_ver::SERVICE_VERSION;
use tables::task::TASK;
use tables::tenant::TENANT;
use tables::worker::WORKER;

// use time::Time;

// ---------------------------------------------------------------------------------------------------------------------
// Create a datasource object
// ---------------------------------------------------------------------------------------------------------------------

pub fn fields_from_table(def: &TableDef) -> String {
    let mut fields = String::from("pkey");
    for field in def.fields() {
        fields.push_str(", ");
        fields.push_str(field.name());
    }

    fields
}

// ---------------------------------------------------------------------------------------------------------------------
// Define the tables
// ---------------------------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct CustomerDO {
    pub pkey: u64,
    pub cust_id: String,
    pub cust_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Product {
    pub pkey: u64,
    pub prod_id: String,
    pub prod_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductVer {
    pub pkey: u64,
    pub fkey_prod: u64,
    pub maj_ver: u32,
    pub min_ver: u32,
    pub rel_ver: Option<u32>,
    pub bld_ver: Option<u32>,
    pub bld_tag: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Service {
    pub pkey: u64,
    pub svc_id: String,
    pub svc_name: String,
    pub is_global: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ServiceVer {
    pub pkey: u64,
    pub fkey_svc: String,
    pub maj_ver: u32,
    pub min_ver: u32,
    pub rel_ver: Option<u32>,
    pub bld_ver: Option<u32>,
    pub bld_tag: Option<String>,
    pub schema_def: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductService {
    pub pkey: u64,
    pub fkey_prod: u64,
    pub fkey_svc: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Request {
    pub pkey: u64,
    pub req_type: String,
    pub req_start: u64,
    pub req_status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Tenant {
    pub pkey: u64,
    pub fkey_acct: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Task {
    pub pkey: u64,
    pub fkey_req: u64,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductTenant {
    pub pkey: u64,
    pub fkey_tnet: u64,
    pub fkey_prod_ver: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Worker {
    pub pkey: u64,
    pub name: String,
    pub host: String,
    pub port: u32,
    pub status: u32,
    // pub last_check: Time,
}

// ---------------------------------------------------------------------------------------------------------------------
// Now create the main function
// ---------------------------------------------------------------------------------------------------------------------

pub fn build_datasource() -> DataSources {
    let customer_ds: DS<CustomerDO> = DS::from(&CUSTOMER);
    let service_ds: DS<Service> = DS::from(&SERVICE);
    let service_ver_ds: DS<ServiceVer> = DS::from(&SERVICE_VERSION);
    let product_ds: DS<Product> = DS::from(&PRODUCT);
    let product_ver_ds: DS<ProductVer> = DS::from(&PRODUCT_VERSION);
    let product_service_ds: DS<ProductService> = DS::from(&PRODUCT_SERVICE);
    let request_ds: DS<Request> = DS::from(&REQUEST);
    let task_ds: DS<Task> = DS::from(&TASK);
    let tenant_ds: DS<Tenant> = DS::from(&TENANT);
    let product_tenant_ds: DS<ProductTenant> = DS::from(&PRODUCT_TENANT);
    let worker_ds: DS<Worker> = DS::from(&WORKER);

    let def = Arc::new(SchemaDef {
        users: Box::new(HashMap::from([(
            String::from("app"),
            DBUser {
                role_id: String::from("app"),
                grants: Box::new(Vec::from([GrantInfo::All])),
            },
        )])),

        tables: Box::new(HashMap::from([
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
    });

    DataSources {
        schema_def: def,
        account: customer_ds,
        service: service_ds,
        service_ver: service_ver_ds,
        product: product_ds,
        product_ver: product_ver_ds,
        product_service: product_service_ds,
        request: request_ds,
        task: task_ds,
        tenant: tenant_ds,
        product_tenant: product_tenant_ds,
        worker: worker_ds,
    }
}

pub struct DataSources {
    pub schema_def: Arc<SchemaDef>,

    // Data Sources
    pub account: DS<CustomerDO>,
    pub service: DS<Service>,
    pub service_ver: DS<ServiceVer>,
    pub product: DS<Product>,
    pub product_ver: DS<ProductVer>,
    pub product_service: DS<ProductService>,
    pub request: DS<Request>,
    pub task: DS<Task>,
    pub tenant: DS<Tenant>,
    pub product_tenant: DS<ProductTenant>,
    pub worker: DS<Worker>,
}
