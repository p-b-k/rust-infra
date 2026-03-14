////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Define the Control Plane schema
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

use infra::data_object::{DObj, DObjFactory};
use infra::schema::{DBUser, GrantInfo, SchemaDef, TableDef};

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
        fields.push_str(field.name);
    }

    fields
}

// ---------------------------------------------------------------------------------------------------------------------
// Define the tables
// ---------------------------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Customer {
    // pub pkey: u64,
    pub cust_id: String,
    pub cust_name: String,
}

pub type CustomerDO<'a> = DObj<'a, Customer>;
pub static CUSTOMER_FACTORY: DObjFactory<'static, Customer> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &CUSTOMER,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Product {
    // pub pkey: Option<u64>,
    pub prod_id: String,
    pub prod_name: String,
}
pub type ProductDO<'a> = DObj<'a, Product>;
pub static PRODUCT_FACTORY: DObjFactory<'static, Product> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &PRODUCT,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductVer {
    // pub pkey: Option<u64>,
    pub fkey_prod: u64,
    pub maj_ver: u32,
    pub min_ver: u32,
    pub rel_ver: Option<u32>,
    pub bld_ver: Option<u32>,
    pub bld_tag: Option<String>,
}
pub type ProductVerDO<'a> = DObj<'a, ProductVer>;
pub static PRODUCT_VER_FACTORY: DObjFactory<'static, ProductVer> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &PRODUCT_VERSION,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Service {
    // pub pkey: Option<u64>,
    pub svc_id: String,
    pub svc_name: String,
    pub is_global: String,
}
pub type ServiceDO<'a> = DObj<'a, Service>;
pub static SERVICE_FACTORY: DObjFactory<'static, Service> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &SERVICE,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ServiceVer {
    // pub pkey: Option<u64>,
    pub fkey_svc: String,
    pub maj_ver: u32,
    pub min_ver: u32,
    pub rel_ver: Option<u32>,
    pub bld_ver: Option<u32>,
    pub bld_tag: Option<String>,
    pub schema_def: Option<String>,
}
pub type ServiceVerDO<'a> = DObj<'a, ServiceVer>;
pub static SERVICE_VER_FACTORY: DObjFactory<'static, ServiceVer> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &SERVICE_VERSION,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductService {
    // pub pkey: Option<u64>,
    pub fkey_prod: u64,
    pub fkey_svc: u64,
}
pub type ProductServiceDO<'a> = DObj<'a, ProductService>;
pub static PRODUCT_SERVICE_FACTORY: DObjFactory<'static, ProductService> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &PRODUCT_SERVICE,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Request {
    // pub pkey: Option<u64>,
    pub req_type: String,
    pub req_start: u64,
    pub req_status: String,
}
pub type RequestDO<'a> = DObj<'a, Request>;
pub static REQUEST_FACTORY: DObjFactory<'static, Request> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &REQUEST,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Tenant {
    // pub pkey: Option<u64>,
    pub fkey_acct: u64,
}
pub type TenantDO<'a> = DObj<'a, Tenant>;
pub static TENANT_FACTORY: DObjFactory<'static, Tenant> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &TENANT,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Task {
    // pub pkey: Option<u64>,
    pub fkey_req: u64,
    pub status: String,
}
pub type TaskDO<'a> = DObj<'a, Task>;
pub static TASK_FACTORY: DObjFactory<'static, Task> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &TASK,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductTenant {
    // pub pkey: Option<u64>,
    pub fkey_tnet: u64,
    pub fkey_prod_ver: u64,
}
pub type ProductTenantDO<'a> = DObj<'a, ProductTenant>;
pub static PRODUCT_TENANT_FACTORY: DObjFactory<'static, ProductTenant> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &PRODUCT_TENANT,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Worker {
    // pub pkey: Option<u64>,
    pub name: String,
    pub host: String,
    pub port: u32,
    pub status: u32,
    // pub last_check: Time,
}
pub type WorkerDO<'a> = DObj<'a, Worker>;
pub static WOKER_FACTORY: DObjFactory<'static, Worker> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &WORKER,
};

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
