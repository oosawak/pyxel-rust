/// System API - Core game loop and initialization
/// 
/// Matches Python Pyxel interface with simplified signatures for Rust.

use pyxel::{Pyxel, PyxelCallback};
use std::sync::{Arc, Mutex};

/// Initialize Pyxel engine
/// 
/// # Arguments
/// * `width` - Screen width in pixels
/// * `height` - Screen height in pixels
/// * `title` - Window title
/// * `fps` - Frames per second (default: 60)
pub fn init(width: u32, height: u32, title: &str, fps: u32) {
    pyxel::init(
        width,
        height,
        Some(title),
        Some(fps),
        None, // quit_key
        None, // display_scale
        None, // capture_scale
        None, // capture_sec
        None, // headless
    );
}

/// Run the game loop with update and draw callbacks
/// 
/// # Arguments
/// * `update` - Update callback function (called every frame)
/// * `draw` - Draw callback function (called every frame after update)
pub fn run(update: impl FnMut() + 'static, draw: impl FnMut() + 'static) {
    struct GameCallback {
        update: Arc<Mutex<Box<dyn FnMut()>>>,
        draw: Arc<Mutex<Box<dyn FnMut()>>>,
    }

    impl PyxelCallback for GameCallback {
        fn update(&mut self, _pyxel: &mut Pyxel) {
            if let Ok(mut f) = self.update.try_lock() {
                f();
            }
        }

        fn draw(&mut self, _pyxel: &mut Pyxel) {
            if let Ok(mut f) = self.draw.try_lock() {
                f();
            }
        }
    }

    let update = Arc::new(Mutex::new(Box::new(update) as Box<dyn FnMut()>));
    let draw = Arc::new(Mutex::new(Box::new(draw) as Box<dyn FnMut()>));

    let callback = GameCallback {
        update: Arc::clone(&update),
        draw: Arc::clone(&draw),
    };

    pyxel::pyxel().run(callback);
}

/// Quit the game
pub fn quit() {
    pyxel::pyxel().quit();
}

/// Get screen width
pub fn width() -> u32 {
    *pyxel::width()
}

/// Get screen height
pub fn height() -> u32 {
    *pyxel::height()
}

/// Get current frame count
pub fn frame_count() -> u32 {
    *pyxel::frame_count()
}
