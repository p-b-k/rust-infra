////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Template Based Cache
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::rescache::{CacheLogic, CacheState, ResCache};

use std::{
    collections::HashMap,
    fs::{exists, metadata, read_to_string},
    time::SystemTime,
};

use serde::Deserialize;

use mime::Mime;

use log::{error, info, warn};

#[derive(Deserialize, Debug)]
pub struct Page {
    pub name: String,
    pub title: String,
    pub icon: String,
    pub desc: String,
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

#[derive(Debug)]
pub enum PageField {
    Title,
    Name,
    Icon,
    Help,
    Desc,
    Body,
}

impl PageCacheEntry {
    pub fn value_for(&self, f: &PageField) -> String {
        match f {
            PageField::Name => self.page.name.clone(),
            PageField::Title => self.page.title.clone(),
            PageField::Icon => self.page.icon.clone(),
            PageField::Help => self.page.help.clone(),
            PageField::Desc => self.page.desc.clone(),
            PageField::Body => self.html.clone(),
        }
    }
}

#[derive(Debug)]
pub enum Part {
    Text(String),
    Field(PageField),
}

impl Part {
    pub fn to_string(&self, entry: &PageCacheEntry) -> String {
        match self {
            Part::Text(s) => s.clone(),
            Part::Field(f) => entry.value_for(f),
        }
    }
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
        if !exists(&self.html_template).unwrap() {
            warn!(target: "PageCacheLogic", "needs_sync: page file does not exist ({})", self.html_template);
            return true;
        }

        let res = metadata(&self.html_template).unwrap().modified().unwrap() > self.timestamp;
        info!("does {} need sync? {res}", self.html_template);
        res
    }

    fn sync(&mut self) -> Option<String> {
        if exists(&self.html_template).unwrap() {
            match read_html_from_file(self.html_template.as_str()) {
                Ok(v) => {
                    self.parts = v;
                    self.timestamp = metadata(&self.html_template).unwrap().modified().unwrap();
                    None
                }
                Err(s) => Some(format!("Error parsing {:?}: {s}", &self.html_template)),
            }
        } else {
            Some(format!("The file {:?} does not exist", &self.html_template))
        }
    }
}

pub struct PageCacheLogic {}

#[derive(Debug)]
enum ParseState {
    Out,
    In,
    Bracket,
    Brace,
}

fn process_template(s: &str, v: &mut Vec<Part>) {
    let mut part = String::new();
    let mut state = ParseState::Out;

    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap();
        info!(target: "process_template", "State = {state:?}, c = {c:?}, i = {i}/{}", s.len());
        match state {
            ParseState::Out => {
                if c == '<' {
                    state = ParseState::Bracket;
                } else {
                    part.push(c);
                }
            }
            ParseState::In => {
                if c == '}' {
                    state = ParseState::Brace;
                } else {
                    part.push(c);
                }
            }
            ParseState::Bracket => {
                if c == '{' {
                    v.push(Part::Text(part));
                    part = String::new();
                    state = ParseState::In;
                } else {
                    part.push('<');
                    part.push(c);
                    state = ParseState::Out;
                }
            }
            ParseState::Brace => {
                if c == '>' {
                    let part_name = part.clone();
                    part = String::new();
                    let p = find_part_from_name(part_name.trim()).unwrap();
                    v.push(Part::Field(p));
                    state = ParseState::Out;
                } else {
                    panic!("Invalid character: {c}");
                }
            }
        }
    }

    v.push(Part::Text(part))
}

fn find_part_from_name(name: &str) -> Option<PageField> {
    if name == "name" {
        Some(PageField::Name)
    } else if name == "title" {
        Some(PageField::Title)
    } else if name == "icon" {
        Some(PageField::Icon)
    } else if name == "help" {
        Some(PageField::Help)
    } else if name == "desc" {
        Some(PageField::Desc)
    } else if name == "body" {
        Some(PageField::Body)
    } else {
        None
    }
}

fn read_page_from_file(file: &str) -> Result<Page, String> {
    let page_content = read_to_string(&file).unwrap();
    match toml::from_str(page_content.as_str()) {
        Ok(p) => Ok(p),
        Err(e) => Err(format!("{}", e.to_string())),
    }
}

fn read_html_from_file(file: &str) -> Result<Vec<Part>, String> {
    let mut v: Vec<Part> = Vec::new();

    process_template(read_to_string(file).unwrap().as_str(), &mut v);

    Ok(v)
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
                warn!(target:"PageCacheLogic", "find_resource: path ({html_path} found, but returning None");

                match read_page_from_file(page_path.as_str()) {
                    Ok(p) => {
                        let html = read_to_string(&html_path).unwrap();
                        Some(PageCacheEntry {
                            page: p,
                            html,
                            html_ts: metadata(&html_path).unwrap().modified().unwrap(),
                            page_ts: metadata(&page_path).unwrap().modified().unwrap(),
                            html_path,
                            page_path,
                        })
                    }
                    Err(e) => {
                        error!("Unable to parse page file {page_path}: {}", e.to_string());
                        None
                    }
                }
            } else {
                match read_page_from_file(page_path.as_str()) {
                    Ok(p) => match read_to_string(page_path.as_str()) {
                        Ok(s) => {
                            info!("html_path = {html_path}");
                            let html_ts = metadata(html_path.as_str()).unwrap().modified().unwrap();
                            info!("page_path = {page_path}");
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
        state: &PageCacheState,
        entry: &PageCacheEntry,
    ) -> Result<String, (u32, String)> {
        let mut content = String::new();

        state.parts.iter().for_each(|it| {
            content.push_str(it.to_string(entry).as_str());
        });

        Ok(content)
    }
}

pub type PageCache = ResCache<PageCacheState, PageCacheEntry, PageCacheLogic>;

impl PageCache {
    pub fn from_root_and_file(root: &str, file: &str) -> Result<PageCache, String> {
        info!("Calling from_root_and_file on {root}, {file}");

        match read_html_from_file(file) {
            Ok(v) => Ok(PageCache {
                phantom: std::marker::PhantomData,
                state: PageCacheState {
                    page_root: root.to_string(),
                    html_template: file.to_string(),
                    parts: v,
                    timestamp: SystemTime::now(),
                },
                map: HashMap::new(),
            }),
            Err(s) => Err(s),
        }
    }
}
