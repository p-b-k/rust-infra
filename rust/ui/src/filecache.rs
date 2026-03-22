////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// File Caching
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{
    collections::HashMap,
    fs::{exists, metadata, read_to_string},
};

use mime::Mime;

use crate::rescache::{CacheEntry, CacheLogic, CacheState, ResCache};

pub struct StaticFileData {
    pub path: String,
    pub data: String,
}

type FileCacheEntry = CacheEntry<StaticFileData>;

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

impl CacheLogic<FileCacheState, StaticFileData> for FileCacheLogic {
    fn needs_sync(_state: &FileCacheState, entry: &FileCacheEntry, _cache_key: &str) -> bool {
        exists(entry.obj.path.as_str()).unwrap()
            && entry.timestamp
                < metadata(entry.obj.path.as_str())
                    .unwrap()
                    .modified()
                    .unwrap()
    }

    fn sync(
        _state: &FileCacheState,
        entry: &mut FileCacheEntry,
        _cache_key: &str,
    ) -> Option<String> {
        entry.obj.data = read_to_string(entry.obj.path.as_str()).unwrap();
        entry.timestamp = metadata(entry.obj.path.as_str())
            .unwrap()
            .modified()
            .unwrap();
        None
    }

    fn find_resource(state: &FileCacheState, cache_key: &str) -> Option<FileCacheEntry> {
        let path = format!("{}/{}", state.root, cache_key);
        if exists(&path).unwrap() {
            Some(FileCacheEntry {
                timestamp: metadata(&path).unwrap().modified().unwrap(),
                obj: StaticFileData {
                    data: read_to_string(&path).unwrap(),
                    path,
                },
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
        entry: &FileCacheEntry,
    ) -> Result<String, (u32, String)> {
        Ok(entry.obj.data.clone())
    }
}

pub type FileCache = ResCache<FileCacheState, StaticFileData, FileCacheLogic>;

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
