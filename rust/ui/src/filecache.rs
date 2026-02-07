////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// File Caching
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use http::response::Response;
use log::debug;
use mime::Mime;
use std::collections::HashMap;
use std::fs::{exists, metadata, read_to_string};
use std::time::SystemTime;

// ---------------------------------------------------------------------------------------------------------------------
// Define the cache logic
// ---------------------------------------------------------------------------------------------------------------------

pub trait CacheLogic {
    fn process_path(&self, file_name: &String) -> String;
    fn needs_update(&self, timestamp: SystemTime, file_name: &String) -> bool;
}

pub struct FileCacheEntry {
    data: String,
    timestamp: SystemTime,
}

pub struct FileCache<T: CacheLogic> {
    pub mime_type: Mime,
    pub root: String,
    pub logic: Box<T>,
    pub cache: HashMap<String, FileCacheEntry>,
}

pub fn create_file_response(content: &String, mime: &String) -> Response<String> {
    let builder = Response::builder().header("Content-Type", format!("{mime}"));

    builder.body(content.clone()).unwrap()
}

impl<T> FileCache<T>
where
    T: CacheLogic,
{
    pub fn new(logic: T, root: String, mime_type: Mime) -> FileCache<T> {
        let cache = HashMap::new();
        FileCache {
            root,
            mime_type,
            logic: Box::new(logic),
            cache,
        }
    }

    pub fn lookup_file(&mut self, file_name: &String) -> Result<Response<String>, String> {
        if exists(file_name).unwrap() {
            // debug!(target: "lookup_file", "file {file_name} exists");
            let mimetype = format!("{}", self.mime_type);
            match self.cache.get(file_name) {
                Some(entry) => {
                    // debug!(target: "lookup_file", "found an entry for {file_name}");
                    if self.logic.needs_update(entry.timestamp, file_name) {
                        debug!(target: "lookup_file", "{file_name} is out of date and needs to be updated");
                        let data = self.logic.process_path(file_name);
                        let return_data = data.clone();
                        let timestamp = metadata(file_name).unwrap().modified().unwrap();
                        self.cache
                            .insert(file_name.clone(), FileCacheEntry { data, timestamp });
                        Ok(create_file_response(&return_data, &mimetype))
                    } else {
                        // debug!(target: "lookup_file", "{file_name} is current and is being returned as is");
                        Ok(create_file_response(&entry.data, &mimetype))
                    }
                }
                None => {
                    // debug!(target: "lookup_file", "{file_name} needs to be added to the cache");
                    let data = self.logic.process_path(file_name);
                    let return_data = data.clone();
                    let timestamp = metadata(file_name).unwrap().modified().unwrap();
                    self.cache
                        .insert(file_name.clone(), FileCacheEntry { data, timestamp });
                    Ok(create_file_response(&return_data, &mimetype))
                }
            }
        } else {
            Err(format!("File {file_name} not found"))
        }
    }
}

pub struct StaticFileCacheLogic {}

impl CacheLogic for StaticFileCacheLogic {
    fn process_path(&self, file_name: &String) -> String {
        // Assume the file exists
        debug!(target: "CacheLogic::process_path", "calling on {file_name}");
        read_to_string(file_name).unwrap()
    }

    fn needs_update(&self, timestamp: SystemTime, file_name: &String) -> bool {
        // Assume the file exists
        // let time_elapsed = timestamp.elapsed().unwrap().as_secs();
        // debug!(target: "CacheLogic::needs_update", "calling on {time_elapsed}, {file_name}");
        timestamp < metadata(file_name).unwrap().modified().unwrap()
    }
}
