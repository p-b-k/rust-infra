////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Application state
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use cplane::app::{DbConfig, PtConfig};
use mysql::{Opts, Pool};
use std::clone::Clone;
use std::sync::Mutex;
use threadpool::ThreadPool;

// use log::debug;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AppConfig {
    pub port: u32,
    pub th_pool_size: usize,
    pub db_pool_size: usize,
    pub pt: PtConfig,
    pub db: DbConfig,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            port: 7021,
            db: DbConfig {
                name: String::from("cplane"),
                user: String::from("cplane_app"),
                pass: String::from("secret"),
                host: String::from("localhost"),
                port: 3306,
            },
            pt: PtConfig::default(),
            th_pool_size: 8,
            db_pool_size: 4,
        }
    }
}

pub struct AppState {
    pub db_pool: Mutex<Option<Pool>>,
    pub config: AppConfig,
    pub th_pool: ThreadPool,
}

impl AppState {}

pub fn create_app_state(db_url: &String, config: AppConfig) -> AppState {
    let opts = Opts::from_url(db_url).unwrap();
    let db_pool = Pool::new(opts).unwrap();

    AppState {
        db_pool: Mutex::new(Some(db_pool)),
        config: config.clone(),
        th_pool: ThreadPool::new(config.th_pool_size),
    }
}
