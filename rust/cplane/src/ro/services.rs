////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Request Objects for Services section data
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

use log::warn;

// TODO: Move this somewhere general
#[derive(Serialize, Deserialize, Clone)]
pub struct CPlaneError {
    pub msg: String,
}

impl CPlaneError {
    pub fn to_string(&self) -> String {
        self.msg.clone()
    }

    pub fn new<T>(msg: &str) -> Result<T, CPlaneError> {
        Err(CPlaneError {
            msg: msg.to_string(),
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ServiceMainRecord {
    pub svc_id: String,
    pub svc_name: String,
    pub version: String,
}

pub fn get_main_services() -> Result<Vec<ServiceMainRecord>, CPlaneError> {
    warn!("get_main_services not implemented, returning bogus data");
    Ok(Vec::from([
        ServiceMainRecord {
            svc_id: "svc1".to_string(),
            svc_name: "Some Service".to_string(),
            version: "1.0.2".to_string(),
        },
        ServiceMainRecord {
            svc_id: "svc2".to_string(),
            svc_name: "Some Other Service".to_string(),
            version: "0.0.2".to_string(),
        },
    ]))
}
