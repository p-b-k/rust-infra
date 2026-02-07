////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Support for multiple servers interacting
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub const DEFAULT_CP_PORT: u32 = 7020;
const DEFAULT_CP_TOKEN: &str = "this is the default token";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PtConfig {
    pub host: String,
    pub port: u32,
    pub token: String,
    pub proto: String,
    pub root: String,
}

impl PtConfig {
    pub fn default() -> PtConfig {
        PtConfig {
            host: String::from("localhost"),
            port: DEFAULT_CP_PORT,
            token: String::from(DEFAULT_CP_TOKEN),
            proto: String::from("http"),
            root: String::from("json"),
        }
    }

    pub fn get_passthrough_url(&self, from_path: &str) -> String {
        let proto = &self.proto;
        let host = &self.host;
        let port = self.port;
        let root = &self.root;

        format!("{proto}://{host}:{port}/{root}/{from_path}")
    }
}

// DB Config

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DbConfig {
    pub name: String,
    pub user: String,
    pub pass: String,
    pub host: String,
    pub port: u32,
}

impl DbConfig {
    pub fn to_url(&self) -> String {
        let name = &self.name;
        let user = &self.user;
        let pass = &self.pass;
        let host = &self.host;
        let port = self.port;

        format!("mysql://{user}:{pass}@{host}:{port}/{name}")
    }
}
