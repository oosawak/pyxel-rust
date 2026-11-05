/// Software drawing primitives for WgpuBackend
///
/// All operations write color indices (u8) into the pixel buffer.
/// Coordinates are in screen pixels (after camera offset).
use super::WgpuState;

// Resource bank layout matches the browser software renderer.
const IMAGE_W: u32 = 256;
const IMAGE_H: u32 = 256;
const IMAGE_SIZE: usize = (IMAGE_W * IMAGE_H) as usize;
const TILEMAP_W: u32 = 256;
const TILEMAP_H: u32 = 256;
const TILE_SIZE: u32 = 8;

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
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
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

    pub fn draw_image(
        &mut self,
        x: f32,
        y: f32,
        img: u32,
        sx: f32,
        sy: f32,
        sw: f32,
        sh: f32,
        colkey: Option<u8>,
    ) {
        if img >= 3 {
            return;
        }
        let (dx, dy) = self.cam(x, y);
        let (sxi, syi, swi, shi) = (sx as i32, sy as i32, sw.abs() as i32, sh.abs() as i32);
        let flip_x = sw < 0.0;
        let flip_y = sh < 0.0;
        for py in 0..shi {
            for px in 0..swi {
                let src_x = if flip_x { sxi + swi - 1 - px } else { sxi + px };
                let src_y = if flip_y { syi + shi - 1 - py } else { syi + py };
                if src_x < 0 || src_y < 0 || src_x >= IMAGE_W as i32 || src_y >= IMAGE_H as i32 {
                    continue;
                }
                let col = self.image_banks[img as usize]
                    [(src_y as u32 * IMAGE_W + src_x as u32) as usize];
                if colkey == Some(col) {
                    continue;
                }
                self.plot(dx + px, dy + py, col);
            }
        }
    }

    pub fn draw_tilemap(
        &mut self,
        x: f32,
        y: f32,
        tm: u32,
        mx: f32,
        my: f32,
        mw: f32,
        mh: f32,
        colkey: Option<u8>,
    ) {
        if tm >= 8 {
            return;
        }
        let tile_x = (mx as u32) / TILE_SIZE;
        let tile_y = (my as u32) / TILE_SIZE;
        let count_x = ((mw as u32) + TILE_SIZE - 1) / TILE_SIZE;
        let count_y = ((mh as u32) + TILE_SIZE - 1) / TILE_SIZE;
        let (base_dx, base_dy) = self.cam(x, y);
        for ty in 0..count_y {
            for tx in 0..count_x {
                let map_tx = tile_x + tx;
                let map_ty = tile_y + ty;
                if map_tx >= TILEMAP_W || map_ty >= TILEMAP_H {
                    continue;
                }
                let (stx, sty) = self.tilemaps[tm as usize][(map_ty * TILEMAP_W + map_tx) as usize];
                if stx == 0 && sty == 0 {
                    continue;
                }
                let src_px = (stx as u32) * TILE_SIZE;
                let src_py = (sty as u32) * TILE_SIZE;
                let dst_x = base_dx + (tx * TILE_SIZE) as i32;
                let dst_y = base_dy + (ty * TILE_SIZE) as i32;
                for py in 0..TILE_SIZE as i32 {
                    for px in 0..TILE_SIZE as i32 {
                        let si = ((src_py as i32 + py) as u32 * IMAGE_W
                            + (src_px as i32 + px) as u32)
                            as usize;
                        if si >= IMAGE_SIZE {
                            continue;
                        }
                        let col = self.image_banks[0][si];
                        if colkey == Some(col) {
                            continue;
                        }
                        self.plot(dst_x + px, dst_y + py, col);
                    }
                }
            }
        }
    }

    // --- Image bank drawing ---

    pub fn image_plot(&mut self, bank: u32, x: i32, y: i32, col: u8) {
        if bank >= 3 {
            return;
        }
        if x < 0 || y < 0 || x >= IMAGE_W as i32 || y >= IMAGE_H as i32 {
            return;
        }
        self.image_banks[bank as usize][(y as u32 * IMAGE_W + x as u32) as usize] = col & 0x0f;
    }

    pub fn image_pset(&mut self, bank: u32, x: f32, y: f32, col: u8) {
        self.image_plot(bank, x as i32, y as i32, col);
    }

    pub fn image_rect(&mut self, bank: u32, x: f32, y: f32, w: f32, h: f32, col: u8) {
        for dy in 0..h as i32 {
            for dx in 0..w as i32 {
                self.image_plot(bank, x as i32 + dx, y as i32 + dy, col);
            }
        }
    }

    pub fn image_line(&mut self, bank: u32, x1: f32, y1: f32, x2: f32, y2: f32, col: u8) {
        let (mut x0, mut y0) = (x1 as i32, y1 as i32);
        let (x1i, y1i) = (x2 as i32, y2 as i32);
        let dx = (x1i - x0).abs();
        let dy = -(y1i - y0).abs();
        let sx = if x0 < x1i { 1 } else { -1 };
        let sy = if y0 < y1i { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            self.image_plot(bank, x0, y0, col);
            if x0 == x1i && y0 == y1i {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn image_tri(
        &mut self,
        bank: u32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        col: u8,
    ) {
        let mut pts = [
            (x1 as i32, y1 as i32),
            (x2 as i32, y2 as i32),
            (x3 as i32, y3 as i32),
        ];
        pts.sort_by_key(|p| p.1);
        let (ax, ay) = pts[0];
        let (bx, by) = pts[1];
        let (cx, cy) = pts[2];
        let interp = |x1: i32, y1: i32, x2: i32, y2: i32, y: i32| -> i32 {
            if y2 == y1 {
                x1
            } else {
                x1 + (x2 - x1) * (y - y1) / (y2 - y1)
            }
        };
        for y in ay..=cy {
            let lx = if y < by {
                interp(ax, ay, bx, by, y)
            } else {
                interp(bx, by, cx, cy, y)
            };
            let rx = interp(ax, ay, cx, cy, y);
            let (l, r) = (lx.min(rx), lx.max(rx));
            for x in l..=r {
                self.image_plot(bank, x, y, col);
            }
        }
    }

    // --- Tilemap access ---

    pub fn tilemap_cls(&mut self, tm: u32, tile: (u8, u8)) {
        if tm >= 8 {
            return;
        }
        for t in self.tilemaps[tm as usize].iter_mut() {
            *t = tile;
        }
    }

    pub fn tilemap_pget(&self, tm: u32, x: u32, y: u32) -> (u8, u8) {
        if tm >= 8 || x >= TILEMAP_W || y >= TILEMAP_H {
            return (0, 0);
        }
        self.tilemaps[tm as usize][(y * TILEMAP_W + x) as usize]
    }

    pub fn tilemap_pset(&mut self, tm: u32, x: u32, y: u32, tile: (u8, u8)) {
        if tm >= 8 || x >= TILEMAP_W || y >= TILEMAP_H {
            return;
        }
        self.tilemaps[tm as usize][(y * TILEMAP_W + x) as usize] = tile;
    }
}
