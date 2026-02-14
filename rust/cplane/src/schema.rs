////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Define the Control Plane schema
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{collections::HashMap, sync::Arc};

use infra::schema::{DBUser, GrantInfo, SchemaDef, TableDef};

use infra::datasource::{DO, DS};

use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};
use tables::account::init as mk_acct;
use tables::product::init as mk_prod;
use tables::product_service::init as mk_prod_svc;
use tables::product_tenant::init as mk_prod_tent;
use tables::product_ver::init as mk_prod_ver;
use tables::request::init as mk_req;
use tables::service::init as mk_svc;
use tables::service_ver::init as mk_svc_ver;
use tables::task::init as mk_task;
use tables::tenant::init as mk_tent;
use tables::worker::init as mk_worker;

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
pub struct Account {
    pub acct_id: String,
    pub acct_name: String,
}

type AccountDO = DO<Account>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Product {
    pub pkey: Option<u64>,
    pub prod_id: String,
    pub prod_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductVer {
    pub pkey: Option<u64>,
    pub fkey_prod: u64,
    pub maj_ver: u32,
    pub min_ver: u32,
    pub rel_ver: Option<u32>,
    pub bld_ver: Option<u32>,
    pub bld_tag: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Service {
    pub pkey: Option<u64>,
    pub svc_id: String,
    pub svc_name: String,
    pub is_global: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ServiceVer {
    pub pkey: Option<u64>,
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
    pub pkey: Option<u64>,
    pub fkey_prod: u64,
    pub fkey_svc: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Request {
    pub pkey: Option<u64>,
    pub req_type: String,
    pub req_start: u64,
    pub req_status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Tenant {
    pub pkey: Option<u64>,
    pub fkey_acct: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Task {
    pub pkey: Option<u64>,
    pub fkey_req: u64,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductTenant {
    pub pkey: Option<u64>,
    pub fkey_tnet: u64,
    pub fkey_prod_ver: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Worker {
    pub pkey: Option<u64>,
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
    let account = mk_acct();
    let product = mk_prod();
    let product_ver = mk_prod_ver();
    let service = mk_svc();
    let service_ver = mk_svc_ver();
    let product_service = mk_prod_svc();
    let request = mk_req();
    let task = mk_task();
    let tenant = mk_tent();
    let product_tenant = mk_prod_tent();
    let worker = mk_worker();

    let account_ds: DS<AccountDO> = DS::from(&account);
    let service_ds: DS<Service> = DS::from(&service);
    let service_ver_ds: DS<ServiceVer> = DS::from(&service_ver);
    let product_ds: DS<Product> = DS::from(&product);
    let product_ver_ds: DS<ProductVer> = DS::from(&product_ver);
    let product_service_ds: DS<ProductService> = DS::from(&product_service);
    let request_ds: DS<Request> = DS::from(&request);
    let task_ds: DS<Task> = DS::from(&task);
    let tenant_ds: DS<Tenant> = DS::from(&tenant);
    let product_tenant_ds: DS<ProductTenant> = DS::from(&product_tenant);
    let worker_ds: DS<Worker> = DS::from(&worker);

    let def = Arc::new(SchemaDef {
        users: Box::new(HashMap::from([(
            String::from("app"),
            DBUser {
                role_id: String::from("app"),
                grants: Box::new(Vec::from([GrantInfo::All])),
            },
        )])),

        tables: Box::new(HashMap::from([
            (String::from("account"), account),
            (String::from("service"), service),
            (String::from("service_ver"), service_ver),
            (String::from("product"), product),
            (String::from("product_ver"), product_ver),
            (String::from("product_service"), product_service),
            (String::from("request"), request),
            (String::from("task"), task),
            (String::from("tenant"), tenant),
            (String::from("product_tenant"), product_tenant),
            (String::from("worker"), worker),
        ])),
    });

    DataSources {
        schema_def: def,
        account: account_ds,
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
    pub account: DS<AccountDO>,
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
