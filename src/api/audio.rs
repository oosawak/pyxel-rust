/// Audio API
///
/// wasm-backend: calls window.pyxelPlaySound(ch, sound) in JS.
/// JS is responsible for Web Audio SFX + BGM playback.

pub fn play(ch: u32, sound: u32, _sec: Option<f32>, _loop_: Option<bool>, _resume: Option<bool>) {
    #[cfg(feature = "pyxel-core-backend")]
    {
        let loop_val = _loop_.unwrap_or(false);
        let resume_val = _resume.unwrap_or(false);
        pyxel::pyxel().play_sound(ch, sound, _sec, loop_val, resume_val);
    }
    #[cfg(feature = "wasm-backend")]
    {
        use wasm_bindgen::JsValue;
        use js_sys::{Function, Reflect};
        if let Some(window) = web_sys::window() {
            if let Ok(f) = Reflect::get(&window, &JsValue::from_str("pyxelPlaySound")) {
                if f.is_function() {
                    let _ = Function::from(f).call2(
                        &JsValue::NULL,
                        &JsValue::from_f64(ch as f64),
                        &JsValue::from_f64(sound as f64),
                    );
                }
            }
        }
    }
    #[cfg(not(any(feature = "pyxel-core-backend", feature = "wasm-backend")))]
    { let _ = (ch, sound); }
}

pub fn playm(_msc: u32, _sec: Option<f32>, _loop_: Option<bool>) {
    #[cfg(feature = "pyxel-core-backend")]
    {
        pyxel::pyxel().play_music(_msc, _sec, _loop_.unwrap_or(false));
    }
}

pub fn stop(ch: Option<u32>) {
    #[cfg(feature = "pyxel-core-backend")]
    {
        match ch {
            Some(c) => pyxel::pyxel().stop_channel(c),
            None => pyxel::pyxel().stop_all_channels(),
        }
    }
    #[cfg(feature = "wasm-backend")]
    {
        use wasm_bindgen::JsValue;
        use js_sys::{Function, Reflect};
        if let Some(window) = web_sys::window() {
            if let Ok(f) = Reflect::get(&window, &JsValue::from_str("pyxelStopSound")) {
                if f.is_function() {
                    let arg = ch.map(|c| JsValue::from_f64(c as f64)).unwrap_or(JsValue::NULL);
                    let _ = Function::from(f).call1(&JsValue::NULL, &arg);
                }
            }
        }
    }
    #[cfg(not(any(feature = "pyxel-core-backend", feature = "wasm-backend")))]
    { let _ = ch; }
}

pub fn is_playing(_ch: u32) -> bool {
    false
}
