//! Named browser files. On native hosts, the vfs:/ prefix explicitly selects the
//! same memory filesystem; other paths keep their ordinary filesystem behavior.
use std::{cell::RefCell, collections::BTreeMap};
thread_local! { static FILES: RefCell<BTreeMap<String, Vec<u8>>> = const { RefCell::new(BTreeMap::new()) }; }
const FILE_LIMIT: usize = 64 * 1024 * 1024;
const TOTAL_LIMIT: usize = 128 * 1024 * 1024;
pub fn is_virtual(path: &str) -> bool {
    cfg!(target_arch = "wasm32") || path.starts_with("vfs:")
}
pub fn normalize(path: &str) -> Result<String, String> {
    if path.len() > 4096 {
        return Err("Virtual filename exceeds 4096 bytes".into());
    }
    let path = path.strip_prefix("vfs:").unwrap_or(path);
    if path.contains(['\\', '\0', ':']) {
        return Err("Invalid virtual filename".into());
    }
    let mut parts = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                if parts.pop().is_none() {
                    return Err("Path escapes the virtual root".into());
                }
            }
            _ => parts.push(part),
        }
    }
    if parts.is_empty() {
        return Err("A filename is required".into());
    }
    Ok(format!("vfs:/{}", parts.join("/")))
}
pub fn put(path: &str, bytes: &[u8]) -> Result<(), String> {
    let path = normalize(path)?;
    if bytes.len() > FILE_LIMIT {
        return Err("Virtual file exceeds 64 MiB".into());
    }
    FILES.with(|files| {
        let mut files = files.borrow_mut();
        if !files.contains_key(&path) && files.len() >= 4096 {
            return Err("Virtual filesystem has 4096 files".into());
        }
        let used: usize = files.values().map(Vec::len).sum();
        if used - files.get(&path).map_or(0, Vec::len) + bytes.len() > TOTAL_LIMIT {
            return Err("Virtual files exceed 128 MiB".into());
        }
        files.insert(path, bytes.to_vec());
        Ok(())
    })
}
pub fn get(path: &str) -> Result<Vec<u8>, String> {
    let path = normalize(path)?;
    FILES.with(|files| {
        files
            .borrow()
            .get(&path)
            .cloned()
            .ok_or_else(|| format!("Virtual file not found: {path}"))
    })
}
pub fn list() -> Vec<String> {
    FILES.with(|files| files.borrow().keys().cloned().collect())
}
pub fn remove(path: &str) -> Result<bool, String> {
    let path = normalize(path)?;
    Ok(FILES.with(|files| files.borrow_mut().remove(&path).is_some()))
}
pub fn read(path: &str) -> Result<Vec<u8>, String> {
    if is_virtual(path) {
        get(path)
    } else {
        std::fs::read(path).map_err(|e| e.to_string())
    }
}
pub fn write(path: &str, bytes: &[u8]) -> Result<(), String> {
    if is_virtual(path) {
        put(path, bytes)
    } else {
        std::fs::write(path, bytes).map_err(|e| e.to_string())
    }
}
