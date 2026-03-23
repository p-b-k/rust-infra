////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Template Based Cache
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::rescache::{CacheEntry, CacheLogic, CacheState, ResCache};

use log::warn;

pub struct Page {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub help: String,
}

pub type PageCacheEntry = CacheEntry<Page>;

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
    pub parts: Vec<Part>,
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

pub type PageCache = ResCache<PageCacheState, PageData, PageCacheLogic>;
