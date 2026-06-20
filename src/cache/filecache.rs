////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// File Caching
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{
    collections::HashMap,
    fs::{exists, metadata, read_to_string},
    time::SystemTime,
};

use mime::Mime;

use crate::cache::rescache::{CacheLogic, CacheState, ResCache};

#[derive(Debug)]
pub struct StaticFileData {
    pub path: String,
    pub data: String,
    pub timestamp: SystemTime,
}

#[derive(Clone)]
pub struct FileCacheState {
    pub root: String,
    pub mime: Mime,
}

impl CacheState for FileCacheState {
    fn needs_sync(&self) -> bool {
        false
    }
    fn sync(&mut self) -> Option<String> {
        None
    }
}

#[derive(Clone)]
pub struct FileCacheLogic {}

impl CacheLogic<FileCacheState, StaticFileData> for FileCacheLogic {
    fn needs_sync(_state: &FileCacheState, entry: &StaticFileData, _cache_key: &str) -> bool {
        exists(entry.path.as_str()).unwrap()
            && entry.timestamp < metadata(entry.path.as_str()).unwrap().modified().unwrap()
    }

    fn sync(
        _state: &FileCacheState,
        entry: &mut StaticFileData,
        _cache_key: &str,
    ) -> Option<String> {
        entry.data = read_to_string(entry.path.as_str()).unwrap();
        entry.timestamp = metadata(entry.path.as_str()).unwrap().modified().unwrap();
        None
    }

    fn find_resource(state: &FileCacheState, cache_key: &str) -> Option<StaticFileData> {
        let path = format!("{}/{}", state.root, cache_key);
        if exists(&path).unwrap() {
            Some(StaticFileData {
                timestamp: metadata(&path).unwrap().modified().unwrap(),
                data: read_to_string(&path).unwrap(),
                path,
            })
        } else {
            None
        }
    }

    fn mime_type(state: &FileCacheState, _cache_key: &str) -> Mime {
        state.mime.clone()
    }

    fn generate_content(
        _state: &FileCacheState,
        entry: &StaticFileData,
    ) -> Result<String, (u32, String)> {
        Ok(entry.data.clone())
    }
}

pub type FileCache = ResCache<FileCacheState, StaticFileData, FileCacheLogic>;

impl FileCache {
    pub fn from_mime_and_root(dev_mode: bool, mime: Mime, root: &str) -> FileCache {
        FileCache {
            phantom: std::marker::PhantomData,
            state: FileCacheState {
                root: root.to_string(),
                mime,
            },
            map: HashMap::new(),
            dynamic: dev_mode,
        }
    }
}
