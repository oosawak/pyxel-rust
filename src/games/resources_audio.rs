//! Interactive shared-bank and streaming-audio example.
use crate::api::*;
use wasm_bindgen::prelude::*;
fn err(e: String) -> JsValue {
    JsValue::from_str(&e)
}
#[wasm_bindgen]
pub fn start_resources_audio() -> Result<(), JsValue> {
    init(320, 180, "Shared resources and Web Audio", 30);
    mouse(true);
    let sprite = images()[1].clone();
    sprite.borrow_mut().clear(0);
    sprite.borrow_mut().draw_rect(0., 0., 8., 8., 8);
    sprite.borrow_mut().draw_rect(8., 0., 8., 8., 10);
    sprite.borrow_mut().draw_rect(0., 8., 8., 8., 11);
    sprite.borrow_mut().draw_rect(8., 8., 8., 8., 12);
    let map = tilemaps()[0].clone();
    map.borrow_mut().imgsrc = ImageSource::Index(1);
    for y in 0..8 {
        for x in 0..10 {
            map.borrow_mut()
                .set_tile(x as f32, y as f32, ((x % 2) as u16, (y % 2) as u16));
        }
    }
    let font =
        Font::from_bytes(include_bytes!("assets/check-font.bdf"), "bdf", None).map_err(err)?;
    sounds()[0]
        .lock()
        .unwrap()
        .set_mml("t240 o4 l16 c e g > c")
        .map_err(err)?;
    sounds()[1]
        .lock()
        .unwrap()
        .set_mml("t240 o3 l16 c r g r")
        .map_err(err)?;
    let wav = sounds()[0].lock().unwrap().wav_bytes(0.25).map_err(err)?;
    sounds()[2]
        .lock()
        .unwrap()
        .load_pcm_bytes(&wav, "wav")
        .map_err(err)?;
    musics()[0].lock().unwrap().set(vec![vec![0], vec![1]]);
    run(
        || {},
        move || {
            cls(1);
            text(8., 8., "SHARED IMAGE / TILEMAP / FONT", 7);
            text(8., 28., "IMAGE BANK 1", 7);
            blt_transform(24., 58., 1, 0., 0., 16., 16., None, None, Some(3.));
            text(92., 28., "TILEMAP / IMGSRC=1", 7);
            bltm(92., 42., 0, 0., 0., 80., 64., None);
            text(192., 28., "ROTATE / SCALE", 7);
            blt_transform(224., 74., 1, 0., 0., 16., 16., None, Some(35.), Some(3.));
            text(8., 126., "BDF FROM BYTES", 7);
            text_with_font(8., 140., "AAAA", 10, Some(&font));
            text(92., 126., "MML / PCM / TWO CHANNELS", 7);
            text(
                92.,
                140.,
                if is_playing(0) || is_playing(1) {
                    "AUDIO: PLAYING"
                } else {
                    "AUDIO: STOPPED"
                },
                11,
            );
        },
    );
    Ok(())
}
#[wasm_bindgen]
pub fn resources_audio_info() -> String {
    serde_json::json!({"frame":frame_count(),"color":images()[1].borrow().pixel(2.,2.),
        "tile":tilemaps()[0].borrow().tile(0.,0.),"positions":[play_pos(0),play_pos(1),play_pos(2)],
        "playing":[is_playing(0),is_playing(1),is_playing(2)],
        "audio":serde_json::from_str::<serde_json::Value>(&crate::backend::wasm_backend::audio::browser_audio_status()).unwrap()}).to_string()
}
#[wasm_bindgen]
pub fn resources_audio_play(kind: u32, loop_: bool, start: f32) -> Result<(), JsValue> {
    if kind == 3 {
        try_playm(0, Some(start), Some(loop_)).map_err(err)
    } else {
        try_play(0, kind, Some(start), Some(loop_), Some(false)).map_err(err)
    }
}
#[wasm_bindgen]
pub fn resources_audio_stop(channel: Option<u32>) {
    stop(channel);
}
#[wasm_bindgen]
pub fn resources_audio_edit() {
    let color = if images()[1].borrow().pixel(2., 2.) == 8 {
        14
    } else {
        8
    };
    image_rect(1, 0., 0., 8., 8., color);
}
#[wasm_bindgen]
pub fn resources_audio_png() -> Result<Vec<u8>, JsValue> {
    let output = Image::new(16, 16);
    output
        .borrow_mut()
        .draw_image(0., 0., &images()[1], 0., 0., 16., 16., None, None, None);
    let result = output.borrow().png_bytes().map_err(err);
    result
}
#[wasm_bindgen]
pub fn resources_audio_load_png(bytes: &[u8]) -> Result<(), JsValue> {
    let source = Image::from_image_bytes(bytes, Some(false)).map_err(err)?;
    let size = {
        let image = source.borrow();
        (image.width(), image.height())
    };
    let target = images()[1].clone();
    let mut target = target.borrow_mut();
    target.clear(0);
    target.draw_image(
        0.,
        0.,
        &source,
        0.,
        0.,
        size.0 as f32,
        size.1 as f32,
        None,
        None,
        None,
    );
    Ok(())
}
#[wasm_bindgen]
pub fn resources_audio_wav() -> Result<Vec<u8>, JsValue> {
    sounds()[0].lock().unwrap().wav_bytes(0.5).map_err(err)
}
