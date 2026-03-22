////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Application state
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use cplane::app::PtConfig;
use std::collections::HashMap;
use std::sync::Mutex;
use std::{clone::Clone, marker::PhantomData};
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
    let html_cache = FileCache {
        phantom: PhantomData {},
        state: FileCacheState {
            mime: mime::TEXT_HTML,
            root: "res/html".to_string(),
        },
        map: HashMap::new(),
    };

    let json_cache = FileCache {
        phantom: PhantomData {},
        state: FileCacheState {
            mime: mime::APPLICATION_JSON,
            root: "res/json".to_string(),
        },
        map: HashMap::new(),
    };

    let css_cache = FileCache {
        phantom: PhantomData {},
        state: FileCacheState {
            mime: mime::TEXT_CSS,
            root: "res/css".to_string(),
        },
        map: HashMap::new(),
    };

    let js_cache = FileCache {
        phantom: PhantomData {},
        state: FileCacheState {
            mime: mime::APPLICATION_JAVASCRIPT,
            root: "res/js".to_string(),
        },
        map: HashMap::new(),
    };

    let svg_cache = FileCache {
        phantom: PhantomData {},
        state: FileCacheState {
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
