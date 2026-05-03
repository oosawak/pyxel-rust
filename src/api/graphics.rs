/// Graphics API - Drawing primitives

use crate::Color;

#[cfg(feature = "wasm-backend")]
fn s() -> &'static mut crate::backend::wasm_backend::WasmState {
    crate::backend::wasm_backend::state()
}
#[cfg(feature = "wgpu-backend")]
fn s() -> &'static mut crate::backend::wgpu_backend::WgpuState {
    crate::backend::wgpu_backend::state()
}

macro_rules! dispatch {
    (soft: $soft:expr, pyxel: $p:expr) => {{
        #[cfg(any(feature = "wgpu-backend", feature = "wasm-backend"))] { $soft }
        #[cfg(feature = "pyxel-core-backend")] { $p }
        #[cfg(not(any(feature = "wgpu-backend", feature = "wasm-backend", feature = "pyxel-core-backend")))]
        { unreachable!("No backend feature enabled") }
    }};
}

pub fn cls(col: Color) {
    dispatch!(soft: s().clear(col), pyxel: pyxel::pyxel().clear(col));
}
pub fn pset(x: f32, y: f32, col: Color) {
    dispatch!(soft: s().pset(x, y, col), pyxel: pyxel::pyxel().set_pixel(x, y, col));
}
pub fn pget(x: f32, y: f32) -> Color {
    dispatch!(soft: s().pget(x, y), pyxel: 0)
}
pub fn line(x1: f32, y1: f32, x2: f32, y2: f32, col: Color) {
    dispatch!(soft: s().draw_line(x1,y1,x2,y2,col), pyxel: pyxel::pyxel().draw_line(x1,y1,x2,y2,col));
}
pub fn rect(x: f32, y: f32, w: f32, h: f32, col: Color) {
    dispatch!(soft: s().draw_rect_border(x,y,w,h,col), pyxel: pyxel::pyxel().draw_rect_border(x,y,w,h,col));
}
pub fn rectfill(x: f32, y: f32, w: f32, h: f32, col: Color) {
    dispatch!(soft: s().draw_rect(x,y,w,h,col), pyxel: pyxel::pyxel().draw_rect(x,y,w,h,col));
}
pub fn circ(x: f32, y: f32, r: f32, col: Color) {
    dispatch!(soft: s().draw_circle_border(x,y,r,col), pyxel: pyxel::pyxel().draw_circle_border(x,y,r,col));
}
pub fn circfill(x: f32, y: f32, r: f32, col: Color) {
    dispatch!(soft: s().draw_circle(x,y,r,col), pyxel: pyxel::pyxel().draw_circle(x,y,r,col));
}
pub fn text(x: f32, y: f32, msg: &str, col: Color) {
    dispatch!(soft: s().draw_text(x,y,msg,col), pyxel: pyxel::pyxel().draw_text(x,y,msg,col,None));
}
pub fn blt(x: f32, y: f32, img: u32, sx: f32, sy: f32, sw: f32, sh: f32, colkey: Option<Color>) {
    dispatch!(
        soft:  s().draw_image(x,y,img,sx,sy,sw,sh,colkey),
        pyxel: pyxel::pyxel().draw_image(x,y,img,sx,sy,sw,sh,colkey,None,None)
    );
}
pub fn bltm(x: f32, y: f32, tm: u32, mx: f32, my: f32, mw: f32, mh: f32, colkey: Option<Color>) {
    dispatch!(
        soft:  s().draw_tilemap(x,y,tm,mx,my,mw,mh,colkey),
        pyxel: pyxel::pyxel().draw_tilemap(x,y,tm,mx,my,mw,mh,colkey,None,None)
    );
}
pub fn clip(x: f32, y: f32, w: f32, h: f32) {
    dispatch!(
        soft:  { let st = s(); st.clip_rect = Some([x as i32, y as i32, w as i32, h as i32]); },
        pyxel: pyxel::pyxel().set_clip_rect(x,y,w,h)
    );
}
pub fn clip_reset() {
    dispatch!(soft: { s().clip_rect = None; }, pyxel: pyxel::pyxel().reset_clip_rect());
}
pub fn camera(x: f32, y: f32) {
    dispatch!(soft: { let st = s(); st.camera_x = x; st.camera_y = y; }, pyxel: { let _ = (x,y); });
}
pub fn camera_reset() {
    dispatch!(soft: { let st = s(); st.camera_x = 0.0; st.camera_y = 0.0; }, pyxel: {});
}
pub fn pal(col1: Color, col2: Color) {
    dispatch!(
        soft:  { s().color_map[col1 as usize & 0x0f] = col2 & 0x0f; },
        pyxel: pyxel::pyxel().map_color(col1, col2)
    );
}
pub fn pal_reset() {
    dispatch!(
        soft:  for i in 0..16u8 { s().color_map[i as usize] = i; },
        pyxel: pyxel::pyxel().reset_color_map()
    );
}
pub fn dither(alpha: f32) {
    dispatch!(soft: { s().dither_alpha = alpha; }, pyxel: pyxel::pyxel().set_dithering(alpha));
}

/// Set palette color: rgb is 0xRRGGBB
pub fn set_palette_color(idx: u8, rgb: u32) {
    dispatch!(
        soft: {
            let i = (idx & 0x0f) as usize;
            s().palette[i] = [
                ((rgb >> 16) & 0xff) as u8,
                ((rgb >> 8) & 0xff) as u8,
                (rgb & 0xff) as u8,
                0xff,
            ];
        },
        pyxel: { let _ = (idx, rgb); }
    );
}
pub fn get_palette_color(idx: u8) -> u32 {
    dispatch!(
        soft: {
            let i = (idx & 0x0f) as usize;
            let [r, g, b, _] = s().palette[i];
            ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
        },
        pyxel: { let _ = idx; 0 }
    )
}

/// Image bank: draw filled rect into offscreen image buffer
pub fn image_rect(bank: u32, x: f32, y: f32, w: f32, h: f32, col: u8) {
    dispatch!(soft: s().image_rect(bank, x, y, w, h, col), pyxel: { let _ = (bank,x,y,w,h,col); });
}
/// Image bank: set single pixel
pub fn image_pset(bank: u32, x: f32, y: f32, col: u8) {
    dispatch!(soft: s().image_pset(bank, x, y, col), pyxel: { let _ = (bank,x,y,col); });
}
/// Image bank: draw line
pub fn image_line(bank: u32, x1: f32, y1: f32, x2: f32, y2: f32, col: u8) {
    dispatch!(soft: s().image_line(bank, x1, y1, x2, y2, col), pyxel: { let _ = (bank,x1,y1,x2,y2,col); });
}
/// Image bank: draw filled triangle
pub fn image_tri(bank: u32, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, col: u8) {
    dispatch!(soft: s().image_tri(bank, x1, y1, x2, y2, x3, y3, col), pyxel: { let _ = (bank,x1,y1,x2,y2,x3,y3,col); });
}
/// Tilemap: clear all tiles
pub fn tilemap_cls(tm: u32, tile: (u8, u8)) {
    dispatch!(soft: s().tilemap_cls(tm, tile), pyxel: { let _ = (tm, tile); });
}
/// Tilemap: get tile at (x, y)
pub fn tilemap_pget(tm: u32, x: u32, y: u32) -> (u8, u8) {
    dispatch!(soft: s().tilemap_pget(tm, x, y), pyxel: { let _ = (tm,x,y); (0,0) })
}
/// Tilemap: set tile at (x, y)
pub fn tilemap_pset(tm: u32, x: u32, y: u32, tile: (u8, u8)) {
    dispatch!(soft: s().tilemap_pset(tm, x, y, tile), pyxel: { let _ = (tm,x,y,tile); });
}

/// Draw a frame from a registered sprite sheet.
/// (x, y) is the top-left in game coords. (col, row) selects the frame.
/// (dest_w, dest_h) is the destination size in game pixels.
/// Only functional in the wasm-backend; no-op on other backends.
pub fn blt_sp(x: f32, y: f32, slot: u32, col: u32, row: u32,
              dest_w: f32, dest_h: f32, flip_x: bool) {
    #[cfg(feature = "wasm-backend")]
    s().blt_sp(x, y, slot, col, row, dest_w, dest_h, flip_x);
    #[cfg(not(feature = "wasm-backend"))]
    let _ = (x, y, slot, col, row, dest_w, dest_h, flip_x);
}
