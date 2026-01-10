////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Application state
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use mysql::{Opts, Pool};
use std::clone::Clone;
use std::sync::Mutex;

use crate::filecache::{FileCache, StaticFileCacheLogic};

#[derive(Clone)]
pub struct AppConfig {
    pub port: u32,
    pub login_page: String,
}

pub struct AppState {
    pub html_cache: Mutex<FileCache<StaticFileCacheLogic>>,
    pub json_cache: Mutex<FileCache<StaticFileCacheLogic>>,
    pub css_cache: Mutex<FileCache<StaticFileCacheLogic>>,
    pub js_cache: Mutex<FileCache<StaticFileCacheLogic>>,
    pub svg_cache: Mutex<FileCache<StaticFileCacheLogic>>,

    pub pool: Mutex<Option<Pool>>,
    pub config: AppConfig,
}

impl AppState {
    pub async fn set_connection_pool(&mut self, url: &String) {
        let opts = Opts::from_url(url).unwrap();
        let new_pool = Pool::new(opts).unwrap();
        let mut pool = self.pool.lock().unwrap();
        let _old_val = pool.insert(new_pool);
        // TODO? Release _old_val?
    }
}

pub fn create_app_state(config: AppConfig) -> AppState {
    let html_cache = FileCache::new(
        StaticFileCacheLogic {},
        String::from("res/html"),
        mime::TEXT_HTML,
    );

    let json_cache = FileCache::new(
        StaticFileCacheLogic {},
        String::from("res/json"),
        mime::APPLICATION_JSON,
    );

    let css_cache = FileCache::new(
        StaticFileCacheLogic {},
        String::from("res/css"),
        mime::TEXT_CSS,
    );

    let js_cache = FileCache::new(
        StaticFileCacheLogic {},
        String::from("res/js"),
        mime::APPLICATION_JAVASCRIPT,
    );

    let svg_cache = FileCache::new(
        StaticFileCacheLogic {},
        String::from("res/svg"),
        mime::IMAGE_SVG,
    );

    AppState {
        html_cache: Mutex::new(html_cache),
        json_cache: Mutex::new(json_cache),
        css_cache: Mutex::new(css_cache),
        js_cache: Mutex::new(js_cache),
        svg_cache: Mutex::new(svg_cache),
        pool: Mutex::new(None),
        config,
    }
}
