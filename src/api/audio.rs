//! Stateful Pyxel synthesis on SDL2 and Web Audio, with optional legacy JS hooks.
#[cfg(all(feature = "wasm-backend", target_arch = "wasm32"))]
fn hook(name: &str, args: &[wasm_bindgen::JsValue]) -> Option<wasm_bindgen::JsValue> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (name, args);
        return None;
    }
    #[cfg(target_arch = "wasm32")]
    {
        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen::JsValue;
        let window = web_sys::window()?;
        let function = js_sys::Reflect::get(&window, &JsValue::from_str(name)).ok()?;
        if !function.is_function() {
            return None;
        }
        let args: js_sys::Array = args.iter().cloned().collect();
        Some(
            js_sys::Function::from(function)
                .apply(&JsValue::NULL, &args)
                .unwrap_or(JsValue::UNDEFINED),
        )
    }
}
pub fn play(ch: u32, sound: u32, sec: Option<f32>, loop_: Option<bool>, resume: Option<bool>) {
    try_play(ch, sound, sec, loop_, resume).expect("Failed to play sound");
}
pub fn try_play(
    ch: u32,
    sound: u32,
    sec: Option<f32>,
    loop_: Option<bool>,
    resume: Option<bool>,
) -> Result<(), String> {
    #[cfg(feature = "pyxel-core-backend")]
    {
        pyxel::pyxel().play_sound(
            ch,
            sound,
            sec,
            loop_.unwrap_or(false),
            resume.unwrap_or(false),
        )?;
    }
    #[cfg(feature = "wasm-backend")]
    {
        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen::JsValue as J;
        #[cfg(target_arch = "wasm32")]
        if hook(
            "pyxelPlaySound",
            &[
                ch.into(),
                sound.into(),
                sec.map_or(J::NULL, |s| J::from_f64(s as f64)),
                loop_.unwrap_or(false).into(),
                resume.unwrap_or(false).into(),
            ],
        )
        .is_some()
        {
            return Ok(());
        }
        let sound = pyxel::sounds()
            .get(sound as usize)
            .cloned()
            .ok_or("Invalid sound index")?;
        let channel = pyxel::channels()
            .get(ch as usize)
            .cloned()
            .ok_or("Invalid channel index")?;
        channel.lock().unwrap().play_sound(
            sound,
            sec,
            loop_.unwrap_or(false),
            resume.unwrap_or(false),
        )?;
        crate::backend::wasm_backend::audio::request_resume();
    }
    #[cfg(not(any(feature = "pyxel-core-backend", feature = "wasm-backend")))]
    {
        let _ = (ch, sound, sec, loop_, resume);
    }
    Ok(())
}
pub fn playm(music: u32, sec: Option<f32>, loop_: Option<bool>) {
    try_playm(music, sec, loop_).expect("Failed to play music");
}
pub fn try_playm(music: u32, sec: Option<f32>, loop_: Option<bool>) -> Result<(), String> {
    #[cfg(feature = "pyxel-core-backend")]
    {
        pyxel::pyxel().play_music(music, sec, loop_.unwrap_or(false))?;
    }
    #[cfg(feature = "wasm-backend")]
    {
        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen::JsValue as J;
        #[cfg(target_arch = "wasm32")]
        if hook(
            "pyxelPlayMusic",
            &[
                music.into(),
                sec.map_or(J::NULL, |s| J::from_f64(s as f64)),
                loop_.unwrap_or(false).into(),
            ],
        )
        .is_some()
        {
            return Ok(());
        }
        let music = pyxel::musics()
            .get(music as usize)
            .cloned()
            .ok_or("Invalid music index")?;
        let sequences = music.lock().unwrap().seqs.clone();
        let channels = crate::api::resource::channels();
        let sounds = crate::api::resource::sounds();
        // Validate and prepare every channel first; invalid music leaves active playback intact.
        let mut prepared = Vec::new();
        for (index, seq) in sequences
            .iter()
            .enumerate()
            .take(channels.len())
            .filter(|(_, seq)| !seq.is_empty())
        {
            let selected = seq
                .iter()
                .map(|&s| {
                    sounds
                        .get(s as usize)
                        .cloned()
                        .ok_or_else(|| "Invalid sound index in music".to_string())
                })
                .collect::<Result<Vec<_>, _>>()?;
            let candidate = pyxel::Channel::new();
            candidate
                .lock()
                .unwrap()
                .play(selected.clone(), sec, loop_.unwrap_or(false), false)?;
            prepared.push((index, selected));
        }
        for (index, selected) in prepared {
            channels[index]
                .lock()
                .unwrap()
                .play(selected, sec, loop_.unwrap_or(false), false)?;
        }
        crate::backend::wasm_backend::audio::request_resume();
    }
    #[cfg(not(any(feature = "pyxel-core-backend", feature = "wasm-backend")))]
    {
        let _ = (music, sec, loop_);
    }
    Ok(())
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
        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen::JsValue;
        #[cfg(target_arch = "wasm32")]
        if hook("pyxelStopSound", &[ch.map_or(JsValue::NULL, |c| c.into())]).is_some() {
            return;
        }
        let channels = crate::api::resource::channels();
        for (index, channel) in channels.iter().enumerate() {
            if ch.is_none() || ch == Some(index as u32) {
                channel.lock().unwrap().stop();
            }
        }
    }
    #[cfg(not(any(feature = "pyxel-core-backend", feature = "wasm-backend")))]
    {
        let _ = ch;
    }
}
pub fn play_pos(ch: u32) -> Option<(u32, f32)> {
    #[cfg(feature = "pyxel-core-backend")]
    {
        return pyxel::pyxel().play_position(ch);
    }
    #[cfg(feature = "wasm-backend")]
    {
        return crate::api::resource::channels()
            .get(ch as usize)?
            .lock()
            .unwrap()
            .play_position();
    }
    #[cfg(not(any(feature = "pyxel-core-backend", feature = "wasm-backend")))]
    {
        let _ = ch;
        None
    }
}
pub fn is_playing(ch: u32) -> bool {
    play_pos(ch).is_some()
}
