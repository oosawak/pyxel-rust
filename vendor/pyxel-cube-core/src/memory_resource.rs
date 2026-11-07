//! Pyxel's official ZIP/TOML resource format with memory I/O instead of paths.
use crate::{ResourceData, RESOURCE_ARCHIVE_NAME, RESOURCE_FORMAT_VERSION};
use std::io::{Cursor, Read, Write};
use zip::{write::SimpleFileOptions, ZipArchive, ZipWriter};
#[derive(Clone, Copy, Default)]
pub struct ResourceOptions {
    pub exclude_images: bool,
    pub exclude_tilemaps: bool,
    pub exclude_sounds: bool,
    pub exclude_musics: bool,
}
pub fn save(options: ResourceOptions) -> Result<Vec<u8>, String> {
    let data = ResourceData::from_banks(
        options.exclude_images,
        options.exclude_tilemaps,
        options.exclude_sounds,
        options.exclude_musics,
    );
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    zip.start_file(RESOURCE_ARCHIVE_NAME, SimpleFileOptions::default())
        .map_err(|e| e.to_string())?;
    zip.write_all(data.to_toml().as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(zip.finish().map_err(|e| e.to_string())?.into_inner())
}
pub fn load(bytes: &[u8], options: ResourceOptions) -> Result<(), String> {
    let mut zip = ZipArchive::new(Cursor::new(bytes)).map_err(|e| e.to_string())?;
    let file = zip
        .by_name(RESOURCE_ARCHIVE_NAME)
        .map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.take(32 * 1024 * 1024 + 1)
        .read_to_string(&mut text)
        .map_err(|e| e.to_string())?;
    if text.len() > 32 * 1024 * 1024 {
        return Err("Resource manifest exceeds 32 MiB".into());
    }
    let data = ResourceData::from_toml(&text)?;
    if !(1..=RESOURCE_FORMAT_VERSION).contains(&data.format_version) {
        return Err(format!(
            "Unsupported resource version {}",
            data.format_version
        ));
    }
    data.apply_to_banks(
        options.exclude_images,
        options.exclude_tilemaps,
        options.exclude_sounds,
        options.exclude_musics,
    )
}
pub fn load_palette(text: &str) -> Result<(), String> {
    let mut colors = Vec::new();
    for (index, line) in text
        .lines()
        .enumerate()
        .filter(|(_, l)| !l.trim().is_empty())
    {
        let line = line.trim();
        if line.len() > 6 {
            return Err(format!("Invalid palette line {}", index + 1));
        }
        colors.push(
            u32::from_str_radix(line, 16)
                .map_err(|_| format!("Invalid palette line {}", index + 1))?,
        );
    }
    if colors.is_empty() || colors.len() > 256 {
        return Err("Palette must contain 1..256 RGB colors".into());
    }
    *crate::colors() = colors;
    Ok(())
}
pub fn save_palette() -> String {
    crate::colors()
        .iter()
        .map(|c| format!("{c:06x}\n"))
        .collect()
}
