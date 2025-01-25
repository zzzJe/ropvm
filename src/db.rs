use sled::IVec;
pub use sled::{Result, Tree};

pub struct Database {
    config: Tree,
    version: Tree,
    cache: Tree,
}

impl Database {
    pub fn new() -> Self {
        let db = sled::open("db").expect("Failed to open database");
        let config_tree = db
            .open_tree("config")
            .expect("Failed to open database:config");
        let version_tree = db
            .open_tree("version")
            .expect("Failed to open database:version");
        let cache_tree = db
            .open_tree("cache")
            .expect("Failed to open database:cache");
        Self {
            config: config_tree,
            version: version_tree,
            cache: cache_tree,
        }
    }
    pub fn get_config_db(&self) -> Tree {
        self.config.clone()
    }
    pub fn get_version_db(&self) -> Tree {
        self.version.clone()
    }
    pub fn get_cache_db(&self) -> Tree {
        self.cache.clone()
    }
}

pub fn ivec_to_string(ivec: &IVec) -> String {
    std::str::from_utf8(ivec).unwrap().to_string()
}
