////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Generic Resource Caching
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{collections::HashMap, marker::PhantomData};

use http::Response;
use infra::error::ErrorResponse;
use log::{info, warn};
use mime::Mime;

pub trait CacheState {
    fn needs_sync(&self) -> bool;
    fn sync(&mut self) -> Option<String>;
}

pub trait CacheLogic<S, E>
where
    S: CacheState,
{
    fn needs_sync(state: &S, entry: &E, cache_key: &str) -> bool;
    fn sync(state: &S, entry: &mut E, cache_key: &str) -> Option<String>;
    fn find_resource(state: &S, cache_key: &str) -> Option<E>;
    fn mime_type(state: &S, cache_key: &str) -> Mime;
    fn generate_content(state: &S, entry: &E) -> Result<String, (u32, String)>;
}

pub struct ResCache<S, E, L>
where
    S: CacheState,
    L: CacheLogic<S, E>,
{
    pub phantom: PhantomData<L>,
    pub state: S,
    pub map: HashMap<String, E>,
    pub dynamic: bool,
}

impl<S, E, L> ResCache<S, E, L>
where
    S: CacheState,
    L: CacheLogic<S, E>,
{
    pub fn new(state: S) -> ResCache<S, E, L> {
        ResCache {
            phantom: PhantomData,
            state,
            map: HashMap::new(),
            dynamic: true,
        }
    }

    pub fn get_entry(&mut self, cache_key: &str) -> Option<String> {
        // Check if the cache itself needs to sync
        if self.state.needs_sync() {
            match self.state.sync() {
                Some(err_msg) => {
                    warn!(target: "ResCache", "Error synching cache state: {err_msg}");
                }
                None => {
                    info!(target: "ResCache", "Resynched Cache State");
                }
            }
        } else {
            info!(target: "ResCache", "No synch needed for {cache_key}");
        }

        info!("About to look for the entry for {cache_key}");

        // Look for the entry
        match self.map.get_mut(cache_key) {
            Some(e) => {
                info!(target: "get_entry", "Got cache hit for {cache_key}");
                if L::needs_sync(&self.state, e, cache_key) {
                    info!(target: "get_entry", "{cache_key} needs synch");
                    match L::sync(&self.state, e, cache_key) {
                        Some(err_msg) => {
                            warn!(target: "ResCache", "Error synching cache state for ({cache_key}): {err_msg}");
                        }
                        _ => {
                            info!("{cache_key} synched successfully")
                        }
                    }
                } else {
                    info!(target: "get_entry", "{cache_key} up to date");
                }

                match L::generate_content(&self.state, e) {
                    Ok(s) => Some(s),
                    Err((_i, m)) => {
                        warn!("Unable to get content for {cache_key}: {m}");
                        None
                    }
                }
            }
            None => {
                info!(target: "get_entry", "Cache miss for {cache_key}");

                match L::find_resource(&self.state, cache_key) {
                    Some(e) => {
                        info!(target: "get_entry", "Found resource for {cache_key}, creating entry");
                        let content = L::generate_content(&self.state, &e);
                        self.map.insert(cache_key.to_string(), e);
                        match content {
                            Ok(s) => Some(s),
                            Err(_) => None,
                        }
                    }
                    None => {
                        info!(target: "get_entry", "No resource found for {cache_key}, returning 404");
                        None
                    }
                }
            }
        }
    }

    pub fn get_result(&mut self, cache_key: &str) -> Result<Response<String>, ErrorResponse> {
        match self.get_entry(cache_key) {
            Some(s) => {
                let mime = L::mime_type(&self.state, cache_key);
                let builder = Response::builder().header("Content-Type", format!("{mime}"));

                Ok(builder.body(s.clone()).unwrap())
            }
            None => {
                let builder = Response::builder()
                    .header("Content-Type", mime::TEXT_PLAIN.to_string())
                    .status(404);

                Ok(builder
                    .body(format!("No entry found for '{cache_key}'"))
                    .unwrap())
            }
        }
    }
}
