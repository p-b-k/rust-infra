////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Template Based Cache
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::rescache::{CacheLogic, CacheState, ResCache};

use std::{
    collections::HashMap,
    fs::{exists, metadata, read_to_string},
    time::SystemTime,
};

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
    pub html: String,
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

fn read_html_from_file(_file: &str) -> Result<Vec<Part>, String> {
    Err("Not Implemented".to_string())
}

impl CacheLogic<PageCacheState, PageCacheEntry> for PageCacheLogic {
    fn needs_sync(_state: &PageCacheState, entry: &PageCacheEntry, _cache_key: &str) -> bool {
        if !exists(&entry.page_path).unwrap() {
            warn!(target: "PageCacheLogic", "needs_sync: page file does not exist ({})", entry.page_path);
            return true;
        }
        if !exists(&entry.html_path).unwrap() {
            warn!(target: "PageCacheLogic", "needs_sync: html file does not exist ({})", entry.page_path);
            return true;
        }

        if metadata(&entry.page_path).unwrap().modified().unwrap() > entry.page_ts {
            return true;
        }

        if metadata(&entry.html_path).unwrap().modified().unwrap() > entry.html_ts {
            return true;
        }

        false
    }

    fn sync(
        _state: &PageCacheState,
        entry: &mut PageCacheEntry,
        _cache_key: &str,
    ) -> Option<String> {
        if !exists(&entry.page_path).unwrap() {
            warn!(target: "PageCacheLogic", "sync: page file does not exist ({})", entry.page_path);
            return Some(format!(
                "sync: Page file ({}) does not exist",
                &entry.page_path
            ));
        }
        if !exists(&entry.html_path).unwrap() {
            warn!(target: "PageCacheLogic", "sync: html file does not exist ({})", entry.page_path);
            return Some(format!(
                "sync: HTML file ({}) does not exist",
                &entry.html_path
            ));
        }

        if metadata(&entry.page_path).unwrap().modified().unwrap() > entry.page_ts {
            match read_page_from_file(&entry.page_path) {
                Ok(p) => {
                    entry.page = p;
                }
                Err(s) => return Some(s),
            }
        }

        if metadata(&entry.html_path).unwrap().modified().unwrap() > entry.html_ts {
            match read_to_string(&entry.html_path) {
                Ok(s) => {
                    entry.html = s;
                }
                Err(s) => return Some(format!("{}", s.to_string())),
            }
        }

        None
    }

    fn find_resource(state: &PageCacheState, cache_key: &str) -> Option<PageCacheEntry> {
        let page_path = format!("{}/{cache_key}.toml", state.page_root);
        let html_path = format!("{}/{cache_key}.html", state.page_root);

        if exists(page_path.as_str()).unwrap() {
            if exists(html_path.as_str()).unwrap() {
                None
            } else {
                match read_page_from_file(page_path.as_str()) {
                    Ok(p) => match read_to_string(page_path.as_str()) {
                        Ok(s) => {
                            let html_ts = metadata(html_path.as_str()).unwrap().modified().unwrap();
                            let page_ts = metadata(page_path.as_str()).unwrap().modified().unwrap();
                            Some(PageCacheEntry {
                                page: p,
                                html: s,
                                page_path,
                                html_path,
                                page_ts,
                                html_ts,
                            })
                        }
                        Err(e) => {
                            warn!(target:"PageCacheLogic", "find_resource: Unable to read html file for {cache_key}");
                            warn!(target:"PageCacheLogic", "find_resource: {}", e.to_string());
                            None
                        }
                    },
                    Err(e) => {
                        warn!(target:"PageCacheLogic", "find_resource: Unable to read page file for {cache_key}");
                        warn!(target:"PageCacheLogic", "find_resource: {}", e.to_string());
                        None
                    }
                }
            }
        } else {
            warn!(target:"PageCacheLogic", "find_resource: page file not found ({page_path})");
            None
        }
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
