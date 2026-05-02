/// Graphics API - Drawing primitives
/// 
/// Matches Python Pyxel interface using floats for coordinates.

use pyxel::Color;

/// Clear screen with specified color
pub fn cls(col: Color) {
    pyxel::pyxel().clear(col);
}

/// Set clipping rectangle (all coordinates/sizes as pixels)
/// Call with no parameters to reset clipping
pub fn clip(x: f32, y: f32, w: f32, h: f32) {
    pyxel::pyxel().set_clip_rect(x, y, w, h);
}

/// Reset clipping to full screen
pub fn clip_reset() {
    pyxel::pyxel().reset_clip_rect();
}

/// Set camera position (not available in this version of pyxel-core)
pub fn camera(_x: f32, _y: f32) {
    // TODO: Camera functionality not yet implemented in pyxel-core
}

/// Reset camera to origin (not available in this version of pyxel-core)
pub fn camera_reset() {
    // TODO: Camera functionality not yet implemented in pyxel-core
}

/// Map color: when src_color is drawn, show dst_color instead
pub fn pal(col1: Color, col2: Color) {
    pyxel::pyxel().map_color(col1, col2);
}

/// Reset all color mappings
pub fn pal_reset() {
    pyxel::pyxel().reset_color_map();
}

/// Set dithering alpha (0.0-1.0)
pub fn dither(alpha: f32) {
    pyxel::pyxel().set_dithering(alpha);
}

// Primitive drawing

/// Get pixel color at (x, y)
pub fn pget(_x: f32, _y: f32) -> Color {
    // Note: pyxel-core doesn't expose a direct get_pixel method on graphics
    // Return a default for now - this may need integration at the image level
    0
}

/// Set pixel at (x, y) to color
pub fn pset(x: f32, y: f32, col: Color) {
    pyxel::pyxel().set_pixel(x, y, col);
}

/// Draw line from (x1, y1) to (x2, y2)
pub fn line(x1: f32, y1: f32, x2: f32, y2: f32, col: Color) {
    pyxel::pyxel().draw_line(x1, y1, x2, y2, col);
}

/// Draw rectangle outline
pub fn rect(x: f32, y: f32, w: f32, h: f32, col: Color) {
    pyxel::pyxel().draw_rect_border(x, y, w, h, col);
}

/// Draw filled rectangle
pub fn rectfill(x: f32, y: f32, w: f32, h: f32, col: Color) {
    pyxel::pyxel().draw_rect(x, y, w, h, col);
}

/// Draw circle outline
pub fn circ(x: f32, y: f32, r: f32, col: Color) {
    pyxel::pyxel().draw_circle_border(x, y, r, col);
}

/// Draw filled circle
pub fn circfill(x: f32, y: f32, r: f32, col: Color) {
    pyxel::pyxel().draw_circle(x, y, r, col);
}

/// Draw text
pub fn text(x: f32, y: f32, s: &str, col: Color) {
    pyxel::pyxel().draw_text(x, y, s, col, None);
}

/// Draw sprite/image
pub fn blt(x: f32, y: f32, img: u32, sx: f32, sy: f32, sw: f32, sh: f32, colkey: Option<Color>) {
    pyxel::pyxel().draw_image(x, y, img, sx, sy, sw, sh, colkey, None, None);
}

/// Draw tilemap
pub fn bltm(x: f32, y: f32, tm: u32, mx: f32, my: f32, mw: f32, mh: f32, colkey: Option<Color>) {
    pyxel::pyxel().draw_tilemap(x, y, tm, mx, my, mw, mh, colkey, None, None);
}

/// Polygon drawing - helper for advanced graphics
pub fn poly(points: &[(f32, f32)], col: Color) {
    if points.is_empty() {
        return;
    }
    for i in 0..points.len() {
        let next = (i + 1) % points.len();
        line(points[i].0, points[i].1, points[next].0, points[next].1, col);
    }
}

/// Filled polygon - helper for advanced graphics
pub fn polyfill(points: &[(f32, f32)], col: Color) {
    if points.len() < 3 {
        return;
    }
    // Simple triangle fan fill
    for i in 1..points.len() - 1 {
        pyxel::pyxel().draw_triangle(
            points[0].0,
            points[0].1,
            points[i].0,
            points[i].1,
            points[i + 1].0,
            points[i + 1].1,
            col,
        );
    }
}
