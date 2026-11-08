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
        #[cfg(any(feature = "wgpu-backend", feature = "wasm-backend"))]
        {
            $soft
        }
        #[cfg(feature = "pyxel-core-backend")]
        {
            $p
        }
    }};
}

pub fn cls(col: Color) {
    dispatch!(soft: s().clear(col), pyxel: pyxel::pyxel().clear(col));
}
pub fn pset(x: f32, y: f32, col: Color) {
    dispatch!(soft: s().pset(x, y, col), pyxel: pyxel::pyxel().set_pixel(x, y, col));
}
pub fn pget(x: f32, y: f32) -> Color {
    dispatch!(soft: s().pget(x, y), pyxel: pyxel::pyxel().pixel(x, y))
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
        soft:  { let st = s(); st.clip_rect = Some([x as i32, y as i32, w as i32, h as i32]); #[cfg(feature="wasm-backend")] st.screen.borrow_mut().set_clip_rect(x,y,w,h); },
        pyxel: pyxel::pyxel().set_clip_rect(x,y,w,h)
    );
}
pub fn clip_reset() {
    dispatch!(soft: { s().clip_rect = None; #[cfg(feature="wasm-backend")] s().screen.borrow_mut().reset_clip_rect(); }, pyxel: pyxel::pyxel().reset_clip_rect());
}
pub fn camera(x: f32, y: f32) {
    dispatch!(soft: { let st = s(); st.camera_x = x; st.camera_y = y; #[cfg(feature="wasm-backend")] st.screen.borrow_mut().set_camera(x,y); }, pyxel: pyxel::pyxel().set_camera(x, y));
}
pub fn camera_reset() {
    dispatch!(soft: { let st = s(); st.camera_x = 0.0; st.camera_y = 0.0; #[cfg(feature="wasm-backend")] st.screen.borrow_mut().reset_camera(); }, pyxel: pyxel::pyxel().reset_camera());
}
pub fn pal(col1: Color, col2: Color) {
    dispatch!(
        soft:  { let st=s();let len=st.color_map.len();st.color_map[col1 as usize %len]=(col2 as usize %len) as u8; #[cfg(feature="wasm-backend")] st.screen.borrow_mut().map_color(col1,col2); },
        pyxel: pyxel::pyxel().map_color(col1, col2)
    );
}
pub fn pal_reset() {
    dispatch!(
        soft: {let st=s();for i in 0..st.color_map.len() {st.color_map[i]=i as u8;} #[cfg(feature="wasm-backend")] st.screen.borrow_mut().reset_color_map();},
        pyxel: pyxel::pyxel().reset_color_map()
    );
}
pub fn dither(alpha: f32) {
    dispatch!(soft: { s().dither_alpha = alpha; #[cfg(feature="wasm-backend")] s().screen.borrow_mut().set_dithering(alpha); }, pyxel: pyxel::pyxel().set_dithering(alpha));
}

/// Set palette color: rgb is 0xRRGGBB
pub fn set_palette_color(idx: u8, rgb: u32) {
    dispatch!(
        soft: {
            #[cfg(feature="wasm-backend")]
            {let mut colors=pyxel::colors();let len=colors.len().max(idx as usize+1);colors.resize(len,0);colors[idx as usize]=rgb & 0xffffff;}
            #[cfg(feature="wgpu-backend")]
            {let i=(idx & 15) as usize;s().palette[i]=[((rgb>>16)&255) as u8,((rgb>>8)&255) as u8,(rgb&255) as u8,255];}
        },
        pyxel: { let colors=&mut *pyxel::colors(); if let Some(color)=colors.get_mut(idx as usize) {*color=rgb & 0xffffff;} }
    );
}
pub fn get_palette_color(idx: u8) -> u32 {
    dispatch!(
        soft: {
            #[cfg(feature="wasm-backend")]
            {pyxel::colors().get(idx as usize).copied().unwrap_or(0)}
            #[cfg(feature="wgpu-backend")]
            {let [r,g,b,_]=s().palette[(idx&15) as usize];((r as u32)<<16)|((g as u32)<<8)|b as u32}
        },
        pyxel: pyxel::colors().get(idx as usize).copied().unwrap_or(0)
    )
}

/// Image bank: draw filled rect into offscreen image buffer
pub fn image_rect(bank: u32, x: f32, y: f32, w: f32, h: f32, col: u8) {
    dispatch!(soft: s().image_rect(bank, x, y, w, h, col), pyxel: { if let Some(i)=pyxel::images().get(bank as usize) {i.borrow_mut().draw_rect(x,y,w,h,col);} });
}
/// Image bank: set single pixel
pub fn image_pset(bank: u32, x: f32, y: f32, col: u8) {
    dispatch!(soft: s().image_pset(bank, x, y, col), pyxel: { if let Some(i)=pyxel::images().get(bank as usize) {i.borrow_mut().set_pixel(x,y,col);} });
}
/// Image bank: draw line
pub fn image_line(bank: u32, x1: f32, y1: f32, x2: f32, y2: f32, col: u8) {
    dispatch!(soft: s().image_line(bank, x1, y1, x2, y2, col), pyxel: { if let Some(i)=pyxel::images().get(bank as usize) {i.borrow_mut().draw_line(x1,y1,x2,y2,col);} });
}
/// Image bank: draw filled triangle
pub fn image_tri(bank: u32, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, col: u8) {
    dispatch!(soft: s().image_tri(bank, x1, y1, x2, y2, x3, y3, col), pyxel: { if let Some(i)=pyxel::images().get(bank as usize) {i.borrow_mut().draw_triangle(x1,y1,x2,y2,x3,y3,col);} });
}
/// Tilemap: clear all tiles
pub fn tilemap_cls(tm: u32, tile: (u8, u8)) {
    dispatch!(soft: s().tilemap_cls(tm, tile), pyxel: { if let Some(m)=pyxel::tilemaps().get(tm as usize) {m.borrow_mut().clear((tile.0.into(),tile.1.into()));} });
}
/// Tilemap: get tile at (x, y)
pub fn tilemap_pget(tm: u32, x: u32, y: u32) -> (u8, u8) {
    dispatch!(soft: s().tilemap_pget(tm, x, y), pyxel: { pyxel::tilemaps().get(tm as usize).map_or((0,0),|m| {let t=m.borrow().tile(x as f32,y as f32);(t.0 as u8,t.1 as u8)}) })
}
/// Tilemap: set tile at (x, y)
pub fn tilemap_pset(tm: u32, x: u32, y: u32, tile: (u8, u8)) {
    dispatch!(soft: s().tilemap_pset(tm, x, y, tile), pyxel: { if let Some(m)=pyxel::tilemaps().get(tm as usize) {m.borrow_mut().set_tile(x as f32,y as f32,(tile.0.into(),tile.1.into()));} });
}

/// Official outline spellings. Legacy rect/circ remain outlines for existing Rust games.
pub fn rectb(x: f32, y: f32, w: f32, h: f32, col: Color) {
    rect(x, y, w, h, col);
}
pub fn circb(x: f32, y: f32, r: f32, col: Color) {
    circ(x, y, r, col);
}

#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn elli(x: f32, y: f32, w: f32, h: f32, col: Color) {
    #[cfg(feature = "wasm-backend")]
    s().raster_core(|image| image.draw_ellipse(x, y, w, h, col));
    #[cfg(feature = "pyxel-core-backend")]
    pyxel::pyxel().draw_ellipse(x, y, w, h, col);
}

#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn ellib(x: f32, y: f32, w: f32, h: f32, col: Color) {
    #[cfg(feature = "wasm-backend")]
    s().raster_core(|image| image.draw_ellipse_border(x, y, w, h, col));
    #[cfg(feature = "pyxel-core-backend")]
    pyxel::pyxel().draw_ellipse_border(x, y, w, h, col);
}

#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn tri(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, col: Color) {
    #[cfg(feature = "wasm-backend")]
    s().raster_core(|image| image.draw_triangle(x1, y1, x2, y2, x3, y3, col));
    #[cfg(feature = "pyxel-core-backend")]
    pyxel::pyxel().draw_triangle(x1, y1, x2, y2, x3, y3, col);
}

#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn trib(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, col: Color) {
    #[cfg(feature = "wasm-backend")]
    s().raster_core(|image| image.draw_triangle_border(x1, y1, x2, y2, x3, y3, col));
    #[cfg(feature = "pyxel-core-backend")]
    pyxel::pyxel().draw_triangle_border(x1, y1, x2, y2, x3, y3, col);
}

#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn fill(x: f32, y: f32, col: Color) {
    #[cfg(feature = "wasm-backend")]
    s().raster_core(|image| image.flood_fill(x, y, col));
    #[cfg(feature = "pyxel-core-backend")]
    pyxel::pyxel().flood_fill(x, y, col);
}

#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub fn text_with_font(x: f32, y: f32, msg: &str, col: Color, font: Option<&crate::RcFont>) {
    #[cfg(feature = "wasm-backend")]
    s().raster_core(|image| image.draw_text(x, y, msg, col, font));
    #[cfg(feature = "pyxel-core-backend")]
    pyxel::pyxel().draw_text(x, y, msg, col, font);
}
#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub fn blt_transform(
    x: f32,
    y: f32,
    img: u32,
    sx: f32,
    sy: f32,
    w: f32,
    h: f32,
    colkey: Option<Color>,
    rotate: Option<f32>,
    scale: Option<f32>,
) {
    #[cfg(feature = "wasm-backend")]
    if let Some(source) = pyxel::images().get(img as usize).cloned() {
        s().raster_core(|image| {
            image.draw_image(x, y, &source, sx, sy, w, h, colkey, rotate, scale)
        });
    }
    #[cfg(feature = "pyxel-core-backend")]
    pyxel::pyxel().draw_image(x, y, img, sx, sy, w, h, colkey, rotate, scale);
}
#[cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
pub fn bltm_transform(
    x: f32,
    y: f32,
    tm: u32,
    sx: f32,
    sy: f32,
    w: f32,
    h: f32,
    colkey: Option<Color>,
    rotate: Option<f32>,
    scale: Option<f32>,
) {
    #[cfg(feature = "wasm-backend")]
    if let Some(source) = pyxel::tilemaps().get(tm as usize).cloned() {
        s().raster_core(|image| {
            image.draw_tilemap(x, y, &source, sx, sy, w, h, colkey, rotate, scale)
        });
    }
    #[cfg(feature = "pyxel-core-backend")]
    pyxel::pyxel().draw_tilemap(x, y, tm, sx, sy, w, h, colkey, rotate, scale);
}

/// Perspective projection of an image plane using the official rasterizer.
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn blt3d(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    img: u32,
    pos: (f32, f32, f32),
    rot: (f32, f32, f32),
    fov: Option<f32>,
    colkey: Option<Color>,
) -> Result<(), String> {
    let image = super::resource::images()
        .get(img as usize)
        .cloned()
        .ok_or("Invalid image bank")?;
    super::resource::screen()
        .borrow_mut()
        .draw_image_3d(x, y, w, h, &image, pos, rot, fov, colkey);
    Ok(())
}
/// Perspective projection of a tilemap plane using the official rasterizer.
#[cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
pub fn bltm3d(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    tm: u32,
    pos: (f32, f32, f32),
    rot: (f32, f32, f32),
    fov: Option<f32>,
    colkey: Option<Color>,
) -> Result<(), String> {
    let map = super::resource::tilemaps()
        .get(tm as usize)
        .cloned()
        .ok_or("Invalid tilemap bank")?;
    super::resource::screen()
        .borrow_mut()
        .draw_tilemap_3d(x, y, w, h, &map, pos, rot, fov, colkey);
    Ok(())
}
