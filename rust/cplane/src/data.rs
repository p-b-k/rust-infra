////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Define the data objects for the application
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::datasource::DS;
use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};

//
// Service -------------------------------------------------
//

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Service {
    pub pkey: u64,
    pub svc_id: String,
    pub svc_name: String,
}

pub fn service() -> DS<Service> {
    DS::new("service", "svc_id, svc_name")
}

//
// Service Ver ---------------------------------------------
//

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ServiceVer {
    pub pkey: u64,
    pub fkey_svc: u64,
}

pub fn service_ver() -> DS<ServiceVer> {
    DS::new("service_ver", "fkey_svc")
}

//
// Product -------------------------------------------------
//

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Product {
    pub pkey: u64,
    pub prod_id: String,
    pub prod_name: String,
}

pub fn product() -> DS<Product> {
    DS::new("product", "prod_id, prod_name")
}

//
// Product Ver ---------------------------------------------
//

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductVer {
    pub pkey: u64,
    pub fkey_prod: u64,
}

pub fn product_ver() -> DS<ProductVer> {
    DS::new("product_ver", "fkey_prod")
}
