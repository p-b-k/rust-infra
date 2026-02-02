////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Define the data objects for the application
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::datasource::DS;
use serde::{Deserialize, Serialize};
use mysql::prelude::FromRow;

//
// Service -------------------------------------------------
//

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Service {
    pub pkey: u64,
    pub svc_id: String,
    pub svc_name: String,
}

pub fn service() -> DS {
    DS {
        table: String::from("service"),
        fields: String::from("svc_id, svc_name"),
    }
}

//
// Service Ver ---------------------------------------------
//

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ServiceVer {
    pub pkey: u64,
    pub fkey_svc: u64,
}

pub fn service_ver() -> DS {
    DS {
        table: String::from("service_ver"),
        fields: String::from("fkey_svc"),
    }
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

pub fn product() -> DS {
    DS {
        table: String::from("product"),
        fields: String::from("prod_id, prod_name"),
    }
}

//
// Product Ver ---------------------------------------------
//

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, FromRow)]
pub struct ProductVer {
    pub pkey: u64,
    pub fkey_prod: u64,
}

pub fn product_ver() -> DS {
    DS {
        table: String::from("product_ver"),
        fields: String::from("fkey_prod"),
    }
}
