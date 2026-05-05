/// NanoTerras — 光の環ゲーム
/// 仙台放射光施設「ナノテラス」をテーマにした
/// 光のリングが画面を回り続けるアンビエントゲーム。
/// タップ/クリックでバースト光粒子を追加できる。
use crate::prelude::*;

const W: u32 = 256;
const H: u32 = 256;

const BLACK:      u8 = 0;
const DARK_BLUE:  u8 = 1;
const TEAL:       u8 = 3;
const DARK_GRAY:  u8 = 5;
const LIGHT_BLUE: u8 = 6;
const WHITE:      u8 = 7;
const YELLOW:     u8 = 10;
const LIGHT_GREEN:u8 = 11;
const SKY_BLUE:   u8 = 12;
const PINK:       u8 = 14;

// ── 軌道リング ─────────────────────────────────────────────────────────────
struct Ring {
    radius: f32,
    speed:  f32,   // rad/frame
    phase:  f32,
    color:  u8,
    n:      usize, // 粒子数
}

impl Ring {
    fn new(radius: f32, speed: f32, phase: f32, color: u8, n: usize) -> Self {
        Ring { radius, speed, phase, color, n }
    }

    fn update(&mut self) {
        self.phase += self.speed;
    }

    fn draw(&self, cx: f32, cy: f32) {
        let step = std::f32::consts::TAU / self.n as f32;
        for i in 0..self.n {
            let angle = self.phase + i as f32 * step;
            let x = cx + self.radius * angle.cos();
            let y = cy + self.radius * angle.sin();
            pset(x, y, self.color);
            // 少しぼかし効果: 隣接ピクセルを暗い色で
            pset(x + 1.0, y, DARK_BLUE);
            pset(x, y + 1.0, DARK_BLUE);
        }
    }
}

// ── バースト粒子 ────────────────────────────────────────────────────────────
struct Burst {
    x:    f32,
    y:    f32,
    vx:   f32,
    vy:   f32,
    life: i32,
    color: u8,
}

impl Burst {
    fn update(&mut self) {
        self.x  += self.vx;
        self.y  += self.vy;
        self.vx *= 0.97;
        self.vy *= 0.97;
        self.life -= 1;
    }
}

// ── ゲーム本体 ──────────────────────────────────────────────────────────────
struct NanoTeras {
    frame:  u32,
    rings:  Vec<Ring>,
    bursts: Vec<Burst>,
    cx: f32,
    cy: f32,
    // コアの脈動用
    pulse: f32,
}

const BURST_COLORS: [u8; 6] = [TEAL, LIGHT_BLUE, WHITE, YELLOW, LIGHT_GREEN, SKY_BLUE];

impl NanoTeras {
    fn new() -> Self {
        let cx = W as f32 / 2.0;
        let cy = H as f32 / 2.0;

        // 軌道ごとに半径・速度・色・粒子数を設定
        let rings = vec![
            Ring::new(18.0,  0.072, 0.0,                  LIGHT_BLUE,  12),
            Ring::new(32.0, -0.053, 1.0,                  TEAL,        20),
            Ring::new(46.0,  0.041, 2.0,                  SKY_BLUE,    28),
            Ring::new(62.0, -0.035, 0.5,                  LIGHT_GREEN, 36),
            Ring::new(78.0,  0.028, 1.2,                  YELLOW,      48),
            Ring::new(96.0, -0.022, 3.0,                  PINK,        60),
            Ring::new(110.0, 0.018, 0.8,                  WHITE,       72),
            // 細い速いリング (光速イメージ)
            Ring::new(25.0,  0.140, 0.0,                  WHITE,       4),
            Ring::new(55.0, -0.110, std::f32::consts::PI, YELLOW,      4),
        ];

        NanoTeras { frame: 0, rings, bursts: Vec::new(), cx, cy, pulse: 0.0 }
    }

    fn spawn_burst(&mut self, x: f32, y: f32) {
        use std::f32::consts::TAU;
        let n = 24usize;
        let step = TAU / n as f32;
        for i in 0..n {
            let angle = i as f32 * step;
            let speed = 1.2 + (i as f32 * 0.13).sin().abs() * 1.5;
            let col_idx = i % BURST_COLORS.len();
            self.bursts.push(Burst {
                x, y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                life: 40 + (i as i32 % 10),
                color: BURST_COLORS[col_idx],
            });
        }
    }

    fn update(&mut self) {
        self.frame += 1;
        self.pulse = (self.frame as f32 * 0.05).sin();

        // リング更新
        for r in &mut self.rings {
            r.update();
        }

        // バースト更新
        for b in &mut self.bursts {
            b.update();
        }
        self.bursts.retain(|b| b.life > 0);

        // クリック/タップでバースト発生
        if btnp(MOUSE_BUTTON_LEFT) {
            let mx = mouse_x() as f32;
            let my = mouse_y() as f32;
            self.spawn_burst(mx, my);
        }

        // 自動バースト (30秒ごと)
        if self.frame % 1800 == 0 || self.frame == 1 {
            self.spawn_burst(self.cx, self.cy);
        }
    }

    fn draw(&self) {
        cls(BLACK);

        // 背景: 薄い放射状グリッド
        self.draw_grid();

        // バースト粒子
        for b in &self.bursts {
            if b.life > 0 {
                pset(b.x, b.y, b.color);
            }
        }

        // 軌道リング
        for r in &self.rings {
            r.draw(self.cx, self.cy);
        }

        // 中心コア (脈動)
        let core_r = (4.0 + self.pulse * 1.5) as f32;
        circ(self.cx, self.cy, core_r + 2.0, DARK_BLUE);
        circ(self.cx, self.cy, core_r + 1.0, TEAL);
        circfill(self.cx, self.cy, core_r, LIGHT_BLUE);
        pset(self.cx, self.cy, WHITE);

        // タイトル
        text(2.0, 2.0, "NanoTerras", LIGHT_BLUE);
        text(2.0, 10.0, "TAP to BURST", DARK_GRAY);
    }

    fn draw_grid(&self) {
        // 中心から放射状に薄い線 (8方向)
        use std::f32::consts::TAU;
        let n = 8;
        let step = TAU / n as f32;
        let max_r = 130.0f32;
        for i in 0..n {
            let angle = i as f32 * step + self.frame as f32 * 0.002;
            let x2 = self.cx + max_r * angle.cos();
            let y2 = self.cy + max_r * angle.sin();
            // 点線風に3ピクセルおき
            let dx = x2 - self.cx;
            let dy = y2 - self.cy;
            let len = (dx * dx + dy * dy).sqrt();
            let steps = (len / 4.0) as usize;
            for s in 0..steps {
                let t = s as f32 * 4.0 / len;
                let px = self.cx + dx * t;
                let py = self.cy + dy * t;
                pset(px, py, DARK_BLUE);
            }
        }
    }
}

// ── エントリポイント ────────────────────────────────────────────────────────
pub fn start() {
    init(W, H, "NanoTerras", 60);

    let game = std::rc::Rc::new(std::cell::RefCell::new(NanoTeras::new()));
    let gu = std::rc::Rc::clone(&game);
    let gd = std::rc::Rc::clone(&game);

    run(
        move || gu.borrow_mut().update(),
        move || gd.borrow().draw(),
    );
}
