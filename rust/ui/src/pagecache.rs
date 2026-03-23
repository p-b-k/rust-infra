////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Template Based Cache
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::rescache::{CacheLogic, CacheState, ResCache};

use std::{collections::HashMap, time::SystemTime};

use mime::Mime;

use log::warn;

pub struct Page {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub help: String,
}

pub struct PageCacheEntry {
    pub page: Page,
    pub page_path: String,
    pub html_path: String,
    pub page_ts: SystemTime,
    pub html_ts: SystemTime,
}

pub enum PageField {
    Title,
    Icon,
    Help,
    Desc,
}

pub enum Part {
    Text(String),
    Field(PageField),
}

pub struct PageData {
    pub title: String,
    pub icon: String,
    pub help: String,
    pub desc: String,
}

pub struct PageCacheState {
    pub page_root: String,
    pub html_template: String,
    pub parts: Vec<Part>,
    pub timestamp: SystemTime,
}

impl CacheState for PageCacheState {
    fn needs_sync(&self) -> bool {
        warn!(target: "PageCacheState", "{} not implemented", "needs_sync");
        false
    }
    fn sync(&mut self) -> Option<String> {
        warn!(target: "PageCacheState", "{} not implemented", "sync");
        None
    }
}

pub struct PageCacheLogic {}

fn read_page_from_file(_file: &str) -> Result<Page, String> {
    Err("Not Implemented".to_string())
}

impl CacheLogic<PageCacheState, PageCacheEntry> for PageCacheLogic {
    fn needs_sync(_state: &PageCacheState, _entry: &PageCacheEntry, _cache_key: &str) -> bool {
        warn!(target: "PageCacheLogic", "{} not implemented", "needs_sync");
        false
    }
    fn sync(
        _state: &PageCacheState,
        _entry: &mut PageCacheEntry,
        _cache_key: &str,
    ) -> Option<String> {
        warn!(target: "PageCacheLogic", "{} not implemented", "sync");
        None
    }
    fn find_resource(_state: &PageCacheState, _cache_key: &str) -> Option<PageCacheEntry> {
        warn!(target: "PageCacheLogic", "{} not implemented", "find_resource");
        None
    }
    fn mime_type(_state: &PageCacheState, _cache_key: &str) -> Mime {
        mime::TEXT_HTML
    }
    fn generate_content(
        _state: &PageCacheState,
        _entry: &PageCacheEntry,
    ) -> Result<String, (u32, String)> {
        warn!(target: "PageCacheLogic", "{} not implemented", "generate_contents");
        Err((
            500,
            format!("PageCacheLogic::generate_contents is not implemented yet"),
        ))
    }
}

pub type PageCache = ResCache<PageCacheState, PageCacheEntry, PageCacheLogic>;

impl PageCache {
    pub fn from_root_and_file(root: &str, file: &str) -> PageCache {
        PageCache {
            phantom: std::marker::PhantomData,
            state: PageCacheState {
                page_root: root.to_string(),
                html_template: file.to_string(),
                parts: Vec::new(),
                timestamp: SystemTime::now(),
            },
            map: HashMap::new(),
        }
    }
}
