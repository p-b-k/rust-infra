////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Define the Control Plane schema
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

use infra::data_object::{AsRecord, DObj, DObjFactory};
use infra::schema::{DBUser, GrantInfo, SchemaDef, TableDef};

use infra::sql::{SqlValue };
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductVer {
    pub fkey_prod: u64,
    pub maj_ver: u64,
    pub min_ver: u64,
    pub rel_ver: Option<u64>,
    pub bld_ver: Option<u64>,
    pub bld_tag: Option<String>,
}

impl<'a> AsRecord<'a> for ProductVer {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        let rel_ver = match self.rel_ver {
            Some(i) => SqlValue::Nullable(Some(Box::new(SqlValue::Id(i)))),
            None => SqlValue::Nullable(None)
        };

        let bld_ver = match self.bld_ver {
            Some(i) => SqlValue::Nullable(Some(Box::new(SqlValue::Id(i)))),
            None => SqlValue::Nullable(None)
        };

        let bld_tag = match self.bld_tag.as_ref() {
            Some(s) => SqlValue::Nullable(Some(Box::new(SqlValue::String(s.clone())))),
            None => SqlValue::Nullable(None)
        };

        Vec::from([
            ("fkey_prod", SqlValue::Id(self.fkey_prod)),
            ("maj_ver", SqlValue::Id(self.maj_ver)),
            ("min_ver", SqlValue::Id(self.maj_ver)),
            ("rel_ver", rel_ver),
            ("bld_ver", bld_ver),
            ("bld_tag", bld_tag),
        ])
    }
}

pub type ProductVerDO<'a> = DObj<'a, ProductVer>;
pub static PRODUCT_VER_FACTORY: DObjFactory<'static, ProductVer> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &PRODUCT_VERSION,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Service {
    pub svc_id: String,
    pub svc_name: String,
    pub is_global: String,
}

impl<'a> AsRecord<'a> for Service {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([
            ("svc_id", SqlValue::String(self.svc_id.clone())),
            ("svc_name", SqlValue::String(self.svc_name.clone())),
            ("is_global", SqlValue::String(self.is_global.clone())),
        ])
    }
}

pub type ServiceDO<'a> = DObj<'a, Service>;
pub static SERVICE_FACTORY: DObjFactory<'static, Service> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &SERVICE,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ServiceVer {
    pub fkey_svc: u64,
    pub maj_ver: u64,
    pub min_ver: u64,
    pub rel_ver: Option<u64>,
    pub bld_ver: Option<u64>,
    pub bld_tag: Option<String>,
    // pub schema_def: Option<String>,
}

impl<'a> AsRecord<'a> for ServiceVer {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        let rel_ver = match self.rel_ver {
            Some(i) => SqlValue::Nullable(Some(Box::new(SqlValue::Id(i)))),
            None => SqlValue::Nullable(None)
        };

        let bld_ver = match self.bld_ver {
            Some(i) => SqlValue::Nullable(Some(Box::new(SqlValue::Id(i)))),
            None => SqlValue::Nullable(None)
        };

        let bld_tag = match self.bld_tag.as_ref() {
            Some(s) => SqlValue::Nullable(Some(Box::new(SqlValue::String(s.clone())))),
            None => SqlValue::Nullable(None)
        };

        Vec::from([

            ("fkey_svc", SqlValue::Id(self.fkey_svc)),
            ("maj_ver", SqlValue::Id(self.maj_ver)),
            ("min_ver", SqlValue::Id(self.maj_ver)),
            ("rel_ver", rel_ver),
            ("bld_ver", bld_ver),
            ("bld_tag", bld_tag),


            
        ])
    }
}

pub type ServiceVerDO<'a> = DObj<'a, ServiceVer>;
pub static SERVICE_VER_FACTORY: DObjFactory<'static, ServiceVer> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &SERVICE_VERSION,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductService {
    pub fkey_prod: u64,
    pub fkey_svc: u64,
}

impl<'a> AsRecord<'a> for ProductService {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([

            ("fkey_prod", SqlValue::Id(self.fkey_prod)),
            ("fkey_svc", SqlValue::Id(self.fkey_svc)),

            
        ])
    }
}

pub type ProductServiceDO<'a> = DObj<'a, ProductService>;
pub static PRODUCT_SERVICE_FACTORY: DObjFactory<'static, ProductService> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &PRODUCT_SERVICE,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Request {
    pub req_type: String,
    pub req_start: u64,
    pub req_status: String,
}

impl<'a> AsRecord<'a> for Request {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([

            ("req_type", SqlValue::String(self.req_type.clone())),
            ("req_start", SqlValue::Id(self.req_start)),
            ("req_status", SqlValue::String(self.req_status.clone())),
            
        ])
    }
}

pub type RequestDO<'a> = DObj<'a, Request>;
pub static REQUEST_FACTORY: DObjFactory<'static, Request> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &REQUEST,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Tenant {
    pub fkey_acct: u64,
}

impl<'a> AsRecord<'a> for Tenant {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([
            ("fkey_acct", SqlValue::Id(self.fkey_acct)),
            
        ])
    }
}

pub type TenantDO<'a> = DObj<'a, Tenant>;
pub static TENANT_FACTORY: DObjFactory<'static, Tenant> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &TENANT,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Task {
    pub fkey_req: u64,
    pub status: String,
}

impl<'a> AsRecord<'a> for Task {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([

            ("fkey_req", SqlValue::Id(self.fkey_req)),
            ("status", SqlValue::String(self.status.clone())),

            
        ])
    }
}

pub type TaskDO<'a> = DObj<'a, Task>;
pub static TASK_FACTORY: DObjFactory<'static, Task> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &TASK,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductTenant {
    pub fkey_tnet: u64,
    pub fkey_prod_ver: u64,
}

impl<'a> AsRecord<'a> for ProductTenant {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([
            
            ("fkey_tnet", SqlValue::Id(self.fkey_tnet)),
            ("fkey_prod_ver", SqlValue::Id(self.fkey_prod_ver)),
        ])
    }
}

pub type ProductTenantDO<'a> = DObj<'a, ProductTenant>;
pub static PRODUCT_TENANT_FACTORY: DObjFactory<'static, ProductTenant> = DObjFactory {
    phantom: std::marker::PhantomData {},
    table: &PRODUCT_TENANT,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Worker {
    pub name: String,
    pub host: String,
    pub port: u32,
    pub status: String,
    // pub last_check: Time,
}

impl<'a> AsRecord<'a> for Worker {
    fn pairs(&self) -> Vec<(&str, SqlValue<'a>)> {
        Vec::from([


            ("name", SqlValue::String(self.name.clone())),
            ("host", SqlValue::String(self.host.clone())),
            ("port", SqlValue::ShortU(self.port)),
            ("status", SqlValue::String(self.status.clone())),

            
        ])
    }
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
