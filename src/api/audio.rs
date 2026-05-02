/// Audio API - stub for wgpu-backend (sound system coming later)

pub fn play(_ch: u32, _sound: u32, _sec: Option<f32>, _loop_: Option<bool>, _resume: Option<bool>) {
    #[cfg(feature = "pyxel-core-backend")]
    {
        let loop_val = _loop_.unwrap_or(false);
        let resume_val = _resume.unwrap_or(false);
        pyxel::pyxel().play_sound(_ch, _sound, _sec, loop_val, resume_val);
    }
    // wgpu-backend: audio not yet implemented
}
pub fn playm(_msc: u32, _sec: Option<f32>, _loop_: Option<bool>) {
    #[cfg(feature = "pyxel-core-backend")]
    {
        pyxel::pyxel().play_music(_msc, _sec, _loop_.unwrap_or(false));
    }
}
pub fn stop(_ch: Option<u32>) {
    #[cfg(feature = "pyxel-core-backend")]
    {
        match _ch {
            Some(c) => pyxel::pyxel().stop_channel(c),
            None => pyxel::pyxel().stop_all_channels(),
        }
    }
}
pub fn is_playing(_ch: u32) -> bool {
    false
}
