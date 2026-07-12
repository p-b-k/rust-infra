////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Basic DB Config info
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DBConfig {
    pub name: String,
    pub user: String,
    pub pass: String,
    pub host: String,
    pub port: u32,
}

impl DBConfig {
    pub fn to_url(&self) -> String {
        let name = &self.name;
        let user = &self.user;
        let pass = &self.pass;
        let host = &self.host;
        let port = self.port;

        format!("mysql://{user}:{pass}@{host}:{port}/{name}")
    }
}
