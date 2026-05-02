/// System API - Core game loop and initialization

// ---------------------------------------------------------------------------
// PyxelCoreBackend
// ---------------------------------------------------------------------------
#[cfg(feature = "pyxel-core-backend")]
mod pyxel_core_impl {
    use pyxel::{Pyxel, PyxelCallback};
    use std::sync::{Arc, Mutex};

    pub fn init(width: u32, height: u32, title: &str, fps: u32) {
        pyxel::init(width, height, Some(title), Some(fps), None, None, None, None, None);
    }

    pub fn run(update: impl FnMut() + 'static, draw: impl FnMut() + 'static) {
        struct GameCallback {
            update: Arc<Mutex<Box<dyn FnMut()>>>,
            draw: Arc<Mutex<Box<dyn FnMut()>>>,
        }
        impl PyxelCallback for GameCallback {
            fn update(&mut self, _: &mut Pyxel) {
                if let Ok(mut f) = self.update.try_lock() { f(); }
            }
            fn draw(&mut self, _: &mut Pyxel) {
                if let Ok(mut f) = self.draw.try_lock() { f(); }
            }
        }
        let callback = GameCallback {
            update: Arc::new(Mutex::new(Box::new(update))),
            draw: Arc::new(Mutex::new(Box::new(draw))),
        };
        pyxel::pyxel().run(callback);
    }

    pub fn quit() { pyxel::pyxel().quit(); }
    pub fn width() -> u32 { *pyxel::width() }
    pub fn height() -> u32 { *pyxel::height() }
    pub fn frame_count() -> u32 { *pyxel::frame_count() }
}

// ---------------------------------------------------------------------------
// WasmBackend
// ---------------------------------------------------------------------------
#[cfg(feature = "wasm-backend")]
mod wasm_impl {
    use crate::backend::wasm_backend;

    pub fn init(width: u32, height: u32, title: &str, fps: u32) {
        wasm_backend::init(width, height, title, fps);
    }
    pub fn run(update: impl FnMut() + 'static, draw: impl FnMut() + 'static) {
        wasm_backend::run(Box::new(update), Box::new(draw));
    }
    pub fn quit() { wasm_backend::state().should_quit = true; }
    pub fn width() -> u32 { wasm_backend::state().width }
    pub fn height() -> u32 { wasm_backend::state().height }
    pub fn frame_count() -> u32 { wasm_backend::state().frame_count }
}

// ---------------------------------------------------------------------------
// WgpuBackend
// ---------------------------------------------------------------------------
#[cfg(feature = "wgpu-backend")]
mod wgpu_impl {
    use crate::backend::wgpu_backend;

    pub fn init(width: u32, height: u32, title: &str, fps: u32) {
        wgpu_backend::init(width, height, title, fps);
    }
    pub fn run(update: impl FnMut() + 'static, draw: impl FnMut() + 'static) {
        wgpu_backend::run(Box::new(update), Box::new(draw));
    }
    pub fn quit() { wgpu_backend::state().should_quit = true; }
    pub fn width() -> u32 { wgpu_backend::state().width }
    pub fn height() -> u32 { wgpu_backend::state().height }
    pub fn frame_count() -> u32 { wgpu_backend::state().frame_count }
}

// ---------------------------------------------------------------------------
// Public API — dispatches to the active backend
// ---------------------------------------------------------------------------

pub fn init(width: u32, height: u32, title: &str, fps: u32) {
    #[cfg(feature = "wasm-backend")]        wasm_impl::init(width, height, title, fps);
    #[cfg(feature = "wgpu-backend")]        wgpu_impl::init(width, height, title, fps);
    #[cfg(feature = "pyxel-core-backend")] pyxel_core_impl::init(width, height, title, fps);
}

pub fn run(update: impl FnMut() + 'static, draw: impl FnMut() + 'static) {
    #[cfg(feature = "wasm-backend")]        wasm_impl::run(update, draw);
    #[cfg(feature = "wgpu-backend")]        wgpu_impl::run(update, draw);
    #[cfg(feature = "pyxel-core-backend")] pyxel_core_impl::run(update, draw);
}

pub fn quit() {
    #[cfg(feature = "wasm-backend")]        wasm_impl::quit();
    #[cfg(feature = "wgpu-backend")]        wgpu_impl::quit();
    #[cfg(feature = "pyxel-core-backend")] pyxel_core_impl::quit();
}

pub fn width() -> u32 {
    #[cfg(feature = "wasm-backend")]        { return wasm_impl::width(); }
    #[cfg(feature = "wgpu-backend")]        { return wgpu_impl::width(); }
    #[cfg(feature = "pyxel-core-backend")] { return pyxel_core_impl::width(); }
    #[allow(unreachable_code)] 0
}

pub fn height() -> u32 {
    #[cfg(feature = "wasm-backend")]        { return wasm_impl::height(); }
    #[cfg(feature = "wgpu-backend")]        { return wgpu_impl::height(); }
    #[cfg(feature = "pyxel-core-backend")] { return pyxel_core_impl::height(); }
    #[allow(unreachable_code)] 0
}

pub fn frame_count() -> u32 {
    #[cfg(feature = "wasm-backend")]        { return wasm_impl::frame_count(); }
    #[cfg(feature = "wgpu-backend")]        { return wgpu_impl::frame_count(); }
    #[cfg(feature = "pyxel-core-backend")] { return pyxel_core_impl::frame_count(); }
    #[allow(unreachable_code)] 0
}
