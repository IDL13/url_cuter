use std::sync::Arc;
use dashmap::DashMap;

pub trait CreateShortUrlRepository {
    fn save(&self, full_url: String, id: String) -> Result<(), String>;
}

#[derive(Clone)]
pub struct StorageRepository {
    store: Arc<DashMap<String, String>>,
}

impl StorageRepository {
    pub fn new(store: Arc<DashMap<String, String>>) -> Self {
        Self { store }
    }
}

impl CreateShortUrlRepository for StorageRepository {
    fn save(&self, full_url: String, id: String) -> Result<(), String> {
        self.store.insert(id, full_url);

        Ok(())
    }
}

impl crate::app::query::get_full_url::GetFullUrlRepository for StorageRepository {
    fn get(&self, id: &str) -> Result<String, String> {
        match self.store.get(id) {
            Some(url) => Ok(url.clone()),
            None => Err("Not found".to_owned()),
        }
    }
}

