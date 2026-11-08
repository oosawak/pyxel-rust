//! Persistent browser user data. A namespace replaces a desktop directory.
use wasm_bindgen::prelude::*;
pub struct UserDataStore {
    storage: web_sys::Storage,
    prefix: String,
}
impl UserDataStore {
    pub fn get(&self, key: &str) -> Result<Option<String>, String> {
        self.storage
            .get_item(&format!("{}{key}", self.prefix))
            .map_err(|e| format!("{e:?}"))
    }
    pub fn set(&self, key: &str, value: &str) -> Result<(), String> {
        self.storage
            .set_item(&format!("{}{key}", self.prefix), value)
            .map_err(|e| format!("{e:?}"))
    }
    pub fn remove(&self, key: &str) -> Result<(), String> {
        self.storage
            .remove_item(&format!("{}{key}", self.prefix))
            .map_err(|e| format!("{e:?}"))
    }
}
pub fn user_data_store(vendor: &str, app: &str) -> Result<UserDataStore, String> {
    if vendor.is_empty() || app.is_empty() {
        return Err("Vendor and app must not be empty".into());
    }
    let storage = web_sys::window()
        .ok_or("No window")?
        .local_storage()
        .map_err(|e| format!("{e:?}"))?
        .ok_or("Browser storage is unavailable")?;
    Ok(UserDataStore {
        storage,
        prefix: format!("pyxel:{}:{vendor}:{}:{app}:", vendor.len(), app.len()),
    })
}
