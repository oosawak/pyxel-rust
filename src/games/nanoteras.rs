/// NanoTerras — 光の環スピンゲーム
/// 画面をぐるぐるスワイプして回転数を稼ぎ、
/// 目標回転数に達したらCLEARED！
use crate::prelude::*;

const W: u32 = 256;
const H: u32 = 256;

const BLACK:       u8 = 0;
const DARK_BLUE:   u8 = 1;
const TEAL:        u8 = 3;
const DARK_GRAY:   u8 = 5;
const LIGHT_BLUE:  u8 = 6;
const WHITE:       u8 = 7;
const YELLOW:      u8 = 10;
const LIGHT_GREEN: u8 = 11;
const SKY_BLUE:    u8 = 12;
const PINK:        u8 = 14;

const TARGET_SPINS: f32 = 30.0;

#[derive(PartialEq)]
enum Phase { Play, Clear }

struct Ring {
    radius: f32,
    base_speed: f32,
    phase: f32,
    color: u8,
    n: usize,
}

impl Ring {
    fn new(radius: f32, speed: f32, phase: f32, color: u8, n: usize) -> Self {
        Ring { radius, base_speed: speed, phase, color, n }
    }
    fn update(&mut self, boost: f32) {
        self.phase += self.base_speed * (1.0 + boost);
    }
    fn draw(&self, cx: f32, cy: f32) {
        let step = std::f32::consts::TAU / self.n as f32;
        for i in 0..self.n {
            let angle = self.phase + i as f32 * step;
            let x = cx + self.radius * angle.cos();
            let y = cy + self.radius * angle.sin();
            pset(x, y, self.color);
            pset(x + 1.0, y, DARK_BLUE);
            pset(x, y + 1.0, DARK_BLUE);
        }
    }
}

struct Particle {
    x: f32, y: f32,
    vx: f32, vy: f32,
    life: i32,
    color: u8,
}

impl Particle {
    fn update(&mut self) {
        self.x += self.vx; self.y += self.vy;
        self.vx *= 0.97;   self.vy *= 0.97;
        self.life -= 1;
    }
}

const BURST_COLORS: [u8; 6] = [TEAL, LIGHT_BLUE, WHITE, YELLOW, LIGHT_GREEN, SKY_BLUE];

struct NanoTeras {
    frame: u32,
    phase: Phase,
    rings: Vec<Ring>,
    particles: Vec<Particle>,
    cx: f32, cy: f32,
    pulse: f32,
    spin_velocity: f32,
    total_spins: f32,
    prev_angle: Option<f32>,
    clear_timer: u32,
}

impl NanoTeras {
    fn new() -> Self {
        let cx = W as f32 / 2.0;
        let cy = H as f32 / 2.0;
        let rings = vec![
            Ring::new(18.0,  0.04,  0.0,                   LIGHT_BLUE,  12),
            Ring::new(32.0, -0.03,  1.0,                   TEAL,        20),
            Ring::new(46.0,  0.025, 2.0,                   SKY_BLUE,    28),
            Ring::new(62.0, -0.02,  0.5,                   LIGHT_GREEN, 36),
            Ring::new(78.0,  0.016, 1.2,                   YELLOW,      48),
            Ring::new(96.0, -0.013, 3.0,                   PINK,        60),
            Ring::new(110.0, 0.010, 0.8,                   WHITE,       72),
            Ring::new(25.0,  0.08,  0.0,                   WHITE,       4),
            Ring::new(55.0, -0.06,  std::f32::consts::PI,  YELLOW,      4),
        ];
        NanoTeras {
            frame: 0, phase: Phase::Play,
            rings, particles: Vec::new(),
            cx, cy, pulse: 0.0,
            spin_velocity: 0.0, total_spins: 0.0,
            prev_angle: None, clear_timer: 0,
        }
    }

    fn spawn_burst(&mut self, x: f32, y: f32, n: usize, speed: f32) {
        let step = std::f32::consts::TAU / n as f32;
        for i in 0..n {
            let angle = i as f32 * step;
            let s = speed + (i as f32 * 0.13).sin().abs() * speed * 0.5;
            self.particles.push(Particle {
                x, y,
                vx: angle.cos() * s,
                vy: angle.sin() * s,
                life: 35 + (i as i32 % 12),
                color: BURST_COLORS[i % BURST_COLORS.len()],
            });
        }
    }

    fn update(&mut self) {
        self.frame += 1;
        self.pulse = (self.frame as f32 * 0.05).sin();

        if self.phase == Phase::Clear {
            self.update_clear();
            return;
        }

        let mx = mouse_x() as f32;
        let my = mouse_y() as f32;
        let held = btn(MOUSE_BUTTON_LEFT);

        if held {
            let dx = mx - self.cx;
            let dy = my - self.cy;
            if (dx * dx + dy * dy).sqrt() > 8.0 {
                let cur = dy.atan2(dx);
                if let Some(prev) = self.prev_angle {
                    let mut delta = cur - prev;
                    if delta >  std::f32::consts::PI { delta -= std::f32::consts::TAU; }
                    if delta < -std::f32::consts::PI { delta += std::f32::consts::TAU; }
                    self.spin_velocity += delta.abs() * 1.5;
                }
                self.prev_angle = Some(cur);
            }
        } else {
            self.prev_angle = None;
        }

        self.spin_velocity *= 0.96;
        if self.spin_velocity < 0.001 { self.spin_velocity = 0.0; }

        self.total_spins += self.spin_velocity / std::f32::consts::TAU;

        let boost = (self.spin_velocity * 3.0).min(20.0);
        for r in &mut self.rings { r.update(boost); }

        if self.spin_velocity > 0.05 && self.frame % 3 == 0 {
            let angle = self.frame as f32 * 0.3;
            let r = 30.0 + (self.frame as f32 * 0.07).sin() * 70.0;
            let px = self.cx + r * angle.cos();
            let py = self.cy + r * angle.sin();
            self.particles.push(Particle {
                x: px, y: py,
                vx: angle.cos() * self.spin_velocity * 0.5,
                vy: angle.sin() * self.spin_velocity * 0.5,
                life: 20,
                color: BURST_COLORS[self.frame as usize % BURST_COLORS.len()],
            });
        }

        for p in &mut self.particles { p.update(); }
        self.particles.retain(|p| p.life > 0);

        if self.total_spins >= TARGET_SPINS {
            self.phase = Phase::Clear;
            self.clear_timer = 0;
            self.spawn_burst(self.cx, self.cy, 48, 3.0);
        }
    }

    fn update_clear(&mut self) {
        self.clear_timer += 1;
        if self.clear_timer % 40 == 0 {
            let angle = self.clear_timer as f32 * 1.1;
            let bx = self.cx + 60.0 * angle.cos();
            let by = self.cy + 60.0 * angle.sin();
            self.spawn_burst(bx, by, 16, 2.0);
        }
        for r in &mut self.rings { r.update(5.0); }
        for p in &mut self.particles { p.update(); }
        self.particles.retain(|p| p.life > 0);
    }

    fn draw(&self) {
        cls(BLACK);
        self.draw_grid();
        for p in &self.particles {
            if p.life > 0 { pset(p.x, p.y, p.color); }
        }
        for r in &self.rings { r.draw(self.cx, self.cy); }

        let core_r = 4.0 + self.pulse * 1.5;
        circ(self.cx, self.cy, core_r + 2.0, DARK_BLUE);
        circ(self.cx, self.cy, core_r + 1.0, TEAL);
        circfill(self.cx, self.cy, core_r, LIGHT_BLUE);
        pset(self.cx, self.cy, WHITE);

        match self.phase {
            Phase::Play  => self.draw_play_ui(),
            Phase::Clear => self.draw_clear_ui(),
        }
    }

    fn draw_play_ui(&self) {
        text(2.0, 2.0, "NanoTerras", LIGHT_BLUE);
        let label = format!("{}/{}", self.total_spins as u32, TARGET_SPINS as u32);
        text(2.0,  H as f32 - 20.0, "SPIN:", DARK_GRAY);
        text(28.0, H as f32 - 20.0, &label, YELLOW);

        let bar_x = 2.0f32;
        let bar_y = H as f32 - 10.0;
        let bar_w = W as f32 - 4.0;
        rect(bar_x, bar_y, bar_w, 6.0, DARK_BLUE);
        let fill = (bar_w - 2.0) * (self.total_spins / TARGET_SPINS).min(1.0);
        if fill > 0.0 {
            let col = if self.spin_velocity > 0.1 { YELLOW } else { TEAL };
            rectfill(bar_x + 1.0, bar_y + 1.0, fill, 4.0, col);
        }
        if self.spin_velocity < 0.02 {
            text(40.0, self.cy + 20.0, "SWIPE to SPIN!", DARK_GRAY);
        }
    }

    fn draw_clear_ui(&self) {
        if self.clear_timer % 8 < 6 {
            text(62.0, self.cy - 10.0, "CLEARED!", WHITE);
        }
        let msg = format!("{} SPINS!", self.total_spins as u32);
        text(72.0, self.cy + 4.0,  &msg, YELLOW);
        text(44.0, self.cy + 18.0, "GREAT WORK!", LIGHT_GREEN);
    }

    fn draw_grid(&self) {
        let max_r = 130.0f32;
        let n = if self.spin_velocity > 0.15 { 12usize } else { 8usize };
        let step = std::f32::consts::TAU / n as f32;
        for i in 0..n {
            let angle = i as f32 * step + self.frame as f32 * 0.002;
            let dx = max_r * angle.cos();
            let dy = max_r * angle.sin();
            let steps = (max_r / 4.0) as usize;
            for s in 0..steps {
                let t = s as f32 * 4.0 / max_r;
                pset(self.cx + dx * t, self.cy + dy * t, DARK_BLUE);
            }
        }
    }
}

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
