////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Request Objects for Services section data
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

use infra::common::CPlaneError;
use infra::version::Version;
use mysql::PooledConn;
use serde::{Deserialize, Serialize};

use log::error;

use crate::tabs::service::SERVICE_FACTORY;
use crate::tabs::service_ver::{
    SERVICE_VER_FACTORY,
    fields::{FKEY_SVC, SVC_VER},
};

const NO_VERSION: &str = "\u{26D4}";

#[derive(Serialize, Deserialize, Clone)]
pub struct ServiceMainRO {
    pub pkey: u64,
    pub svc_id: String,
    pub svc_name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ServiceDetailRO {
    pub pkey: u64,
    pub svc_id: String,
    pub svc_name: String,
    pub version: String,
}

pub fn get_main_services(conn: &mut PooledConn) -> Result<Vec<ServiceMainRO>, CPlaneError> {
    let mut vmap: HashMap<u64, Version> = HashMap::new();

    match SERVICE_VER_FACTORY.max(conn, &SVC_VER, &FKEY_SVC) {
        Ok(versions) => {
            vmap = versions;
        }
        Err(e) => {
            error!("Unable to retrieve service versions: {}", e.to_string());
        }
    }

    match SERVICE_FACTORY.all(conn) {
        Ok(services) => {
            let mut result: Vec<ServiceMainRO> = Vec::new();

            for svc in services {
                result.push(ServiceMainRO {
                    pkey: svc.pkey.unwrap(),
                    svc_id: svc.obj.svc_id,
                    svc_name: svc.obj.svc_name,
                    version: match vmap.get(&svc.pkey.unwrap()) {
                        Some(v) => v.to_short_string(),
                        None => NO_VERSION.to_string(),
                    },
                });
            }

            Ok(result)
        }
        Err(e) => CPlaneError::new(e.to_string().as_str()),
    }
}

pub fn get_service_detail(
    conn: &mut PooledConn,
    pkey: u64,
) -> Result<Option<ServiceDetailRO>, CPlaneError> {
    let mut vmap: HashMap<u64, String> = HashMap::new();

    // TODO: Fix up query mechanism to handle a single max value
    match SERVICE_VER_FACTORY.max(conn, &SVC_VER, &FKEY_SVC) {
        Ok(versions) => {
            vmap = versions;
        }
        Err(e) => {
            error!("Unable to retrieve service versions: {}", e.to_string());
        }
    }

    let max_ver = vmap.get(&pkey);

    match SERVICE_FACTORY.fetch(conn, pkey) {
        Ok(s) => Ok(Some(ServiceDetailRO {
            pkey,
            svc_id: s.obj.svc_id,
            svc_name: s.obj.svc_name,
            version: match max_ver {
                Some(v) => {
                    let ver = Version::from_string(v).expect("Unable to parse version string");
                    ver.to_short_string()
                }
                None => NO_VERSION.to_string(),
            },
        })),
        Err(e) => CPlaneError::new(e.to_string().as_str()),
    }
}
