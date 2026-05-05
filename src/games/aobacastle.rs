/// Aoba Castle Ninja — 青葉城 忍者登り
use crate::prelude::*;

const W: u32 = 400;
const H: u32 = 640;
const WF: f32 = W as f32;
const HF: f32 = H as f32;

const NW: f32 = 20.0;
const NH: f32 = 24.0;
const PLAT_H: f32 = 10.0;
const WORLD_H: f32 = 4000.0;

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

#[derive(PartialEq, Clone, Copy)]
enum Diff { Easy, Normal, Hard }

struct DiffConfig {
    steps:      u32,
    step_scale: f32,
    plat_w_mul: f32,
    gravity:    f32,
    jump_vel:   f32,
    moon_size:  f32,
    moon_col:   u8,
}

impl DiffConfig {
    fn get(d: Diff) -> Self {
        match d {
            Diff::Easy   => DiffConfig { steps: 34, step_scale: 0.75, plat_w_mul: 1.5,
                                         gravity: 0.38, jump_vel: -9.0,
                                         moon_size: 20.0, moon_col: LIGHT_GRAY },
            Diff::Normal => DiffConfig { steps: 28, step_scale: 1.00, plat_w_mul: 1.1,
                                         gravity: 0.42, jump_vel: -8.2,
                                         moon_size: 18.0, moon_col: LIGHT_GRAY },
            Diff::Hard   => DiffConfig { steps: 22, step_scale: 1.25, plat_w_mul: 0.70,
                                         gravity: 0.48, jump_vel: -8.0,
                                         moon_size: 16.0, moon_col: YELLOW },
        }
    }
    fn label(d: Diff) -> &'static str {
        match d { Diff::Easy => "EASY", Diff::Normal => "NORMAL", Diff::Hard => "HARD" }
    }
    fn label_col(d: Diff) -> u8 {
        match d { Diff::Easy => LIGHT_GREEN, Diff::Normal => YELLOW, Diff::Hard => RED }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Phase { Title, Play, Clear, GameOver }

#[derive(Clone)]
struct Platform { x: f32, y: f32, w: f32 }

struct Ninja {
    x: f32, y: f32, vx: f32, vy: f32,
    grounded: bool, face_r: bool,
    anim: u32, trail: Vec<(f32, f32)>,
}

impl Ninja {
    fn new() -> Self {
        Ninja { x: WF / 2.0 - NW / 2.0, y: WORLD_H - HF * 0.15,
                vx: 0.0, vy: 0.0, grounded: false, face_r: true,
                anim: 0, trail: Vec::new() }
    }

    fn update(&mut self, platforms: &[Platform], cfg: &DiffConfig) {
        self.anim += 1;

        // タッチ/マウス 3ゾーン判定（補助）
        let touch = btn(MOUSE_BUTTON_LEFT);
        let mx = mouse_x() as f32;
        let my = mouse_y() as f32;
        let touch_jump  = touch && my < HF / 3.0;
        let touch_left  = touch && !touch_jump && mx < WF / 2.0;
        let touch_right = touch && !touch_jump && mx >= WF / 2.0;

        if btn(KEY_LEFT)  || btn(KEY_A) || touch_left  { self.vx = -3.6; self.face_r = false; }
        else if btn(KEY_RIGHT) || btn(KEY_D) || touch_right { self.vx = 3.6; self.face_r = true; }
        else { self.vx *= 0.75; }

        let jump_key = btnp(KEY_SPACE) || btnp(KEY_Z) || btnp(KEY_UP) || btnp(KEY_W);
        if (jump_key || touch_jump) && self.grounded {
            self.vy = cfg.jump_vel;
            self.grounded = false;
        }

        self.vy = (self.vy + cfg.gravity).min(18.0);
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
            rectfill(tx + 2.0, ts + 2.0, NW - 4.0, NH - 4.0, col);
        }
        rectfill(self.x, sy, NW, NH, BLACK);
        let eye_x = if self.face_r { self.x + 13.0 } else { self.x + 3.0 };
        pset(eye_x, sy + 7.0, WHITE);
        if !self.grounded && self.anim % 8 < 4 {
            pset(self.x + 8.0, sy + 13.0, LIGHT_GRAY);
            pset(self.x + 10.0, sy + 15.0, LIGHT_GRAY);
        }
    }
}

struct Particle { x: f32, y: f32, vx: f32, vy: f32, life: i32, col: u8 }
impl Particle {
    fn update(&mut self) {
        self.x += self.vx; self.y += self.vy; self.vy += 0.2; self.life -= 1;
    }
}

struct Game {
    phase: Phase, diff: Diff, diff_sel: usize,
    ninja: Ninja, platforms: Vec<Platform>,
    cam_y: f32, particles: Vec<Particle>,
    frame: u32, clear_timer: u32,
}

const DIFFS: [Diff; 3] = [Diff::Easy, Diff::Normal, Diff::Hard];

impl Game {
    fn new() -> Self {
        let diff = Diff::Normal;
        let platforms = Self::gen_platforms(&DiffConfig::get(diff));
        Game { phase: Phase::Title, diff, diff_sel: 1,
               ninja: Ninja::new(), platforms,
               cam_y: WORLD_H - HF,
               particles: Vec::new(), frame: 0, clear_timer: 0 }
    }

    fn gen_platforms(cfg: &DiffConfig) -> Vec<Platform> {
        let mut v = Vec::new();
        v.push(Platform { x: 0.0, y: WORLD_H - 40.0, w: WF });

        let step_h = (WORLD_H - 160.0) / cfg.steps as f32 * cfg.step_scale;
        let xs:      [f32; 8] = [20.0, 76.0, 136.0, 196.0, 250.0, 300.0, 30.0, 164.0];
        let base_ws: [f32; 8] = [68.0, 60.0,  52.0,  64.0,  56.0,  48.0, 76.0,  56.0];

        for i in 0..cfg.steps {
            let idx = (i * 3 + i / 2) as usize % xs.len();
            let y = WORLD_H - 80.0 - step_h * (i + 1) as f32;
            if y < 40.0 { break; }
            let w = (base_ws[idx] * cfg.plat_w_mul).max(24.0);
            v.push(Platform { x: xs[idx], y, w });
            if i % 3 == 1 {
                let idx2 = (idx + 4) % xs.len();
                let w2 = (base_ws[idx2] * cfg.plat_w_mul - 8.0).max(20.0);
                v.push(Platform { x: xs[idx2], y: y - 16.0, w: w2 });
            }
        }
        v.push(Platform { x: 50.0, y: 70.0, w: WF - 100.0 });
        v
    }

    fn reset_with_diff(&mut self, diff: Diff) {
        self.diff = diff;
        self.ninja       = Ninja::new();
        self.platforms   = Self::gen_platforms(&DiffConfig::get(diff));
        self.cam_y       = WORLD_H - HF;
        self.particles.clear();
        self.frame = 0; self.clear_timer = 0;
    }

    fn spawn_burst(&mut self) {
        let cols = [YELLOW, WHITE, LIGHT_GREEN, SKY_BLUE, PINK, ORANGE];
        for i in 0..48usize {
            let angle = i as f32 * std::f32::consts::TAU / 48.0;
            let s = 3.0 + (i as f32 * 0.2).sin().abs() * 4.0;
            self.particles.push(Particle {
                x: self.ninja.x + NW / 2.0, y: self.ninja.y,
                vx: angle.cos() * s, vy: angle.sin() * s,
                life: 50 + (i as i32 % 12), col: cols[i % cols.len()],
            });
        }
    }

    fn update(&mut self) {
        self.frame += 1;
        match self.phase {
            Phase::Title => {
                if btnp(KEY_LEFT) || btnp(KEY_A) { if self.diff_sel > 0 { self.diff_sel -= 1; } }
                if btnp(KEY_RIGHT) || btnp(KEY_D) { if self.diff_sel < 2 { self.diff_sel += 1; } }
                if btnp(MOUSE_BUTTON_LEFT) {
                    let mx = mouse_x() as f32;
                    let my = mouse_y() as f32;
                    if my < HF * 2.0 / 3.0 {
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
                if target_cam < self.cam_y { self.cam_y += (target_cam - self.cam_y) * 0.1; }
                self.cam_y = self.cam_y.max(0.0);
                for p in &mut self.particles { p.update(); }
                self.particles.retain(|p| p.life > 0);
                if self.ninja.y > WORLD_H + 60.0 { self.phase = Phase::GameOver; }
                if self.ninja.grounded && self.ninja.y < 80.0 {
                    self.phase = Phase::Clear;
                    self.spawn_burst();
                }
            }
            Phase::Clear => {
                self.clear_timer += 1;
                for p in &mut self.particles { p.update(); }
                self.particles.retain(|p| p.life > 0);
                if self.clear_timer % 50 == 0 && self.clear_timer < 300 { self.spawn_burst(); }
                if (btnp(KEY_SPACE) || btnp(KEY_Z) || btnp(MOUSE_BUTTON_LEFT))
                    && self.clear_timer > 60 { self.phase = Phase::Title; }
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
            Phase::Title => self.draw_title(),
            _ => {
                self.draw_play();
                match self.phase {
                    Phase::Clear    => self.draw_clear(),
                    Phase::GameOver => self.draw_gameover(),
                    _               => self.draw_hud(),
                }
            }
        }
    }

    fn draw_title(&self) {
        rectfill(0.0, 0.0, WF, HF, INDIGO);
        let stars: [(f32,f32); 12] = [
            (24.0,16.0),(90.0,8.0),(180.0,24.0),(260.0,12.0),(310.0,40.0),
            (50.0,60.0),(140.0,50.0),(220.0,36.0),(10.0,90.0),(280.0,70.0),
            (110.0,100.0),(170.0,80.0),
        ];
        for (i, &(sx, sy)) in stars.iter().enumerate() {
            let col = if (self.frame / 20 + i as u32) % 3 == 0 { LIGHT_GRAY } else { WHITE };
            pset(sx, sy, col);
        }
        // 月
        circfill(WF - 44.0, 36.0, 18.0, WHITE);
        circfill(WF - 36.0, 28.0, 14.0, INDIGO);
        // 城シルエット
        self.draw_castle_silhouette(60.0, HF * 0.28);
        // タイトルボックス
        rectfill(16.0, 136.0, WF - 32.0, 104.0, BLACK);
        rect(16.0, 136.0, WF - 32.0, 104.0, YELLOW);
        text(40.0, 148.0, "AOBA CASTLE", YELLOW);
        text(56.0, 168.0, "NINJA CLIMB", WHITE);
        // 難易度セレクター
        text(44.0, 194.0, "DIFFICULTY:", LIGHT_GRAY);
        let labels = ["EASY", "NORMAL", "HARD"];
        let cols   = [LIGHT_GREEN, YELLOW, RED];
        let lx: [f32; 3] = [16.0, 104.0, 220.0];
        for (i, (&lbl, &col)) in labels.iter().zip(cols.iter()).enumerate() {
            if self.diff_sel == i {
                rectfill(lx[i]-2.0, 212.0, (lbl.len()*4+6) as f32 * 2.0, 18.0, DARK_GRAY);
                rect(lx[i]-2.0, 212.0, (lbl.len()*4+6) as f32 * 2.0, 18.0, col);
                text(lx[i], 216.0, lbl, col);
            } else {
                text(lx[i], 216.0, lbl, DARK_GRAY);
            }
        }
        if self.diff_sel > 0 { text(4.0, 216.0, "<", LIGHT_GRAY); }
        if self.diff_sel < 2 { text(WF - 10.0, 216.0, ">", LIGHT_GRAY); }
        text(56.0, 240.0, "PRESS JUMP!", LIGHT_GREEN);
        text(4.0, HF - 20.0, "L/R:diff  UP/Z:start", DARK_GRAY);
    }

    fn draw_play(&self) {
        let cam = self.cam_y;
        let cfg = DiffConfig::get(self.diff);
        let t = (1.0 - self.ninja.y / WORLD_H).max(0.0).min(1.0);
        let sky = if t > 0.6 { DARK_BLUE } else { INDIGO };
        rectfill(0.0, 0.0, WF, HF, sky);

        // 星
        if t > 0.4 {
            let stars: [(f32,f32); 10] = [
                (24.0,16.0),(90.0,8.0),(180.0,24.0),(260.0,12.0),(310.0,40.0),
                (50.0,60.0),(140.0,50.0),(10.0,90.0),(280.0,70.0),(110.0,100.0),
            ];
            for (i, &(sx, sy)) in stars.iter().enumerate() {
                if (self.frame / 25 + i as u32) % 3 != 0 { pset(sx, sy, LIGHT_GRAY); }
            }
        }

        // 月 (高度で満ち欠け)
        let moon_r   = cfg.moon_size;
        let crescent = moon_r * (1.0 - t) * 0.9;
        circfill(WF - 40.0, 28.0, moon_r, cfg.moon_col);
        if crescent > 1.0 {
            circfill(WF - 40.0 + crescent * 0.7, 28.0 - crescent * 0.3,
                     crescent + 4.0, sky);
        }

        self.draw_wall(cam);

        for p in &self.platforms {
            let sy = p.y - cam;
            if sy < -PLAT_H || sy > HF + 2.0 { continue; }
            rectfill(p.x, sy, p.w, PLAT_H, DARK_GRAY);
            rect(p.x, sy, p.w, PLAT_H, LIGHT_GRAY);
            let segs = (p.w / 16.0) as usize;
            for s in 1..segs {
                line(p.x + s as f32 * 16.0, sy,
                     p.x + s as f32 * 16.0, sy + PLAT_H - 1.0, DARK_BLUE);
            }
        }

        for p in &self.particles {
            let sy = p.y - cam;
            if sy >= 0.0 && sy < HF { pset(p.x, sy, p.col); }
        }

        self.ninja.draw(cam);

        let castle_sy = 0.0 - cam;
        if castle_sy < HF { self.draw_castle_silhouette(60.0, castle_sy); }

        let progress = t;
        let ind_h = HF - 40.0;
        rect(WF - 12.0, 16.0, 8.0, ind_h, DARK_BLUE);
        let fill_h = ind_h * progress;
        rectfill(WF - 10.0, 16.0 + ind_h - fill_h, 4.0, fill_h, YELLOW);
        text(WF - 14.0, 4.0, "^", LIGHT_GRAY);
    }

    fn draw_hud(&self) {
        let t   = (1.0 - self.ninja.y / WORLD_H).max(0.0).min(1.0);
        let pct = (t * 100.0) as u32;
        let diff_lbl = DiffConfig::label(self.diff);
        let diff_col = DiffConfig::label_col(self.diff);
        text(4.0, 4.0, diff_lbl, diff_col);
        let msg = format!("{}%", pct);
        text(WF - 4.0 * (msg.len() as f32 + 1.0) - 14.0, 4.0, &msg, YELLOW);
    }

    fn draw_wall(&self, cam: f32) {
        let mut row = 0usize;
        let mut yy = cam % 32.0;
        while yy < HF {
            let offset = if row % 2 == 0 { 0.0 } else { 16.0 };
            let mut xx = offset - 16.0;
            while xx < WF {
                rectfill(xx, yy, 14.0, 14.0, DARK_BLUE);
                rect(xx, yy, 14.0, 14.0, DARK_PRP);
                xx += 16.0;
            }
            yy += 16.0;
            row += 1;
        }
    }

    fn draw_castle_silhouette(&self, x: f32, y: f32) {
        rectfill(x + 20.0, y + 60.0,  80.0, 40.0, DARK_GRAY);
        rectfill(x + 30.0, y + 30.0,  60.0, 36.0, DARK_GRAY);
        rectfill(x + 40.0, y + 10.0,  40.0, 24.0, DARK_GRAY);
        rectfill(x + 54.0, y,          12.0, 16.0, DARK_GRAY);
        pset(x + 44.0, y + 40.0, YELLOW);
        pset(x + 72.0, y + 40.0, YELLOW);
        pset(x + 54.0, y + 20.0, YELLOW);
    }

    fn draw_clear(&self) {
        let diff_col = DiffConfig::label_col(self.diff);
        rectfill(36.0, HF/2.0-44.0, WF-72.0, 100.0, BLACK);
        rect(36.0, HF/2.0-44.0, WF-72.0, 100.0, diff_col);
        if self.clear_timer % 8 < 6 {
            text(80.0, HF/2.0-32.0, "CLEARED!", WHITE);
        }
        text(56.0, HF/2.0-8.0,  "AOBA CASTLE", YELLOW);
        text(68.0, HF/2.0+12.0, "CONQUERED!",  LIGHT_GREEN);
        let dlbl = DiffConfig::label(self.diff);
        let dmsg = format!("[{}]", dlbl);
        text(WF/2.0 - dmsg.len() as f32 * 2.0, HF/2.0+32.0, &dmsg, diff_col);
        if self.clear_timer > 60 {
            text(56.0, HF/2.0+48.0, "PRESS JUMP", LIGHT_GRAY);
        }
    }

    fn draw_gameover(&self) {
        rectfill(44.0, HF/2.0-28.0, WF-88.0, 60.0, BLACK);
        rect(44.0, HF/2.0-28.0, WF-88.0, 60.0, RED);
        text(72.0, HF/2.0-16.0, "GAME OVER", RED);
        text(60.0, HF/2.0+8.0,  "PRESS JUMP", LIGHT_GRAY);
    }
}

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
