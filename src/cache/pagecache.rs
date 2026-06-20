////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Template Based Cache
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::cache::rescache::{CacheLogic, CacheState, ResCache};

use std::{
    collections::HashMap,
    fs::{exists, metadata, read_dir, read_to_string},
    path::Path,
    time::SystemTime,
};

use serde::{Deserialize, Serialize};

use mime::Mime;

use log::{debug, error, info, warn};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Page {
    pub name: String,
    pub title: String,
    pub icon: String,
    pub desc: String,
    pub help: String,
    pub css: Vec<String>,
    pub js: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PageCacheEntry {
    pub id: String,
    pub page: Page,
    pub html: String,
    pub page_path: String,
    pub html_path: String,
    pub page_ts: SystemTime,
    pub html_ts: SystemTime,
}

#[derive(Clone, Debug)]
pub enum PageField {
    Id,
    Title,
    Name,
    Icon,
    Help,
    Desc,
    Body,
    Css,
    Js,
}

impl PageCacheEntry {
    pub fn value_for(&self, f: &PageField) -> String {
        match f {
            PageField::Id => self.id.clone(),
            PageField::Name => self.page.name.clone(),
            PageField::Title => self.page.title.clone(),
            PageField::Icon => format!("/static/svg/pages/black/{}", self.page.icon),
            PageField::Help => self.page.help.clone(),
            PageField::Desc => self.page.desc.clone(),
            PageField::Body => self.html.clone(),
            PageField::Css => css_as_string(&self.page.css),
            PageField::Js => js_as_string(&self.page.js),
        }
    }
}

fn js_as_string(js: &Vec<String>) -> String {
    let mut result = String::new();

    js.iter()
        .for_each(|i| result.push_str(format!("<script src=\"{i}\"></script>").as_str()));

    result
}

fn css_as_string(css: &Vec<String>) -> String {
    let mut result = String::new();

    css.iter()
        .for_each(|i| result.push_str(format!("<link rel=\"stylesheet\" href=\"{i}\"/>").as_str()));

    result
}

#[derive(Clone, Debug)]
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

#[derive(Clone)]
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
        info!(target: "PageCacheLogic", "does {} need sync? {res}", self.html_template);
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

#[derive(Clone)]
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
        // info!(target: "process_template", "State = {state:?}, c = {c:?}, i = {i}/{}", s.len());
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
                    let part_name = part.trim().to_string();
                    part = String::new();
                    let p = find_part_from_name(part_name.trim());
                    match p {
                        Some(pp) => {
                            v.push(Part::Field(pp));
                            state = ParseState::Out;
                        }
                        None => {
                            panic!("Invalid part name: {part_name:?}");
                        }
                    }
                } else {
                    panic!("Invalid character: {c}");
                }
            }
        }
    }

    v.push(Part::Text(part))
}

fn find_part_from_name(name: &str) -> Option<PageField> {
    if name == "id" {
        Some(PageField::Id)
    } else if name == "name" {
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
    } else if name == "css" {
        Some(PageField::Css)
    } else if name == "js" {
        Some(PageField::Js)
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

    process_template(
        read_to_string(file)
            .expect(format!("Couln't read file {file}").as_str())
            .as_str(),
        &mut v,
    );

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
        let page_spath = format!("{}/{cache_key}.toml", state.page_root);
        let html_spath = format!("{}/{cache_key}.html", state.page_root);

        let page_ppath = Path::new(page_spath.as_str());
        let html_ppath = Path::new(html_spath.as_str());

        let page_path = String::from(page_ppath.to_str().unwrap());
        let html_path = String::from(html_ppath.to_str().unwrap());
        let page_id = String::from(page_ppath.file_stem().unwrap().to_str().unwrap());

        if exists(page_path.as_str()).unwrap() {
            if exists(html_path.as_str()).unwrap() {
                debug!(target:"PageCacheLogic", "find_resource: path ({html_path}) found");
                match read_page_from_file(page_path.as_str()) {
                    Ok(p) => {
                        debug!(target:"PageCacheLogic", "find_resource: read page ({html_path}) okay");
                        let html = read_to_string(&html_path).unwrap();
                        Some(PageCacheEntry {
                            id: page_id,
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
                debug!(target:"PageCacheLogic", "find_resource: file {page_path} does not exist");
                match read_page_from_file(page_path.as_str()) {
                    Ok(p) => match read_to_string(page_path.as_str()) {
                        Ok(s) => {
                            info!("html_path = {html_path}");
                            let html_ts = metadata(html_path.as_str()).unwrap().modified().unwrap();
                            info!("page_path = {page_path}");
                            let page_ts = metadata(page_path.as_str()).unwrap().modified().unwrap();
                            Some(PageCacheEntry {
                                id: page_id,
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
    pub fn from_root_and_file(dev_mode: bool, root: &str, file: &str) -> Result<PageCache, String> {
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
                dynamic: dev_mode,
            }),
            Err(s) => Err(s),
        }
    }

    pub fn initialize(&mut self) {
        for file in read_dir(self.state.page_root.as_str()).unwrap() {
            let entry = file.unwrap();

            let path = entry.path();
            let ext = path.extension().unwrap();
            let stem = path.file_stem().unwrap();

            if ext == "toml" {
                self.get_entry(stem.to_str().unwrap()).unwrap();
            }
        }
    }
}
