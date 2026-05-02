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
