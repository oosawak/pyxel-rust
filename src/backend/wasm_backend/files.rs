//! File picker/drop/download bridges for named virtual files.
use crate::api::browser_files;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn browser_files_write(path: &str, bytes: &[u8]) -> Result<(), JsValue> {
    browser_files::put(path, bytes).map_err(|e| JsValue::from_str(&e))
}
#[wasm_bindgen]
pub fn browser_files_read(path: &str) -> Result<Vec<u8>, JsValue> {
    browser_files::get(path).map_err(|e| JsValue::from_str(&e))
}
#[wasm_bindgen]
pub fn browser_files_list() -> String {
    serde_json::to_string(&browser_files::list()).unwrap()
}
#[wasm_bindgen]
pub fn browser_files_remove(path: &str) -> Result<bool, JsValue> {
    browser_files::remove(path).map_err(|e| JsValue::from_str(&e))
}
