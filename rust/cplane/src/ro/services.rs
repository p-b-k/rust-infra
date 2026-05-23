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

pub struct ServicesVersionsRO {
    pub fkey_svc: u64,
    pub svc_ver: Version,
}

pub fn get_main_services(conn: &mut PooledConn) -> Result<Vec<ServiceMainRO>, CPlaneError> {
    let mut vmap: HashMap<u64, String> = HashMap::new();

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
                        Some(v) => Version::from_string(v)
                            .expect("Unable to parse version string")
                            .to_short_string(),

                        None => NO_VERSION.to_string(),
                    },
                });
            }

            Ok(result)
        }
        Err(e) => CPlaneError::new(e.to_string().as_str()),
    }
}
