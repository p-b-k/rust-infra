////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Application state
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use cplane::app::PtConfig;
use std::clone::Clone;
use std::sync::Mutex;
use ui::filecache::FileCache;

use ui::{
    filecache::{FileCacheLogic, FileCacheState},
    rescache::ResCache,
};

#[derive(Clone)]
pub struct AppConfig {
    pub port: u32,
    pub login_page: String,
    pub pt: PtConfig,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            port: 7021,
            login_page: String::from("res/html/login.html"),
            pt: PtConfig::default(),
        }
    }
}

pub struct AppState {
    pub html_cache: Mutex<ResCache<FileCacheState, String, FileCacheLogic>>,
    pub json_cache: Mutex<ResCache<FileCacheState, String, FileCacheLogic>>,
    pub css_cache: Mutex<ResCache<FileCacheState, String, FileCacheLogic>>,
    pub js_cache: Mutex<ResCache<FileCacheState, String, FileCacheLogic>>,
    pub svg_cache: Mutex<ResCache<FileCacheState, String, FileCacheLogic>>,

    pub config: AppConfig,
}

pub fn create_app_state(config: AppConfig) -> AppState {
    let html_cache = FileCache::from_mime_and_root(mime::TEXT_HTML, "res/html");
    let json_cache = FileCache::from_mime_and_root(mime::APPLICATION_JSON, "res/json");
    let css_cache = FileCache::from_mime_and_root(mime::TEXT_CSS, "res/css");
    let js_cache = FileCache::from_mime_and_root(mime::APPLICATION_JAVASCRIPT, "res/js");
    let svg_cache = FileCache::from_mime_and_root(mime::IMAGE_SVG, "res/svg");

    AppState {
        html_cache: Mutex::new(html_cache),
        json_cache: Mutex::new(json_cache),
        css_cache: Mutex::new(css_cache),
        js_cache: Mutex::new(js_cache),
        svg_cache: Mutex::new(svg_cache),
        config,
    }
}
