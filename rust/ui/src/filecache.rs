////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// File Caching
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{
    collections::HashMap,
    fs::{exists, metadata, read_to_string},
};

use mime::Mime;

use crate::rescache::{CacheEntry, CacheLogic, CacheState, ResCache};

type FileCacheEntry = CacheEntry<String>;

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

pub struct FileCacheLogic {}

impl CacheLogic<FileCacheState, String> for FileCacheLogic {
    fn needs_sync(_state: &FileCacheState, entry: &FileCacheEntry, _cache_key: &str) -> bool {
        exists(entry.obj.as_str()).unwrap()
            && entry.timestamp < metadata(entry.obj.as_str()).unwrap().modified().unwrap()
    }

    fn sync(
        _state: &FileCacheState,
        entry: &mut CacheEntry<String>,
        _cache_key: &str,
    ) -> Option<String> {
        entry.data = read_to_string(entry.obj.as_str()).unwrap();
        entry.timestamp = metadata(entry.obj.as_str()).unwrap().modified().unwrap();
        None
    }

    fn find_resource(state: &FileCacheState, cache_key: &str) -> Option<CacheEntry<String>> {
        let path = format!("{}/{}", state.root, cache_key);
        if exists(&path).unwrap() {
            Some(FileCacheEntry {
                data: read_to_string(&path).unwrap(),
                timestamp: metadata(&path).unwrap().modified().unwrap(),
                obj: path,
            })
        } else {
            None
        }
    }

    fn mime_type(state: &FileCacheState, _cache_key: &str) -> Mime {
        state.mime.clone()
    }
}

pub type FileCache = ResCache<FileCacheState, String, FileCacheLogic>;

impl FileCache {
    pub fn from_mime_and_root(mime: Mime, root: &str) -> FileCache {
        FileCache {
            phantom: std::marker::PhantomData,
            state: FileCacheState {
                root: root.to_string(),
                mime,
            },
            map: HashMap::new(),
        }
    }
}
