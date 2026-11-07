//! Frame-based software input shared by Canvas and wgpu.
use crate::api::constants::*;
use std::collections::BTreeMap;

#[derive(Default)]
struct Button {
    held: bool,
    pressed: bool,
    released: bool,
    age: u32,
}
#[derive(Default)]
pub struct InputState {
    buttons: BTreeMap<u32, Button>,
    values: BTreeMap<u32, i32>,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub mouse_visible: bool,
    pub input_keys: Vec<u32>,
}
impl InputState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn tick(&mut self) {
        self.buttons.retain(|_, b| {
            b.pressed = false;
            b.released = false;
            if b.held {
                b.age = b.age.saturating_add(1);
            }
            b.held
        });
        self.values.insert(MOUSE_WHEEL_X, 0);
        self.values.insert(MOUSE_WHEEL_Y, 0);
        self.input_keys.clear();
    }
    pub fn set_key(&mut self, key: u32, down: bool) {
        if key == KEY_UNKNOWN || key == KEY_NONE || key == 255 {
            return;
        }
        let button = self.buttons.entry(key).or_default();
        if down && !button.held {
            button.pressed = true;
            button.age = 0;
            let mouse = [
                MOUSE_BUTTON_LEFT,
                MOUSE_BUTTON_MIDDLE,
                MOUSE_BUTTON_RIGHT,
                MOUSE_BUTTON_X1,
                MOUSE_BUTTON_X2,
            ]
            .contains(&key);
            let gamepad = super::gamepad_keys::BUTTONS
                .iter()
                .flatten()
                .any(|k| *k == Some(key));
            if !mouse && !gamepad {
                self.input_keys.push(key);
            }
        }
        if !down && button.held {
            button.released = true;
        }
        button.held = down;
        for (virtual_key, left, right) in [
            (KEY_SHIFT, KEY_LSHIFT, KEY_RSHIFT),
            (KEY_CTRL, KEY_LCTRL, KEY_RCTRL),
            (KEY_ALT, KEY_LALT, KEY_RALT),
            (KEY_GUI, KEY_LGUI, KEY_RGUI),
        ] {
            if key == left || key == right {
                let held = [left, right]
                    .iter()
                    .any(|k| self.buttons.get(k).is_some_and(|b| b.held));
                self.set_key(virtual_key, held);
            }
        }
    }
    pub fn btn(&self, key: u32) -> bool {
        self.buttons.get(&key).is_some_and(|b| b.held || b.pressed)
    }
    pub fn btnp(&self, key: u32) -> bool {
        self.btnp_hold(key, 0, 0)
    }
    pub fn btnp_hold(&self, key: u32, hold: u32, repeat: u32) -> bool {
        self.buttons.get(&key).is_some_and(|b| {
            b.pressed
                || (b.held && repeat > 0 && b.age >= hold && (b.age - hold).is_multiple_of(repeat))
        })
    }
    pub fn btnr(&self, key: u32) -> bool {
        self.buttons.get(&key).is_some_and(|b| b.released)
    }
    pub fn value(&self, key: u32) -> i32 {
        match key {
            MOUSE_POS_X => self.mouse_x,
            MOUSE_POS_Y => self.mouse_y,
            _ => self.values.get(&key).copied().unwrap_or(0),
        }
    }
    pub fn set_value(&mut self, key: u32, value: i32) {
        match key {
            MOUSE_POS_X => self.mouse_x = value,
            MOUSE_POS_Y => self.mouse_y = value,
            _ => {
                self.values.insert(key, value);
            }
        }
    }
    pub fn release_all(&mut self) {
        for button in self.buttons.values_mut() {
            if button.held {
                button.released = true;
            }
            button.held = false;
            button.pressed = false;
        }
        for axes in super::gamepad_keys::AXES {
            for key in axes {
                self.set_value(key, 0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeat_uses_game_frames_and_ignores_duplicate_keydown() {
        let mut i = InputState::new();
        i.set_key(KEY_A, true);
        assert!(i.btnp_hold(KEY_A, 3, 2));
        for frame in 1..9 {
            i.tick();
            i.set_key(KEY_A, true);
            assert_eq!(i.btnp_hold(KEY_A, 3, 2), frame >= 3 && (frame - 3) % 2 == 0);
            assert!(!i.btnp(KEY_A));
        }
        i.set_key(KEY_A, false);
        assert!(i.btnr(KEY_A));
        assert!(!i.btn(KEY_A));
        i.tick();
        assert!(!i.btnr(KEY_A));
    }
    #[test]
    fn switching_modifier_hands_does_not_retrigger_or_release_the_group() {
        let mut i = InputState::new();
        i.set_key(KEY_LSHIFT, true);
        i.tick();
        i.set_key(KEY_RSHIFT, true);
        i.set_key(KEY_LSHIFT, false);
        assert!(i.btn(KEY_SHIFT));
        assert!(!i.btnp(KEY_SHIFT));
        assert!(!i.btnr(KEY_SHIFT));
        i.set_key(KEY_RSHIFT, false);
        assert!(i.btnr(KEY_SHIFT));
        i.tick();
        i.set_key(MOUSE_BUTTON_LEFT, true);
        i.set_key(GAMEPAD1_BUTTON_A, true);
        assert!(i.input_keys.is_empty());
    }
    #[test]
    fn quick_tap_modifiers_focus_loss_and_analog_values() {
        let mut i = InputState::new();
        i.set_key(KEY_A, true);
        i.set_key(KEY_A, false);
        assert!(i.btn(KEY_A) && i.btnp(KEY_A) && i.btnr(KEY_A));
        i.tick();
        assert!(!i.btn(KEY_A));
        i.set_key(KEY_LSHIFT, true);
        assert!(i.btn(KEY_SHIFT));
        i.tick();
        i.set_key(KEY_RSHIFT, true);
        i.set_key(KEY_LSHIFT, false);
        assert!(i.btn(KEY_SHIFT));
        i.set_value(MOUSE_POS_X, -4);
        i.set_value(MOUSE_WHEEL_Y, 3);
        i.set_value(GAMEPAD4_AXIS_LEFTX, -32768);
        i.tick();
        assert_eq!(i.value(MOUSE_WHEEL_Y), 0);
        assert_eq!(i.value(MOUSE_POS_X), -4);
        assert_eq!(i.value(GAMEPAD4_AXIS_LEFTX), -32768);
        i.release_all();
        assert!(!i.btn(KEY_SHIFT));
        assert!(i.btnr(KEY_RSHIFT));
        assert_eq!(i.value(GAMEPAD4_AXIS_LEFTX), 0);
        i.set_key(KEY_UNKNOWN, true);
        assert!(!i.btn(KEY_UNKNOWN));
    }
}
