////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Application state
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use cplane::app::PtConfig;
use std::collections::HashMap;
use std::sync::Mutex;
use std::{clone::Clone, marker::PhantomData};

use ui::{
    filecache::{FileCache, RFileCache, RFileCacheLogic, RFileCacheState, StaticFileCacheLogic},
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
    pub html_cache: Mutex<FileCache<StaticFileCacheLogic>>,
    pub json_cache: Mutex<FileCache<StaticFileCacheLogic>>,
    pub css_cache: Mutex<FileCache<StaticFileCacheLogic>>,
    pub js_cache: Mutex<FileCache<StaticFileCacheLogic>>,
    // pub svg_cache: Mutex<FileCache<StaticFileCacheLogic>>,
    pub svg_cache: Mutex<ResCache<RFileCacheState, String, RFileCacheLogic>>,

    pub config: AppConfig,
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

    // let svg_cache = FileCache::new(
    //     StaticFileCacheLogic {},
    //     String::from("res/svg"),
    //     mime::IMAGE_SVG,
    // );
    let svg_cache = RFileCache {
        phantom: PhantomData {},
        state: RFileCacheState {
            mime: mime::IMAGE_SVG,
            root: "res/svg".to_string(),
        },
        map: HashMap::new(),
    };

    AppState {
        html_cache: Mutex::new(html_cache),
        json_cache: Mutex::new(json_cache),
        css_cache: Mutex::new(css_cache),
        js_cache: Mutex::new(js_cache),
        svg_cache: Mutex::new(svg_cache),
        config,
    }
}
