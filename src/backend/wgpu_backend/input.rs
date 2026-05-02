/// Input state for WgpuBackend
///
/// Tracks per-frame key state: held, just-pressed, just-released.
/// Keycodes are stored as u32 (VirtualKeyCode as u32).

const KEY_COUNT: usize = 256;

pub struct InputState {
    /// Key held this frame
    held: [bool; KEY_COUNT],
    /// Key pressed this frame (one shot)
    pressed: [bool; KEY_COUNT],
    /// Key released this frame (one shot)
    released: [bool; KEY_COUNT],

    pub mouse_x: i32,
    pub mouse_y: i32,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            held: [false; KEY_COUNT],
            pressed: [false; KEY_COUNT],
            released: [false; KEY_COUNT],
            mouse_x: 0,
            mouse_y: 0,
        }
    }

    /// Called each frame to clear one-shot flags
    pub fn tick(&mut self) {
        self.pressed = [false; KEY_COUNT];
        self.released = [false; KEY_COUNT];
    }

    pub fn set_key(&mut self, idx: u32, is_down: bool) {
        let idx = idx as usize;
        if idx >= KEY_COUNT {
            return;
        }
        if is_down && !self.held[idx] {
            self.pressed[idx] = true;
        }
        if !is_down && self.held[idx] {
            self.released[idx] = true;
        }
        self.held[idx] = is_down;
    }

    pub fn btn(&self, key: u32) -> bool {
        let idx = key as usize;
        if idx < KEY_COUNT { self.held[idx] } else { false }
    }

    pub fn btnp(&self, key: u32) -> bool {
        let idx = key as usize;
        if idx < KEY_COUNT { self.pressed[idx] } else { false }
    }

    pub fn btnr(&self, key: u32) -> bool {
        let idx = key as usize;
        if idx < KEY_COUNT { self.released[idx] } else { false }
    }
}
