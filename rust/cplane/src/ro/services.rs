////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Request Objects for Services section data
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::common::CPlaneError;
use mysql::PooledConn;
use serde::{Deserialize, Serialize};

use log::warn;

use crate::tabs::service::SERVICE_FACTORY;

#[derive(Serialize, Deserialize, Clone)]
pub struct ServiceMainRecord {
    pub svc_id: String,
    pub svc_name: String,
    pub version: String,
}

pub fn get_main_services(conn: &mut PooledConn) -> Result<Vec<ServiceMainRecord>, CPlaneError> {
    warn!("get_main_services not implemented, returning bogus data");

    // let mut pool = state.pool.lock().unwrap();
    // let mut_pool = pool.as_mut();
    // let mut conn: PooledConn = mut_pool.unwrap().get_conn().unwrap();

    match SERVICE_FACTORY.all(conn) {
        Ok(services) => {
            let mut result: Vec<ServiceMainRecord> = Vec::new();

            for svc in services {
                result.push(ServiceMainRecord {
                    svc_id: svc.obj.svc_id,
                    svc_name: svc.obj.svc_name,
                    version: "".to_string(),
                })
            }

            Ok(result)
        }
        Err(e) => CPlaneError::new(e.to_string().as_str()),
    }
    // let services = SERVICE_FACTORY.all(conn).unwrap();

    // Ok(Vec::from([
    //     ServiceMainRecord {
    //         svc_id: "svc1".to_string(),
    //         svc_name: "Some Service".to_string(),
    //         version: "1.0.2".to_string(),
    //     },
    //     ServiceMainRecord {
    //         svc_id: "svc2".to_string(),
    //         svc_name: "Some Other Service".to_string(),
    //         version: "0.0.2".to_string(),
    //     },
    // ]))
}
