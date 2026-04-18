////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Try putting struct in subdir
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum RequestMsgType {
    Log,
    Warn,
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct RequestMsg {
    typ: RequestMsgType,
    msg: String,
}

pub struct RequestError {
    msg: String,
}

#[derive(Serialize, Deserialize)]
pub enum RequestStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Deserialize, Serialize)]
pub struct RequestRO {
    req_type: String,
    req_scope: String,
    req_status: RequestStatus,
    req_msgs: u32,
    req_warns: u32,
    req_errors: u32,
}

impl RequestError {
    pub fn new(msg: &str) -> RequestError {
        RequestError {
            msg: msg.to_string(),
        }
    }

    pub fn as_str(&self) -> String {
        self.msg.clone()
    }
}

impl RequestMsg {
    pub fn new(typ: RequestMsgType, msg: String) -> RequestMsg {
        RequestMsg { msg, typ }
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }

    pub fn typ(&self) -> RequestMsgType {
        self.typ
    }
}
