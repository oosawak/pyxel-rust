//! Window, text, file, palette and official archive API exercise.
use crate::api::*;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
thread_local! {static TEXT:RefCell<String>=const{RefCell::new(String::new())};static DROPS:RefCell<serde_json::Value>=RefCell::new(serde_json::json!([]));}
fn err(e: String) -> JsValue {
    JsValue::from_str(&e)
}
fn palette() {
    let mut bank = colors();
    bank.resize(256, 0);
    for i in 16..256 {
        let r = (i % 16) * 17;
        let g = (i / 16) * 17;
        bank[i] = ((r as u32) << 16) | ((g as u32) << 8) | (255 - r) as u32;
    }
}
#[wasm_bindgen]
pub fn start_browser_system() -> Result<(), JsValue> {
    start_browser_system_with_options(0, 2, 0, false, KEY_ESCAPE)
}
#[wasm_bindgen]
pub fn start_browser_system_with_options(
    display_scale: u32,
    capture_scale: u32,
    capture_sec: u32,
    headless: bool,
    quit_key: u32,
) -> Result<(), JsValue> {
    init_with_options(
        320,
        192,
        "Pyxel Rust / Browser APIs",
        30,
        InitOptions {
            display_scale: (display_scale > 0).then_some(display_scale),
            capture_scale,
            capture_sec,
            headless,
            quit_key,
        },
    )
    .map_err(err)?;
    mouse(true);
    palette();
    images()[1].borrow_mut().draw_rect(0., 0., 16., 16., 200);
    tilemaps()[0].borrow_mut().imgsrc = ImageSource::Index(1);
    sounds()[0]
        .lock()
        .unwrap()
        .set("c2e2g2c3", "p", "7", "n", 5)
        .map_err(err)?;
    musics()[0].lock().unwrap().set(vec![vec![0]]);
    let update = || {
        let text = input_text();
        if !text.is_empty() {
            TEXT.with(|v| v.borrow_mut().push_str(&text));
        }
        let names = dropped_files();
        if !names.is_empty() {
            DROPS.with(|v|*v.borrow_mut()=serde_json::Value::Array(names.iter().enumerate().map(|(index,name)|{
            let bytes=dropped_file_bytes(index).unwrap_or_default();serde_json::json!({"name":name,"bytes":bytes.len(),"preview":String::from_utf8_lossy(&bytes[..bytes.len().min(48)])})
        }).collect()));
        }
    };
    let draw = || {
        cls(1);
        text(8., 8., "WINDOW / TEXT / FILES / PALETTE", 7);
        for i in 0..256 {
            rectfill(
                (8 + (i % 16) * 8) as f32,
                (28 + (i / 16) * 6) as f32,
                8.,
                6.,
                i as u8,
            );
        }
        text(160., 32., "LIVE BANK 1", 7);
        blt_transform(184., 72., 1, 0., 0., 16., 16., None, None, Some(3.));
        text(8., height() as f32 - 40., "PYXRES / PYXPAL / TMX / PNG", 7);
        text(
            8.,
            height() as f32 - 24.,
            "256 COLORS / IME / FILE DROP",
            11,
        );
    };
    draw();
    if headless {
        crate::backend::wasm_backend::recording::capture();
    } else {
        run(update, draw);
    }
    Ok(())
}
#[wasm_bindgen]
pub fn browser_demo_info() -> String {
    serde_json::json!({"frame":frame_count(),"width":width(),"height":height(),"palette":colors().len(),"images":images().len(),
        "bank_color":images().get(1).map(|i|i.borrow().pixel(2.,2.)),"text":TEXT.with(|v|v.borrow().clone()),"drops":DROPS.with(|v|v.borrow().clone()),
        "playing":is_playing(0),"options":({let s=crate::backend::wasm_backend::state();serde_json::json!({"display_scale":s.display_scale,"capture_scale":s.capture_scale,"headless":s.headless,"quit_key":s.quit_key,"running":s.running,"recorded":s.recorder.as_ref().map_or(0,|r|r.frame_count())})})}).to_string()
}
#[wasm_bindgen]
pub fn browser_demo_resize(w: u32, h: u32) -> Result<(), JsValue> {
    resize(w, h).map_err(err)
}
#[wasm_bindgen]
pub fn browser_demo_title(value: &str) {
    title(value)
}
#[wasm_bindgen]
pub fn browser_demo_flags(integer: bool, monitor: bool) {
    integer_scale(integer);
    perf_monitor(monitor)
}
#[wasm_bindgen]
pub fn browser_demo_icon() -> Result<(), JsValue> {
    icon(
        &[
            "00088000", "00888800", "088cc880", "888cc888", "88888888", "08888880", "00888800",
            "00088000",
        ],
        4,
        Some(0),
    )
    .map_err(err)
}
#[wasm_bindgen]
pub fn browser_demo_edit() -> Result<(), JsValue> {
    let image = images()
        .get(1)
        .cloned()
        .ok_or_else(|| err("Image bank 1 does not exist".into()))?;
    let color = if image.borrow().pixel(2., 2.) == 200 {
        201
    } else {
        200
    };
    image.borrow_mut().draw_rect(0., 0., 16., 16., color);
    Ok(())
}
#[wasm_bindgen]
pub fn browser_demo_save() -> Result<Vec<u8>, JsValue> {
    save_bytes(ResourceOptions::default()).map_err(err)
}
#[wasm_bindgen]
pub fn browser_demo_load(bytes: &[u8]) -> Result<(), JsValue> {
    load_bytes(bytes, ResourceOptions::default()).map_err(err)
}
#[wasm_bindgen]
pub fn browser_demo_palette(text: &str) -> Result<(), JsValue> {
    load_pal_text(text).map_err(err)
}
#[wasm_bindgen]
pub fn browser_demo_palette_text() -> String {
    save_pal_text()
}
#[wasm_bindgen]
pub fn browser_demo_reset_palette() {
    palette()
}
#[wasm_bindgen]
pub fn browser_demo_tmx(text: &str) -> Result<(), JsValue> {
    let map = Tilemap::from_tmx_text(text, 0).map_err(err)?;
    with_tilemaps(|banks| banks[0] = map);
    Ok(())
}
#[wasm_bindgen]
pub fn browser_demo_bgm() -> Result<String, JsValue> {
    let parts = gen_bgm(0, 0, 3, 123, true).map_err(err)?;
    Ok(serde_json::to_string(&parts).unwrap())
}
#[wasm_bindgen]
pub fn browser_demo_stop() {
    stop(None)
}

/// Exercise the official filled/outline names in a small exportable image.
#[wasm_bindgen]
pub fn browser_demo_image_png(scale: u32) -> Result<Vec<u8>, JsValue> {
    let image = Image::new(8, 8);
    image.borrow_mut().clear(11);
    image.borrow_mut().draw_rect(0., 0., 4., 4., 8);
    image.borrow_mut().draw_rect(4., 4., 4., 4., 8);
    let result = image.borrow().png_bytes_scaled(scale).map_err(err);
    result
}
#[wasm_bindgen]
pub fn browser_demo_shapes() {
    cls(1);
    rect(8., 28., 16., 16., 8);
    rectb(32., 28., 16., 16., 8);
    circ(64., 36., 8., 11);
    circb(88., 36., 8., 11);
}
