////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Define the data objects for the application
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::datasource::DS;
use serde::{Deserialize, Serialize};

//
// Service -------------------------------------------------
//

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Service {
    pub pkey: u64,
    pub svc_id: String,
    pub svc_name: String,
}

pub fn service() -> DS<(u64, String, String), Service> {
    DS {
        table: String::from("service"),
        fields: String::from("svc_id, svc_name"),
        cons: |(pkey, svc_id, svc_name)| Service {
            pkey,
            svc_id,
            svc_name,
        },
    }
}

//
// Service Ver ---------------------------------------------
//

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ServiceVer {
    pub pkey: u64,
    pub fkey_svc: u64,
}

pub fn service_ver() -> DS<(u64, u64), ServiceVer> {
    DS {
        table: String::from("service_ver"),
        fields: String::from("fkey_svc"),
        cons: |(pkey, fkey_svc)| ServiceVer { pkey, fkey_svc },
    }
}

//
// Product -------------------------------------------------
//

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Product {
    pub pkey: u64,
    pub prod_id: String,
    pub prod_name: String,
}

pub fn product() -> DS<(u64, String, String), Product> {
    DS {
        table: String::from("product"),
        fields: String::from("prod_id, prod_name"),
        cons: |(pkey, prod_id, prod_name)| Product {
            pkey,
            prod_id,
            prod_name,
        },
    }
}

//
// Product Ver ---------------------------------------------
//

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProductVer {
    pub pkey: u64,
    pub fkey_prod: u64,
}

pub fn product_ver() -> DS<(u64, u64), ProductVer> {
    DS {
        table: String::from("product_ver"),
        fields: String::from("fkey_prod"),
        cons: |(pkey, fkey_prod)| ProductVer { pkey, fkey_prod },
    }
}
