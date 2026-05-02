/// Software drawing primitives for WgpuBackend
///
/// All operations write color indices (u8) into the pixel buffer.
/// Coordinates are in screen pixels (after camera offset).

use super::WgpuState;

impl WgpuState {
    // ------------------------------------------------------------------
    // Internal helpers
    // ------------------------------------------------------------------

    fn plot(&mut self, x: i32, y: i32, col: u8) {
        // Apply clip rect
        if let Some([cx, cy, cw, ch]) = self.clip_rect {
            if x < cx || y < cy || x >= cx + cw || y >= cy + ch {
                return;
            }
        }
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        // Dithering: skip alternate pixels when alpha < 1
        if self.dither_alpha < 1.0 {
            let threshold = (x + y) & 1;
            if self.dither_alpha <= 0.5 && threshold == 0 {
                return;
            }
        }
        let idx = (y as u32 * self.width + x as u32) as usize;
        self.pixel_buffer[idx] = col & 0x0f;
    }

    fn cam(&self, x: f32, y: f32) -> (i32, i32) {
        ((x - self.camera_x) as i32, (y - self.camera_y) as i32)
    }

    // ------------------------------------------------------------------
    // Public drawing API
    // ------------------------------------------------------------------

    pub fn clear(&mut self, col: u8) {
        let c = col & 0x0f;
        for px in self.pixel_buffer.iter_mut() {
            *px = c;
        }
    }

    pub fn pset(&mut self, x: f32, y: f32, col: u8) {
        let (px, py) = self.cam(x, y);
        self.plot(px, py, col);
    }

    pub fn pget(&self, x: f32, y: f32) -> u8 {
        let px = (x - self.camera_x) as i32;
        let py = (y - self.camera_y) as i32;
        if px < 0 || py < 0 || px >= self.width as i32 || py >= self.height as i32 {
            return 0;
        }
        self.pixel_buffer[(py as u32 * self.width + px as u32) as usize]
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, col: u8) {
        let (mut x0, mut y0) = self.cam(x1, y1);
        let (x1, y1) = self.cam(x2, y2);
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx: i32 = if x0 < x1 { 1 } else { -1 };
        let sy: i32 = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            self.plot(x0, y0, col);
            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; x0 += sx; }
            if e2 <= dx { err += dx; y0 += sy; }
        }
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, col: u8) {
        let (px, py) = self.cam(x, y);
        let pw = w as i32;
        let ph = h as i32;
        for row in 0..ph {
            for col_i in 0..pw {
                self.plot(px + col_i, py + row, col);
            }
        }
    }

    pub fn draw_rect_border(&mut self, x: f32, y: f32, w: f32, h: f32, col: u8) {
        let (px, py) = self.cam(x, y);
        let pw = w as i32;
        let ph = h as i32;
        for i in 0..pw {
            self.plot(px + i, py, col);
            self.plot(px + i, py + ph - 1, col);
        }
        for i in 0..ph {
            self.plot(px, py + i, col);
            self.plot(px + pw - 1, py + i, col);
        }
    }

    pub fn draw_circle(&mut self, cx: f32, cy: f32, r: f32, col: u8) {
        // Filled circle via scanline
        let (ocx, ocy) = self.cam(cx, cy);
        let ri = r as i32;
        for dy in -ri..=ri {
            let dx = ((ri * ri - dy * dy) as f32).sqrt() as i32;
            for x in (ocx - dx)..=(ocx + dx) {
                self.plot(x, ocy + dy, col);
            }
        }
    }

    pub fn draw_circle_border(&mut self, cx: f32, cy: f32, r: f32, col: u8) {
        // Midpoint circle algorithm
        let (ocx, ocy) = self.cam(cx, cy);
        let mut x = r as i32;
        let mut y = 0i32;
        let mut err = 0i32;
        while x >= y {
            self.plot(ocx + x, ocy + y, col);
            self.plot(ocx + y, ocy + x, col);
            self.plot(ocx - y, ocy + x, col);
            self.plot(ocx - x, ocy + y, col);
            self.plot(ocx - x, ocy - y, col);
            self.plot(ocx - y, ocy - x, col);
            self.plot(ocx + y, ocy - x, col);
            self.plot(ocx + x, ocy - y, col);
            if err <= 0 {
                y += 1;
                err += 2 * y + 1;
            }
            if err > 0 {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }

    pub fn draw_text(&mut self, x: f32, y: f32, s: &str, col: u8) {
        use crate::backend::font::FONT;
        let (mut px, py) = self.cam(x, y);
        let base_x = px;
        for ch in s.chars() {
            if ch == '\n' {
                px = base_x;
                // next line — but draw_text only does single line for now
                continue;
            }
            let ci = ch as usize;
            if ci < 32 || ci >= 128 {
                px += 4;
                continue;
            }
            let glyph = &FONT[ci - 32];
            for (row, &bits) in glyph.iter().enumerate() {
                for bit in 0..4u8 {
                    if bits & (0x80 >> bit) != 0 {
                        self.plot(px + bit as i32, py + row as i32, col);
                    }
                }
            }
            px += 4;
        }
    }

    /// Draw image from image bank (stub — loads from pyxel image bank later)
    pub fn draw_image(
        &mut self,
        _x: f32, _y: f32,
        _img: u32,
        _sx: f32, _sy: f32, _sw: f32, _sh: f32,
        _colkey: Option<u8>,
    ) {
        // TODO: implement image bank
    }

    /// Draw tilemap (stub)
    pub fn draw_tilemap(
        &mut self,
        _x: f32, _y: f32,
        _tm: u32,
        _mx: f32, _my: f32, _mw: f32, _mh: f32,
        _colkey: Option<u8>,
    ) {
        // TODO: implement tilemap
    }
}
