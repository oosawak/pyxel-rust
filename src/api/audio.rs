/// Audio API - Sound and music playback
/// 
/// Functions for playing sounds and music in your game.

/// Play a sound or list of sounds on a channel
/// 
/// # Arguments
/// * `ch` - Channel (0-3)
/// * `sound` - Sound index or slice of indices
/// * `sec` - Duration in seconds (None = full sound)
/// * `loop_` - Whether to loop (default: false)
/// * `resume` - Whether to resume if already playing (default: false)
pub fn play(ch: u32, sound: u32, sec: Option<f32>, loop_: Option<bool>, resume: Option<bool>) {
    let loop_val = loop_.unwrap_or(false);
    let resume_val = resume.unwrap_or(false);
    pyxel::pyxel().play_sound(ch, sound, sec, loop_val, resume_val);
}

/// Play music on a channel
/// 
/// # Arguments
/// * `msc` - Music index
/// * `sec` - Duration in seconds (None = full music)
/// * `loop_` - Whether to loop (default: false)
pub fn playm(msc: u32, sec: Option<f32>, loop_: Option<bool>) {
    let loop_val = loop_.unwrap_or(false);
    pyxel::pyxel().play_music(msc, sec, loop_val);
}

/// Stop playback on a channel
pub fn stop(ch: Option<u32>) {
    match ch {
        Some(ch) => pyxel::pyxel().stop_channel(ch),
        None => pyxel::pyxel().stop_all_channels(),
    }
}

/// Check if audio is playing on a channel
pub fn is_playing(_ch: u32) -> bool {
    // Check if channel is actively playing
    // Using a basic implementation - may need refinement
    true // Placeholder: would need proper state tracking
}
