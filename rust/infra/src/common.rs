////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Basic, everyday objects
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

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
