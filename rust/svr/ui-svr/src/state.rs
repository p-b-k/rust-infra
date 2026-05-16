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
    pub cache_root: String,
    pub login_page: String,
    pub pt: PtConfig,
    pub dev_mode: bool,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            port: 7021,
            cache_root: String::from("."),
            login_page: String::from("res/html/login.html"),
            pt: PtConfig::default(),
            dev_mode: false,
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
    let res_root = format!("{}/res", config.cache_root);
    let html_cache = FileCache::from_mime_and_root(
        config.dev_mode,
        mime::TEXT_HTML,
        format!("{res_root}/html").as_str(),
    );
    let json_cache = FileCache::from_mime_and_root(
        config.dev_mode,
        mime::APPLICATION_JSON,
        format!("{res_root}/json").as_str(),
    );
    let css_cache = FileCache::from_mime_and_root(
        config.dev_mode,
        mime::TEXT_CSS,
        format!("{res_root}/css").as_str(),
    );
    let js_cache = FileCache::from_mime_and_root(
        config.dev_mode,
        mime::APPLICATION_JAVASCRIPT,
        format!("{res_root}/js").as_str(),
    );
    let svg_cache = FileCache::from_mime_and_root(
        config.dev_mode,
        mime::IMAGE_SVG,
        format!("{res_root}/svg").as_str(),
    );
    let mut page_cache = PageCache::from_root_and_file(
        config.dev_mode,
        format!("{res_root}/pages").as_str(),
        format!("{res_root}/templates/main.html").as_str(),
    )
    .unwrap();
    page_cache.initialize();

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
