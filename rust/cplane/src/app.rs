////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Support for multiple servers interacting
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub const DEFAULT_CP_PORT: u32 = 7020;
const DEFAULT_CP_TOKEN: &str = "this is the default token";

#[derive(Clone)]
pub struct PtConfig {
    pub host: String,
    pub port: u32,
    pub token: String,
}

impl PtConfig {
    pub fn default() -> PtConfig {
        PtConfig {
            host: String::from("localhost"),
            port: DEFAULT_CP_PORT,
            token: String::from(DEFAULT_CP_TOKEN),
        }
    }
}
