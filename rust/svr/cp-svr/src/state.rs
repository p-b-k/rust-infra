////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Application state
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use cplane::app::{DEFAULT_CP_PORT, DbConfig, PtConfig};
use mysql::{Opts, Pool};
use std::clone::Clone;
use std::sync::Mutex;
use ui::filecache::FileCache;

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
    pub html_cache: Mutex<FileCache>,
    pub json_cache: Mutex<FileCache>,
    pub css_cache: Mutex<FileCache>,
    pub js_cache: Mutex<FileCache>,
    pub svg_cache: Mutex<FileCache>,

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
    let html_cache = FileCache::from_mime_and_root(mime::TEXT_HTML, "res/html");
    let json_cache = FileCache::from_mime_and_root(mime::APPLICATION_JSON, "res/json");
    let css_cache = FileCache::from_mime_and_root(mime::TEXT_CSS, "res/css");
    let js_cache = FileCache::from_mime_and_root(mime::APPLICATION_JAVASCRIPT, "res/js");
    let svg_cache = FileCache::from_mime_and_root(mime::IMAGE_SVG, "res/svg");

    let opts = Opts::from_url(db_url).unwrap();
    let conn_pool = Pool::new(opts).unwrap();

    AppState {
        html_cache: Mutex::new(html_cache),
        json_cache: Mutex::new(json_cache),
        css_cache: Mutex::new(css_cache),
        js_cache: Mutex::new(js_cache),
        svg_cache: Mutex::new(svg_cache),
        pool: Mutex::new(Some(conn_pool)),
        config,
    }
}
