/// Aoba Castle Ninja — 青葉城 忍者登り
/// 石垣の足場を踏み台に天守閣を目指す縦スクロールゲーム
use crate::prelude::*;

const W: u32 = 160;
const H: u32 = 200;
const WF: f32 = W as f32;
const HF: f32 = H as f32;

// 忍者サイズ
const NW: f32 = 8.0;
const NH: f32 = 10.0;

// 足場
const PLAT_H: f32 = 4.0;

// 世界の高さ
const WORLD_H: f32 = 1400.0;

// パレット
const BLACK:       u8 = 0;
const DARK_BLUE:   u8 = 1;
const DARK_PRP:    u8 = 2;
const DARK_GRAY:   u8 = 5;
const LIGHT_GRAY:  u8 = 6;
const WHITE:       u8 = 7;
const RED:         u8 = 8;
const ORANGE:      u8 = 9;
const YELLOW:      u8 = 10;
const LIGHT_GREEN: u8 = 11;
const SKY_BLUE:    u8 = 12;
const INDIGO:      u8 = 13;
const PINK:        u8 = 14;

// ── 難易度 ─────────────────────────────────────────────────────────────────
#[derive(PartialEq, Clone, Copy)]
enum Diff { Easy, Normal, Hard }

struct DiffConfig {
    steps:       u32,
    step_scale:  f32, // ステップ間隔倍率
    plat_w_mul:  f32, // 足場幅倍率
    gravity:     f32,
    jump_vel:    f32,
    moon_size:   f32, // 月の半径
    moon_col:    u8,  // 月の色
}

impl DiffConfig {
    fn get(d: Diff) -> Self {
        match d {
            Diff::Easy   => DiffConfig { steps: 34, step_scale: 0.75, plat_w_mul: 1.4,
                                         gravity: 0.35, jump_vel: -6.5,
                                         moon_size: 8.0, moon_col: LIGHT_GRAY },
            Diff::Normal => DiffConfig { steps: 28, step_scale: 1.00, plat_w_mul: 1.0,
                                         gravity: 0.40, jump_vel: -6.5,
                                         moon_size: 7.0, moon_col: LIGHT_GRAY },
            Diff::Hard   => DiffConfig { steps: 22, step_scale: 1.30, plat_w_mul: 0.65,
                                         gravity: 0.48, jump_vel: -6.2,
                                         moon_size: 6.0, moon_col: YELLOW },
        }
    }
    fn label(d: Diff) -> &'static str {
        match d { Diff::Easy => "EASY", Diff::Normal => "NORMAL", Diff::Hard => "HARD" }
    }
    fn label_col(d: Diff) -> u8 {
        match d { Diff::Easy => LIGHT_GREEN, Diff::Normal => YELLOW, Diff::Hard => RED }
    }
}

// ── ゲーム状態 ─────────────────────────────────────────────────────────────
#[derive(PartialEq, Clone, Copy)]
enum Phase { Title, Play, Clear, GameOver }

// ── 足場 ──────────────────────────────────────────────────────────────────
#[derive(Clone)]
struct Platform { x: f32, y: f32, w: f32 }

// ── 忍者プレイヤー ─────────────────────────────────────────────────────────
struct Ninja {
    x: f32, y: f32,
    vx: f32, vy: f32,
    grounded: bool,
    face_r:   bool,
    anim:     u32,
    trail:    Vec<(f32, f32)>,
}

impl Ninja {
    fn new() -> Self {
        Ninja { x: WF / 2.0 - NW / 2.0, y: WORLD_H - HF * 0.15,
                vx: 0.0, vy: 0.0, grounded: false, face_r: true,
                anim: 0, trail: Vec::new() }
    }

    fn update(&mut self, platforms: &[Platform], cfg: &DiffConfig) {
        self.anim += 1;

        // タッチ/マウス 3ゾーン判定
        // 上1/3 = ジャンプ、左下 = 左移動、右下 = 右移動
        let touch = btn(MOUSE_BUTTON_LEFT);
        let mx = mouse_x() as f32;
        let my = mouse_y() as f32;
        let touch_jump  = touch && my < HF / 3.0;
        let touch_left  = touch && !touch_jump && mx < WF / 2.0;
        let touch_right = touch && !touch_jump && mx >= WF / 2.0;

        if btn(KEY_LEFT)  || btn(KEY_A) || touch_left  { self.vx = -1.8; self.face_r = false; }
        else if btn(KEY_RIGHT) || btn(KEY_D) || touch_right { self.vx = 1.8; self.face_r = true; }
        else { self.vx *= 0.75; }

        let jump_key = btnp(KEY_SPACE) || btnp(KEY_Z) || btnp(KEY_UP) || btnp(KEY_W);
        // タッチジャンプ: 上ゾーンに入った瞬間(エッジ検出は btn で代用 — 連続入力で連打防止)
        let touch_jump_edge = touch_jump && my < HF / 3.0;
        if (jump_key || touch_jump_edge) && self.grounded {
            self.vy = cfg.jump_vel;
            self.grounded = false;
        }

        self.vy = (self.vy + cfg.gravity).min(10.0);
        self.x += self.vx;
        self.y += self.vy;
        self.x = self.x.max(0.0).min(WF - NW);

        self.grounded = false;
        for p in platforms {
            if self.vy >= 0.0
                && self.x + NW > p.x
                && self.x < p.x + p.w
                && self.y + NH >= p.y
                && self.y + NH <= p.y + PLAT_H + self.vy.abs() + 1.0
            {
                self.y = p.y - NH;
                self.vy = 0.0;
                self.grounded = true;
            }
        }

        if self.anim % 2 == 0 {
            self.trail.push((self.x, self.y));
            if self.trail.len() > 4 { self.trail.remove(0); }
        }
    }

    fn draw(&self, cam_y: f32) {
        let sy = self.y - cam_y;
        for (i, &(tx, ty)) in self.trail.iter().enumerate() {
            let ts = ty - cam_y;
            let col = if i < 2 { DARK_BLUE } else { DARK_PRP };
            rectfill(tx + 1.0, ts + 1.0, NW - 2.0, NH - 2.0, col);
        }
        rectfill(self.x, sy, NW, NH, BLACK);
        let eye_x = if self.face_r { self.x + 5.0 } else { self.x + 1.0 };
        pset(eye_x, sy + 3.0, WHITE);
        if !self.grounded && self.anim % 8 < 4 {
            pset(self.x + 3.0, sy + 6.0, LIGHT_GRAY);
            pset(self.x + 4.0, sy + 7.0, LIGHT_GRAY);
        }
    }
}

// ── パーティクル ──────────────────────────────────────────────────────────
struct Particle { x: f32, y: f32, vx: f32, vy: f32, life: i32, col: u8 }
impl Particle {
    fn update(&mut self) {
        self.x += self.vx; self.y += self.vy; self.vy += 0.1; self.life -= 1;
    }
}

// ── ゲーム ────────────────────────────────────────────────────────────────
struct Game {
    phase:       Phase,
    diff:        Diff,
    diff_sel:    usize, // タイトル選択カーソル
    ninja:       Ninja,
    platforms:   Vec<Platform>,
    cam_y:       f32,
    particles:   Vec<Particle>,
    frame:       u32,
    clear_timer: u32,
}

const DIFFS: [Diff; 3] = [Diff::Easy, Diff::Normal, Diff::Hard];

impl Game {
    fn new() -> Self {
        let diff = Diff::Normal;
        let cfg = DiffConfig::get(diff);
        let platforms = Self::gen_platforms(&cfg);
        Game {
            phase: Phase::Title, diff, diff_sel: 1,
            ninja: Ninja::new(), platforms,
            cam_y: WORLD_H - HF,
            particles: Vec::new(),
            frame: 0, clear_timer: 0,
        }
    }

    fn gen_platforms(cfg: &DiffConfig) -> Vec<Platform> {
        let mut v = Vec::new();
        v.push(Platform { x: 0.0, y: WORLD_H - 20.0, w: WF });

        let step_h = (WORLD_H - 80.0) / cfg.steps as f32 * cfg.step_scale;
        let xs: [f32; 8] = [8.0, 30.0, 52.0, 75.0, 95.0, 115.0, 10.0, 65.0];
        let base_ws: [f32; 8] = [28.0, 24.0, 20.0, 26.0, 22.0, 18.0, 30.0, 22.0];

        for i in 0..cfg.steps {
            let idx = (i * 3 + i / 2) as usize % xs.len();
            let y = WORLD_H - 40.0 - step_h * (i + 1) as f32;
            if y < 20.0 { break; }
            let w = (base_ws[idx] * cfg.plat_w_mul).max(10.0);
            v.push(Platform { x: xs[idx], y, w });
            if i % 3 == 1 {
                let idx2 = (idx + 4) % xs.len();
                let w2 = (base_ws[idx2] * cfg.plat_w_mul - 4.0).max(8.0);
                v.push(Platform { x: xs[idx2], y: y - 6.0, w: w2 });
            }
        }
        // 天守閣直前
        v.push(Platform { x: 20.0, y: 30.0, w: WF - 40.0 });
        v
    }

    fn reset_with_diff(&mut self, diff: Diff) {
        self.diff = diff;
        let cfg = DiffConfig::get(diff);
        self.ninja       = Ninja::new();
        self.platforms   = Self::gen_platforms(&cfg);
        self.cam_y       = WORLD_H - HF;
        self.particles.clear();
        self.frame       = 0;
        self.clear_timer = 0;
    }

    fn spawn_burst(&mut self, cols: &[u8]) {
        for i in 0..32usize {
            let angle = i as f32 * std::f32::consts::TAU / 32.0;
            let s = 2.0 + (i as f32 * 0.2).sin().abs() * 2.0;
            self.particles.push(Particle {
                x: self.ninja.x + NW / 2.0, y: self.ninja.y,
                vx: angle.cos() * s, vy: angle.sin() * s,
                life: 40 + (i as i32 % 10),
                col: cols[i % cols.len()],
            });
        }
    }

    fn update(&mut self) {
        self.frame += 1;

        match self.phase {
            Phase::Title => {
                // 難易度選択: 左右キー or 左右タップ
                if btnp(KEY_LEFT) || btnp(KEY_A) {
                    if self.diff_sel > 0 { self.diff_sel -= 1; }
                }
                if btnp(KEY_RIGHT) || btnp(KEY_D) {
                    if self.diff_sel < 2 { self.diff_sel += 1; }
                }
                // タッチ: 上ゾーン=開始、左下=難易度↓、右下=難易度↑
                if btnp(MOUSE_BUTTON_LEFT) {
                    let mx = mouse_x() as f32;
                    let my = mouse_y() as f32;
                    if my < HF / 3.0 {
                        // 上タップ → 開始
                        let chosen = DIFFS[self.diff_sel];
                        self.reset_with_diff(chosen);
                        self.phase = Phase::Play;
                    } else if mx < WF / 2.0 {
                        if self.diff_sel > 0 { self.diff_sel -= 1; }
                    } else {
                        if self.diff_sel < 2 { self.diff_sel += 1; }
                    }
                }
                if btnp(KEY_SPACE) || btnp(KEY_Z) || btnp(KEY_UP) || btnp(KEY_W) {
                    let chosen = DIFFS[self.diff_sel];
                    self.reset_with_diff(chosen);
                    self.phase = Phase::Play;
                }
            }
            Phase::Play => {
                let cfg = DiffConfig::get(self.diff);
                self.ninja.update(&self.platforms, &cfg);

                let target_cam = self.ninja.y - HF * 0.65;
                if target_cam < self.cam_y {
                    self.cam_y += (target_cam - self.cam_y) * 0.1;
                }
                self.cam_y = self.cam_y.max(0.0);

                for p in &mut self.particles { p.update(); }
                self.particles.retain(|p| p.life > 0);

                if self.ninja.y > WORLD_H + 30.0 { self.phase = Phase::GameOver; }

                if self.ninja.grounded && self.ninja.y < 40.0 {
                    self.phase = Phase::Clear;
                    let burst_cols = [YELLOW, WHITE, LIGHT_GREEN, SKY_BLUE, PINK, ORANGE];
                    self.spawn_burst(&burst_cols);
                }
            }
            Phase::Clear => {
                self.clear_timer += 1;
                for p in &mut self.particles { p.update(); }
                self.particles.retain(|p| p.life > 0);
                if self.clear_timer % 50 == 0 && self.clear_timer < 300 {
                    let burst_cols = [YELLOW, WHITE, LIGHT_GREEN, SKY_BLUE, PINK, ORANGE];
                    self.spawn_burst(&burst_cols);
                }
                if (btnp(KEY_SPACE) || btnp(KEY_Z) || btnp(MOUSE_BUTTON_LEFT))
                    && self.clear_timer > 60 {
                    self.phase = Phase::Title;
                }
            }
            Phase::GameOver => {
                if btnp(KEY_SPACE) || btnp(KEY_Z) || btnp(MOUSE_BUTTON_LEFT) {
                    self.reset_with_diff(self.diff);
                    self.phase = Phase::Play;
                }
            }
        }
    }

    fn draw(&self) {
        cls(DARK_BLUE);
        match self.phase {
            Phase::Title                          => self.draw_title(),
            Phase::Play | Phase::Clear | Phase::GameOver => {
                self.draw_play();
                match self.phase {
                    Phase::Clear    => self.draw_clear(),
                    Phase::GameOver => self.draw_gameover(),
                    _               => self.draw_hud(),
                }
            }
        }
    }

    // ── タイトル ─────────────────────────────────────────────────────────────
    fn draw_title(&self) {
        // 夜空
        rectfill(0.0, 0.0, WF, HF, INDIGO);
        // 星をちりばめる
        let stars: [(f32,f32); 12] = [
            (12.0,8.0),(45.0,4.0),(90.0,12.0),(130.0,6.0),(155.0,20.0),
            (25.0,30.0),(70.0,25.0),(110.0,18.0),(5.0,45.0),(140.0,35.0),
            (55.0,50.0),(85.0,40.0),
        ];
        for (i, &(sx, sy)) in stars.iter().enumerate() {
            let col = if (self.frame / 20 + i as u32) % 3 == 0 { LIGHT_GRAY } else { WHITE };
            pset(sx, sy, col);
        }
        // 月 (タイトルは常に満月)
        circfill(WF - 22.0, 18.0, 9.0, WHITE);
        circfill(WF - 18.0, 14.0, 7.0, INDIGO); // 三日月の欠け
        // 城シルエット
        self.draw_castle_silhouette(30.0, HF * 0.28);
        // タイトルボックス
        rectfill(8.0, 68.0, WF - 16.0, 52.0, BLACK);
        rect(8.0, 68.0, WF - 16.0, 52.0, YELLOW);
        text(20.0, 74.0, "AOBA CASTLE", YELLOW);
        text(28.0, 84.0, "NINJA CLIMB", WHITE);

        // 難易度セレクター
        text(22.0, 97.0, "DIFFICULTY:", LIGHT_GRAY);
        let labels = ["EASY", "NORMAL", "HARD"];
        let cols   = [LIGHT_GREEN, YELLOW, RED];
        let lx: [f32; 3] = [8.0, 52.0, 110.0];
        for (i, (&lbl, &col)) in labels.iter().zip(cols.iter()).enumerate() {
            if self.diff_sel == i {
                rectfill(lx[i] - 1.0, 106.0, (lbl.len() * 4 + 3) as f32, 9.0, DARK_GRAY);
                rect(lx[i] - 1.0, 106.0, (lbl.len() * 4 + 3) as f32, 9.0, col);
                text(lx[i], 108.0, lbl, col);
            } else {
                text(lx[i], 108.0, lbl, DARK_GRAY);
            }
        }
        // 矢印ガイド
        if self.diff_sel > 0 { text(2.0, 108.0, "<", LIGHT_GRAY); }
        if self.diff_sel < 2 { text(WF - 6.0, 108.0, ">", LIGHT_GRAY); }

        text(28.0, 120.0, "PRESS JUMP!", LIGHT_GREEN);

        // タッチゾーンガイド (タイトルの下1/3)
        line(0.0, HF * 2.0 / 3.0, WF, HF * 2.0 / 3.0, DARK_GRAY);
        line(WF / 2.0, HF * 2.0 / 3.0, WF / 2.0, HF, DARK_GRAY);
        text(4.0, HF * 2.0 / 3.0 + 3.0, "< EASIER", DARK_GRAY);
        text(WF / 2.0 + 2.0, HF * 2.0 / 3.0 + 3.0, "HARDER >", DARK_GRAY);
        // 上ゾーン
        text(WF / 2.0 - 14.0, HF * 2.0 / 3.0 - 8.0, "TAP:START", DARK_GRAY);

        // キーボード操作説明
        text(2.0, HF - 10.0, "L/R:diff  UP/Z:start", DARK_GRAY);
    }

    // ── プレイ ────────────────────────────────────────────────────────────────
    fn draw_play(&self) {
        let cam  = self.cam_y;
        let cfg  = DiffConfig::get(self.diff);

        // 高度に応じた空の色 (下=INDIGO, 上=DARK_BLUE)
        let t = (1.0 - self.ninja.y / WORLD_H).max(0.0).min(1.0);
        let sky = if t > 0.6 { DARK_BLUE } else { INDIGO };
        rectfill(0.0, 0.0, WF, HF, sky);

        // 星 (上部では多く光る)
        if t > 0.4 {
            let stars: [(f32,f32); 10] = [
                (12.0,8.0),(45.0,4.0),(90.0,12.0),(130.0,6.0),(155.0,20.0),
                (25.0,30.0),(70.0,25.0),(5.0,45.0),(140.0,35.0),(55.0,50.0),
            ];
            for (i, &(sx, sy)) in stars.iter().enumerate() {
                if (self.frame / 25 + i as u32) % 3 != 0 {
                    pset(sx, sy, LIGHT_GRAY);
                }
            }
        }

        // 月 — 常に表示、高度で「満ち欠け」
        // t=0(下): 三日月 / t=1(上): 満月
        let moon_r   = cfg.moon_size;
        let crescent = moon_r * (1.0 - t) * 0.9; // 欠けの大きさ
        circfill(WF - 20.0, 14.0, moon_r, cfg.moon_col);
        if crescent > 0.5 {
            circfill(WF - 20.0 + crescent * 0.7, 14.0 - crescent * 0.3,
                     crescent + 2.0, sky);
        }

        // 石垣の壁面 (背景テクスチャ)
        self.draw_wall(cam);

        // 足場
        for p in &self.platforms {
            let sy = p.y - cam;
            if sy < -PLAT_H || sy > HF + 2.0 { continue; }
            rectfill(p.x, sy, p.w, PLAT_H, DARK_GRAY);
            rect(p.x, sy, p.w, PLAT_H, LIGHT_GRAY);
            let segs = (p.w / 8.0) as usize;
            for s in 1..segs {
                line(p.x + s as f32 * 8.0, sy,
                     p.x + s as f32 * 8.0, sy + PLAT_H - 1.0, DARK_BLUE);
            }
        }

        // パーティクル
        for p in &self.particles {
            let sy = p.y - cam;
            if sy >= 0.0 && sy < HF { pset(p.x, sy, p.col); }
        }

        // 忍者
        self.ninja.draw(cam);

        // 天守閣 (上部が見えたら)
        let castle_sy = 0.0 - cam;
        if castle_sy < HF { self.draw_castle_silhouette(30.0, castle_sy); }

        // 高度バー (右端)
        let progress = t;
        let ind_h = HF - 20.0;
        rect(WF - 6.0, 8.0, 4.0, ind_h, DARK_BLUE);
        let fill_h = ind_h * progress;
        rectfill(WF - 5.0, 8.0 + ind_h - fill_h, 2.0, fill_h, YELLOW);
        text(WF - 8.0, 2.0, "^", LIGHT_GRAY);
    }

    fn draw_hud(&self) {
        let t   = (1.0 - self.ninja.y / WORLD_H).max(0.0).min(1.0);
        let pct = (t * 100.0) as u32;
        let diff_lbl = DiffConfig::label(self.diff);
        let diff_col = DiffConfig::label_col(self.diff);
        text(2.0, 2.0, diff_lbl, diff_col);
        let msg = format!("{}%", pct);
        text(WF - 4.0 * (msg.len() as f32 + 1.0), 2.0, &msg, YELLOW);

        // タッチガイド: 最初の180フレーム(3秒)だけ表示
        if self.frame < 180 {
            let alpha = if self.frame > 140 { 255 - (self.frame - 140) * 5 } else { 80 };
            let col = if alpha > 40 { DARK_GRAY } else { BLACK };
            // 分割ライン (上1/3)
            line(0.0, HF / 3.0, WF - 8.0, HF / 3.0, col);
            // 縦ライン (左右)
            line(WF / 2.0, HF / 3.0, WF / 2.0, HF, col);
            // ラベル
            text(WF / 2.0 - 14.0, HF / 3.0 + 2.0, "JUMP", col);
            text(4.0, HF / 2.0 + 20.0, "LEFT", col);
            text(WF / 2.0 + 4.0, HF / 2.0 + 20.0, "RIGHT", col);
        }
    }

    fn draw_wall(&self, cam: f32) {
        let mut row = 0usize;
        let mut yy = cam % 16.0;
        while yy < HF {
            let offset = if row % 2 == 0 { 0.0 } else { 8.0 };
            let mut xx = offset - 8.0;
            while xx < WF {
                rectfill(xx, yy, 7.0, 7.0, DARK_BLUE);
                rect(xx, yy, 7.0, 7.0, DARK_PRP);
                xx += 8.0;
            }
            yy += 8.0;
            row += 1;
        }
    }

    fn draw_castle_silhouette(&self, x: f32, y: f32) {
        rectfill(x + 10.0, y + 30.0, 40.0, 20.0, DARK_GRAY);
        rectfill(x + 15.0, y + 15.0, 30.0, 18.0, DARK_GRAY);
        rectfill(x + 20.0, y + 5.0,  20.0, 12.0, DARK_GRAY);
        rectfill(x + 27.0, y,          6.0,  8.0, DARK_GRAY);
        pset(x + 22.0, y + 20.0, YELLOW);
        pset(x + 36.0, y + 20.0, YELLOW);
        pset(x + 27.0, y + 10.0, YELLOW);
    }

    fn draw_clear(&self) {
        let diff_col = DiffConfig::label_col(self.diff);
        rectfill(18.0, HF / 2.0 - 22.0, WF - 36.0, 50.0, BLACK);
        rect(18.0, HF / 2.0 - 22.0, WF - 36.0, 50.0, diff_col);
        if self.clear_timer % 8 < 6 {
            text(40.0, HF / 2.0 - 16.0, "CLEARED!", WHITE);
        }
        text(28.0, HF / 2.0 - 4.0,  "AOBA CASTLE",  YELLOW);
        text(34.0, HF / 2.0 + 6.0,  "CONQUERED!",   LIGHT_GREEN);
        let dlbl = DiffConfig::label(self.diff);
        let dmsg = format!("[{}]", dlbl);
        text(WF / 2.0 - dmsg.len() as f32 * 2.0,
             HF / 2.0 + 16.0, &dmsg, diff_col);
        if self.clear_timer > 60 {
            text(28.0, HF / 2.0 + 24.0, "PRESS JUMP", LIGHT_GRAY);
        }
    }

    fn draw_gameover(&self) {
        rectfill(22.0, HF / 2.0 - 14.0, WF - 44.0, 30.0, BLACK);
        rect(22.0, HF / 2.0 - 14.0, WF - 44.0, 30.0, RED);
        text(36.0, HF / 2.0 - 8.0, "GAME OVER", RED);
        text(30.0, HF / 2.0 + 4.0, "PRESS JUMP", LIGHT_GRAY);
    }
}

// ── エントリポイント ────────────────────────────────────────────────────────
pub fn start() {
    init(W, H, "Aoba Castle Ninja", 60);
    let game = std::rc::Rc::new(std::cell::RefCell::new(Game::new()));
    let gu = std::rc::Rc::clone(&game);
    let gd = std::rc::Rc::clone(&game);
    run(
        move || gu.borrow_mut().update(),
        move || gd.borrow().draw(),
    );
}
