////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Application state
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use cplane::app::PtConfig;
use std::clone::Clone;
use std::sync::Mutex;
use ui::filecache::FileCache;
use ui::pagecache::PageCache;

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
    pub html_cache: Mutex<FileCache>,
    pub json_cache: Mutex<FileCache>,
    pub css_cache: Mutex<FileCache>,
    pub js_cache: Mutex<FileCache>,
    pub svg_cache: Mutex<FileCache>,

    pub page_cache: Mutex<PageCache>,

    pub config: AppConfig,
}

pub fn create_app_state(config: AppConfig) -> AppState {
    let html_cache = FileCache::from_mime_and_root(mime::TEXT_HTML, "res/html");
    let json_cache = FileCache::from_mime_and_root(mime::APPLICATION_JSON, "res/json");
    let css_cache = FileCache::from_mime_and_root(mime::TEXT_CSS, "res/css");
    let js_cache = FileCache::from_mime_and_root(mime::APPLICATION_JAVASCRIPT, "res/js");
    let svg_cache = FileCache::from_mime_and_root(mime::IMAGE_SVG, "res/svg");
    let page_cache = PageCache::from_root_and_file("res/pages", "res/templates/main.html").unwrap();

    AppState {
        html_cache: Mutex::new(html_cache),
        json_cache: Mutex::new(json_cache),
        css_cache: Mutex::new(css_cache),
        js_cache: Mutex::new(js_cache),
        svg_cache: Mutex::new(svg_cache),
        page_cache: Mutex::new(page_cache),

        config,
    }
}
