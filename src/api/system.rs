/// System API - Core game loop and initialization

// ---------------------------------------------------------------------------
// PyxelCoreBackend
// ---------------------------------------------------------------------------
#[cfg(feature = "pyxel-core-backend")]
mod pyxel_core_impl {
    use pyxel::{Pyxel, PyxelCallback};

    pub fn init(width: u32, height: u32, title: &str, fps: u32) {
        pyxel::init(
            width,
            height,
            Some(title),
            Some(fps),
            None,
            None,
            None,
            None,
            None,
        )
        .expect("Failed to initialize Pyxel");
    }

    pub fn run(update: impl FnMut() + 'static, draw: impl FnMut() + 'static) {
        struct GameCallback {
            update: Box<dyn FnMut()>,
            draw: Box<dyn FnMut()>,
        }
        impl PyxelCallback for GameCallback {
            fn update(&mut self) {
                (self.update)();
            }
            fn draw(&mut self) {
                (self.draw)();
            }
        }
        let callback = GameCallback {
            update: Box::new(update),
            draw: Box::new(draw),
        };
        Pyxel::run(callback);
    }

    pub fn quit() {
        Pyxel::quit();
    }
    pub fn width() -> u32 {
        *pyxel::width()
    }
    pub fn height() -> u32 {
        *pyxel::height()
    }
    pub fn frame_count() -> u32 {
        *pyxel::frame_count()
    }
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
    pub fn quit() {
        crate::api::audio::stop(None);
        wasm_backend::state().should_quit = true;
    }
    pub fn width() -> u32 {
        wasm_backend::state().width
    }
    pub fn height() -> u32 {
        wasm_backend::state().height
    }
    pub fn frame_count() -> u32 {
        wasm_backend::state().frame_count
    }
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
    pub fn quit() {
        wgpu_backend::state().should_quit = true;
    }
    pub fn width() -> u32 {
        wgpu_backend::state().width
    }
    pub fn height() -> u32 {
        wgpu_backend::state().height
    }
    pub fn frame_count() -> u32 {
        wgpu_backend::state().frame_count
    }
}

// ---------------------------------------------------------------------------
// Public API — dispatches to the active backend
// ---------------------------------------------------------------------------

/// Extended official initialization settings. `init` remains the small convenience
/// entry point; this fallible API validates settings before replacing browser state.
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
#[derive(Clone, Debug)]
pub struct InitOptions {
    pub quit_key: u32,
    pub display_scale: Option<u32>,
    pub capture_scale: u32,
    pub capture_sec: u32,
    pub headless: bool,
}
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
impl Default for InitOptions {
    fn default() -> Self {
        Self {
            quit_key: super::KEY_ESCAPE,
            display_scale: None,
            capture_scale: 2,
            capture_sec: 10,
            headless: false,
        }
    }
}
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn init_with_options(
    width: u32,
    height: u32,
    title: &str,
    fps: u32,
    options: InitOptions,
) -> Result<(), String> {
    if width == 0
        || height == 0
        || fps == 0
        || u64::from(width) * u64::from(height) > 16 * 1024 * 1024
    {
        return Err("Dimensions and fps must be positive; screen must fit 16 Mi pixels".into());
    }
    if options.display_scale == Some(0) || options.capture_scale == 0 {
        return Err("Display and capture scales must be positive".into());
    }
    #[cfg(feature = "wasm-backend")]
    {
        crate::backend::wasm_backend::init_with_options(width, height, title, fps, options)
    }
    #[cfg(feature = "pyxel-core-backend")]
    {
        pyxel::init(
            width,
            height,
            Some(title),
            Some(fps),
            Some(options.quit_key),
            options.display_scale,
            Some(options.capture_scale),
            Some(options.capture_sec),
            Some(options.headless),
        )
    }
}

pub fn init(width: u32, height: u32, title: &str, fps: u32) {
    #[cfg(feature = "wasm-backend")]
    wasm_impl::init(width, height, title, fps);
    #[cfg(feature = "wgpu-backend")]
    wgpu_impl::init(width, height, title, fps);
    #[cfg(feature = "pyxel-core-backend")]
    pyxel_core_impl::init(width, height, title, fps);
}

pub fn run(update: impl FnMut() + 'static, draw: impl FnMut() + 'static) {
    #[cfg(feature = "wasm-backend")]
    wasm_impl::run(update, draw);
    #[cfg(feature = "wgpu-backend")]
    wgpu_impl::run(update, draw);
    #[cfg(feature = "pyxel-core-backend")]
    pyxel_core_impl::run(update, draw);
}

pub fn quit() {
    #[cfg(feature = "wasm-backend")]
    wasm_impl::quit();
    #[cfg(feature = "wgpu-backend")]
    wgpu_impl::quit();
    #[cfg(feature = "pyxel-core-backend")]
    pyxel_core_impl::quit();
}

pub fn width() -> u32 {
    #[cfg(feature = "wasm-backend")]
    {
        return wasm_impl::width();
    }
    #[cfg(feature = "wgpu-backend")]
    {
        return wgpu_impl::width();
    }
    #[cfg(feature = "pyxel-core-backend")]
    {
        return pyxel_core_impl::width();
    }
    #[allow(unreachable_code)]
    0
}

pub fn height() -> u32 {
    #[cfg(feature = "wasm-backend")]
    {
        return wasm_impl::height();
    }
    #[cfg(feature = "wgpu-backend")]
    {
        return wgpu_impl::height();
    }
    #[cfg(feature = "pyxel-core-backend")]
    {
        return pyxel_core_impl::height();
    }
    #[allow(unreachable_code)]
    0
}

pub fn frame_count() -> u32 {
    #[cfg(feature = "wasm-backend")]
    {
        return wasm_impl::frame_count();
    }
    #[cfg(feature = "wgpu-backend")]
    {
        return wgpu_impl::frame_count();
    }
    #[cfg(feature = "pyxel-core-backend")]
    {
        return pyxel_core_impl::frame_count();
    }
    #[allow(unreachable_code)]
    0
}

#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn title(text: &str) {
    #[cfg(feature = "wasm-backend")]
    crate::backend::wasm_backend::system::title(text);
    #[cfg(feature = "pyxel-core-backend")]
    pyxel::pyxel().set_title(text);
}
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn resize(width: u32, height: u32) -> Result<(), String> {
    #[cfg(feature = "wasm-backend")]
    {
        crate::backend::wasm_backend::system::resize(width, height)
    }
    #[cfg(feature = "pyxel-core-backend")]
    {
        pyxel::pyxel().set_screen_size(width, height)
    }
}
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn fullscreen(enabled: bool) {
    #[cfg(feature = "wasm-backend")]
    crate::backend::wasm_backend::system::fullscreen(enabled);
    #[cfg(feature = "pyxel-core-backend")]
    pyxel::pyxel().set_fullscreen(enabled);
}
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn integer_scale(enabled: bool) {
    #[cfg(feature = "wasm-backend")]
    crate::backend::wasm_backend::system::integer_scale(enabled);
    #[cfg(feature = "pyxel-core-backend")]
    pyxel::pyxel().set_integer_scale(enabled);
}
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn perf_monitor(enabled: bool) {
    #[cfg(feature = "wasm-backend")]
    crate::backend::wasm_backend::system::perf_monitor(enabled);
    #[cfg(feature = "pyxel-core-backend")]
    pyxel::pyxel().set_perf_monitor(enabled);
}
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn icon<S: AsRef<str>>(data: &[S], scale: u32, transparent: Option<u8>) -> Result<(), String> {
    #[cfg(feature = "wasm-backend")]
    {
        crate::backend::wasm_backend::system::icon(data, scale, transparent)
    }
    #[cfg(feature = "pyxel-core-backend")]
    {
        pyxel::pyxel().set_icon(data, scale, transparent)
    }
}

#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn show() {
    #[cfg(feature = "wasm-backend")]
    crate::backend::wasm_backend::system::browser_show();
    #[cfg(feature = "pyxel-core-backend")]
    pyxel::Pyxel::show_screen();
}
#[cfg(feature = "wasm-backend")]
pub async fn flip() -> Result<(), String> {
    crate::backend::wasm_backend::system::browser_flip()
        .await
        .map_err(|e| format!("{e:?}"))
}
#[cfg(feature = "pyxel-core-backend")]
pub fn flip() {
    pyxel::Pyxel::flip_screen();
}
#[cfg(feature = "pyxel-core-backend")]
pub fn set_reset_handler(handler: impl FnMut(Option<String>) + Send + 'static) {
    *pyxel::reset_callback() = Some(Box::new(handler));
}
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn reset() -> Result<(), String> {
    #[cfg(feature = "wasm-backend")]
    {
        crate::backend::wasm_backend::system::browser_reset().map_err(|e| format!("{e:?}"))
    }
    #[cfg(feature = "pyxel-core-backend")]
    {
        if pyxel::reset_callback().is_none() {
            return Err("Register set_reset_handler before reset".into());
        }
        pyxel::Pyxel::restart();
        Ok(())
    }
}
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn screen_mode(mode: u32) -> Result<(), String> {
    if mode > 2 {
        return Err("Screen mode must be 0..2".into());
    }
    #[cfg(feature = "wasm-backend")]
    {
        crate::backend::wasm_backend::system::browser_screen_mode(mode)
            .map_err(|e| format!("{e:?}"))
    }
    #[cfg(feature = "pyxel-core-backend")]
    {
        pyxel::pyxel().set_screen_mode(mode);
        Ok(())
    }
}
