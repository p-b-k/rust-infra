////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Application state
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use cplane::app::{DEFAULT_CP_PORT, DbConfig, PtConfig};
use mysql::{Opts, Pool};
use std::clone::Clone;
use std::sync::Mutex;

use log::debug;

#[derive(Clone)]
pub struct AppConfig {
    pub port: u32,
    pub login_page: String,
    pub db: DbConfig,
    pub pt: PtConfig,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            port: DEFAULT_CP_PORT,
            login_page: String::from("res/html/login.html"),
            db: DbConfig {
                name: String::from("cplane"),
                user: String::from("cplane_app"),
                pass: String::from("secret"),
                host: String::from("localhost"),
                port: 3306,
            },
            pt: PtConfig::default(),
        }
    }
}

pub struct AppState {
    pub pool: Mutex<Option<Pool>>,
    pub config: AppConfig,
}

impl AppState {
    pub async fn set_connection_pool(&mut self, url: &String) {
        debug!("set_connection_pool called");
        let opts = Opts::from_url(url).unwrap();
        let new_pool = Pool::new(opts).unwrap();
        let mut pool = self.pool.lock().unwrap();
        let _old_val = pool.insert(new_pool);
        // TODO? Release _old_val?
    }
}

pub fn create_app_state(db_url: &String, config: AppConfig) -> AppState {
    let opts = Opts::from_url(db_url).unwrap();
    let conn_pool = Pool::new(opts).unwrap();

    AppState {
        pool: Mutex::new(Some(conn_pool)),
        config,
    }
}
