/// Arisa Quest — RPG battle demo
/// Turn-based battle system with map exploration
use pyxel_rust::prelude::*;
use std::collections::VecDeque;

const W: u32 = 160;
const H: u32 = 120;
const TILE: i32 = 8;
const MAP_W: usize = 20;
const MAP_H: usize = 15;

// ── Palette ──────────────────────────────────────────────────────────────────
const BLACK:      u8 = 0;
const DARK_BLUE:  u8 = 1;
const DARK_PRP:   u8 = 2;
const DARK_GRN:   u8 = 3;
const BROWN:      u8 = 4;
const DARK_GRAY:  u8 = 5;
const LIGHT_GRAY: u8 = 6;
const WHITE:      u8 = 7;
const RED:        u8 = 8;
const ORANGE:     u8 = 9;
const YELLOW:     u8 = 10;
const GREEN:      u8 = 11;
const BLUE:       u8 = 12;
const INDIGO:     u8 = 13;
const PINK:       u8 = 14;

// ── Game states ──────────────────────────────────────────────────────────────
#[derive(PartialEq, Clone, Copy)]
enum GameState { Title, Map, Battle }

#[derive(PartialEq, Clone, Copy)]
enum BattlePhase {
    SelectCmd,
    ShowMsg,
    Victory,
    Defeat,
    Fled,
}

// ── Enemy definitions ────────────────────────────────────────────────────────
struct EnemyDef {
    name:  &'static str,
    hp:    i32,
    atk:   i32,
    exp:   i32,
    col:   u8,
    col2:  u8,
}

const ENEMIES: [EnemyDef; 5] = [
    EnemyDef { name: "スライム",        hp: 20, atk: 4,  exp: 10, col: BLUE,      col2: WHITE      },
    EnemyDef { name: "シャドウキャット", hp: 35, atk: 7,  exp: 25, col: DARK_PRP,  col2: PINK       },
    EnemyDef { name: "フレイムインプ",   hp: 28, atk: 9,  exp: 20, col: RED,       col2: ORANGE     },
    EnemyDef { name: "ゴースト",         hp: 40, atk: 6,  exp: 30, col: INDIGO,    col2: LIGHT_GRAY },
    EnemyDef { name: "ロックゴーレム",   hp: 60, atk: 11, exp: 50, col: DARK_GRAY, col2: LIGHT_GRAY },
];

// ── Enemy ────────────────────────────────────────────────────────────────────
struct Enemy {
    idx:   usize,
    name:  String,
    hp:    i32,
    max_hp: i32,
    atk:   i32,
    exp:   i32,
    col:   u8,
    col2:  u8,
    timer: i32,
    flash: i32,
}

impl Enemy {
    fn new(idx: usize) -> Self {
        let d = &ENEMIES[idx];
        Enemy {
            idx,
            name: d.name.to_string(),
            hp: d.hp, max_hp: d.hp,
            atk: d.atk, exp: d.exp,
            col: d.col, col2: d.col2,
            timer: 0, flash: 0,
        }
    }
    fn alive(&self) -> bool { self.hp > 0 }

    fn draw(&self, x: f32, y: f32) {
        if self.flash > 0 && self.flash % 4 < 2 { return; }
        match self.idx {
            0 => draw_slime(x, y, self.col, self.col2, self.timer),
            1 => draw_shadow_cat(x, y, self.col, self.col2, self.timer),
            2 => draw_flame_imp(x, y, self.col, self.col2, self.timer),
            3 => draw_ghost(x, y, self.col, self.col2, self.timer),
            _ => draw_golem(x, y, self.col, self.col2, self.timer),
        }
    }
}

// ── Sprite drawing functions ─────────────────────────────────────────────────
fn draw_slime(x: f32, y: f32, col: u8, col2: u8, t: i32) {
    let bob = if (t / 15) % 2 == 0 { 0.0 } else { 1.0 };
    circfill(x, y + bob, 10.0, col);
    pset(x - 3.0, y - 2.0 + bob, BLACK);
    pset(x + 3.0, y - 2.0 + bob, BLACK);
    pset(x - 4.0, y - 5.0 + bob, col2);
    rectfill(x - 9.0, y + 8.0 + bob, 18.0, 3.0, col);
}

fn draw_shadow_cat(x: f32, y: f32, col: u8, col2: u8, t: i32) {
    let tail = if (t / 10) % 2 == 0 { 0.0 } else { 2.0 };
    rectfill(x - 7.0, y - 4.0, 14.0, 12.0, col);
    circfill(x, y - 8.0, 7.0, col);
    rectfill(x - 6.0, y - 14.0, 3.0, 5.0, col);
    rectfill(x + 3.0, y - 14.0, 3.0, 5.0, col);
    pset(x - 3.0, y - 8.0, col2);
    pset(x + 3.0, y - 8.0, col2);
    line(x + 7.0, y + 4.0, x + 14.0, y + tail, col);
}

fn draw_flame_imp(x: f32, y: f32, col: u8, col2: u8, t: i32) {
    let fl = if (t / 8) % 2 == 0 { 0.0 } else { 1.0 };
    rectfill(x - 14.0, y - 6.0, 8.0, 10.0, DARK_GRAY);
    rectfill(x + 6.0,  y - 6.0, 8.0, 10.0, DARK_GRAY);
    rectfill(x - 5.0, y - 4.0, 10.0, 12.0, col);
    circfill(x, y - 8.0, 6.0, col);
    for i in 0i32..3 {
        let fx = x - 4.0 + i as f32 * 4.0;
        pset(fx, y - 13.0 - fl, col2);
        pset(fx, y - 12.0,      col2);
    }
    pset(x - 2.0, y - 8.0, col2);
    pset(x + 2.0, y - 8.0, col2);
}

fn draw_ghost(x: f32, y: f32, col: u8, col2: u8, t: i32) {
    let fl = (t as f32 * 0.1).sin() * 2.0;
    circfill(x, y - 4.0 + fl, 9.0, col);
    rectfill(x - 9.0, y - 4.0 + fl, 18.0, 10.0, col);
    for i in 0i32..3 {
        pset(x - 6.0 + i as f32 * 6.0, y + 6.0 + fl, BLACK);
    }
    pset(x - 3.0, y - 4.0 + fl, col2);
    pset(x + 3.0, y - 4.0 + fl, col2);
}

fn draw_golem(x: f32, y: f32, col: u8, col2: u8, t: i32) {
    let s = if (t / 20) % 2 == 0 { 0.0 } else { 1.0 };
    rectfill(x - 7.0, y + 6.0 + s,  6.0, 6.0, col);
    rectfill(x + 1.0, y + 6.0 - s,  6.0, 6.0, col);
    rectfill(x - 8.0, y - 6.0, 16.0, 14.0, col);
    rectfill(x - 14.0, y - 4.0, 6.0, 8.0, col);
    rectfill(x + 8.0,  y - 4.0, 6.0, 8.0, col);
    rectfill(x - 7.0, y - 15.0, 14.0, 10.0, col);
    rectfill(x - 4.0, y - 12.0, 3.0, 3.0, col2);
    rectfill(x + 1.0, y - 12.0, 3.0, 3.0, col2);
    line(x - 2.0, y - 2.0, x + 2.0, y + 2.0, col2);
}

fn draw_arisa(x: f32, y: f32, t: i32) {
    let bob = if (t / 20) % 2 == 0 { 0.0 } else { 1.0 };
    // Cat ears
    rectfill(x - 5.0, y - 18.0 + bob, 3.0, 4.0, PINK);
    rectfill(x + 2.0, y - 18.0 + bob, 3.0, 4.0, PINK);
    // Helmet
    circfill(x, y - 13.0 + bob, 7.0, BLUE);
    // Visor
    rectfill(x - 4.0, y - 15.0 + bob, 8.0, 4.0, INDIGO);
    pset(x - 2.0, y - 14.0 + bob, BLUE);
    pset(x + 2.0, y - 14.0 + bob, BLUE);
    // Body
    rectfill(x - 5.0, y - 6.0 + bob, 10.0, 10.0, BLUE);
    rectfill(x + 5.0, y - 5.0 + bob,  4.0,  8.0, DARK_BLUE);
    // Arms
    rectfill(x - 9.0, y - 4.0 + bob, 4.0, 6.0, BLUE);
    rectfill(x + 5.0, y - 4.0 + bob, 4.0, 6.0, BLUE);
    // Legs
    rectfill(x - 5.0, y + 4.0 + bob, 4.0, 6.0, DARK_BLUE);
    rectfill(x + 1.0, y + 4.0 + bob, 4.0, 6.0, DARK_BLUE);
    // Tail
    line(x - 5.0, y + 6.0 + bob, x - 10.0, y + 2.0 + bob, PINK);
    line(x - 10.0, y + 2.0 + bob, x - 12.0, y + 4.0 + bob, PINK);
}

// ── Player ───────────────────────────────────────────────────────────────────
struct Player {
    hp: i32, max_hp: i32,
    mp: i32, max_mp: i32,
    lv: i32, exp: i32,
    atk: i32,
    map_x: i32, map_y: i32,
    flash: i32, timer: i32,
}

impl Player {
    fn new() -> Self {
        Player {
            hp: 128, max_hp: 128,
            mp: 54,  max_mp: 54,
            lv: 18,  exp: 0,
            atk: 15,
            map_x: 10, map_y: 7,
            flash: 0,  timer: 0,
        }
    }
    fn draw_battle(&self, x: f32, y: f32) {
        if self.flash > 0 && self.flash % 4 < 2 { return; }
        draw_arisa(x, y, self.timer);
    }
    fn draw_map(&self) {
        let px = self.map_x as f32 * TILE as f32;
        let py = self.map_y as f32 * TILE as f32;
        circfill(px + 4.0, py + 4.0, 3.0, PINK);
        pset(px + 4.0, py + 2.0, WHITE);
    }
}

// ── Map data ─────────────────────────────────────────────────────────────────
// 0=grass 1=wall 2=tree
const MAP_DATA: [[u8; MAP_W]; MAP_H] = [
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,2,2,0,0,0,0,0,0,0,2,2,0,0,0,0,0,1],
    [1,0,2,2,2,0,0,0,0,0,0,2,2,2,0,0,0,0,0,1],
    [1,0,0,2,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,2,2,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,2,2,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,1],
    [1,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,1],
    [1,0,2,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,1],
    [1,0,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
];

fn draw_map_tiles() {
    for y in 0..MAP_H {
        for x in 0..MAP_W {
            let px = x as f32 * TILE as f32;
            let py = y as f32 * TILE as f32;
            match MAP_DATA[y][x] {
                1 => {
                    rectfill(px, py, TILE as f32, TILE as f32, DARK_GRAY);
                }
                2 => {
                    rectfill(px, py, TILE as f32, TILE as f32, DARK_GRN);
                    rectfill(px + 3.0, py + 5.0, 2.0, 3.0, BROWN);
                    circfill(px + 4.0, py + 3.0, 3.0, GREEN);
                }
                _ => {
                    rectfill(px, py, TILE as f32, TILE as f32, DARK_GRN);
                    pset(px + 1.0, py + 1.0, GREEN);
                    pset(px + 5.0, py + 3.0, GREEN);
                    pset(px + 3.0, py + 6.0, GREEN);
                }
            }
        }
    }
}

// ── Battle background ─────────────────────────────────────────────────────────
fn draw_battle_bg(t: i32) {
    // Night sky
    rectfill(0.0, 0.0, W as f32, 55.0, DARK_BLUE);
    // Moon
    circfill(140.0, 12.0, 7.0, YELLOW);
    circfill(143.0, 10.0, 5.0, DARK_BLUE); // crescent
    // Stars (twinkling)
    let stars: &[(f32, f32)] = &[
        (10.0,5.0),(30.0,12.0),(55.0,8.0),(80.0,3.0),(110.0,18.0),(25.0,20.0),(70.0,25.0),
    ];
    for (i, &(sx, sy)) in stars.iter().enumerate() {
        if (t / 20 + i as i32) % 2 == 0 {
            pset(sx, sy, WHITE);
        } else {
            pset(sx, sy, LIGHT_GRAY);
        }
    }
    // Background trees
    for i in 0i32..5 {
        let tx = 8.0 + i as f32 * 32.0;
        rectfill(tx + 4.0, 28.0, 5.0, 28.0, BROWN);
        circfill(tx + 6.0, 25.0, 9.0, DARK_GRN);
        circfill(tx + 4.0, 22.0, 6.0, GREEN);
    }
    // Ground
    rectfill(0.0, 55.0, W as f32, 30.0, DARK_GRN);
    rectfill(0.0, 55.0, W as f32, 3.0, GREEN);
}

// ── Battle UI ────────────────────────────────────────────────────────────────
const CMDS: [&str; 4] = ["たたかう", "スキル", "どうぐ", "にげる"];

fn draw_battle_ui(player: &Player, cursor: usize) {
    // Bottom panel
    rectfill(0.0, 85.0, W as f32, 35.0, DARK_BLUE);
    rect(0.0, 85.0, W as f32, 35.0, WHITE);
    line(80.0, 85.0, 80.0, 120.0, WHITE);

    // Player status (left)
    text(2.0, 87.0, "HP", YELLOW);
    let hp_w = 40.0 * player.hp as f32 / player.max_hp as f32;
    rectfill(14.0, 87.0, 40.0, 4.0, DARK_GRAY);
    rectfill(14.0, 87.0, hp_w, 4.0, GREEN);
    text(2.0, 93.0, &format!("{}/{}", player.hp, player.max_hp), WHITE);

    text(2.0, 100.0, "MP", BLUE);
    let mp_w = 40.0 * player.mp as f32 / player.max_mp as f32;
    rectfill(14.0, 100.0, 40.0, 4.0, DARK_GRAY);
    rectfill(14.0, 100.0, mp_w, 4.0, BLUE);
    text(2.0, 106.0, &format!("{}/{}", player.mp, player.max_mp), WHITE);

    text(2.0, 113.0, &format!("Lv{}", player.lv), YELLOW);

    // Commands (right 2x2 grid)
    for (i, &cmd) in CMDS.iter().enumerate() {
        let col = if i % 2 == 0 { 83.0 } else { 122.0 };
        let row = 89.0 + (i / 2) as f32 * 14.0;
        if i == cursor {
            rectfill(col - 1.0, row - 1.0, 36.0, 11.0, INDIGO);
            text(col + 1.0, row + 1.0, cmd, YELLOW);
        } else {
            text(col + 1.0, row + 1.0, cmd, WHITE);
        }
    }
}

fn draw_enemy_hud(enemy: &Enemy) {
    let pct = enemy.hp as f32 / enemy.max_hp as f32;
    text(2.0, 2.0, &enemy.name, WHITE);
    rectfill(2.0, 10.0, 60.0, 4.0, DARK_GRAY);
    let bar_col = if pct > 0.5 { GREEN } else if pct > 0.25 { YELLOW } else { RED };
    rectfill(2.0, 10.0, 60.0 * pct, 4.0, bar_col);
    text(2.0, 16.0, &format!("{}/{}", enemy.hp, enemy.max_hp), LIGHT_GRAY);
}

fn draw_msg_box(msg: &str) {
    rectfill(0.0, 68.0, W as f32, 17.0, BLACK);
    rect(0.0, 68.0, W as f32, 17.0, WHITE);
    text(4.0, 74.0, msg, WHITE);
}

// ── Game ─────────────────────────────────────────────────────────────────────
struct Game {
    state:   GameState,
    player:  Player,
    enemy:   Option<Enemy>,
    phase:   BattlePhase,
    cursor:  usize,
    msgs:    VecDeque<String>,
    cur_msg: String,
    msg_t:   i32,
    timer:   i32,
    enc_t:   i32,
}

impl Game {
    fn new() -> Self {
        Game {
            state:   GameState::Title,
            player:  Player::new(),
            enemy:   None,
            phase:   BattlePhase::SelectCmd,
            cursor:  0,
            msgs:    VecDeque::new(),
            cur_msg: String::new(),
            msg_t:   0,
            timer:   0,
            enc_t:   0,
        }
    }

    fn start_battle(&mut self, idx: usize) {
        let enemy = Enemy::new(idx);
        self.cur_msg = format!("{}があらわれた！", enemy.name);
        self.enemy = Some(enemy);
        self.phase = BattlePhase::ShowMsg;
        self.msg_t = 90;
        self.cursor = 0;
        self.state = GameState::Battle;
    }

    fn push_msg(&mut self, s: &str) {
        self.msgs.push_back(s.to_string());
    }

    fn next_msg(&mut self) {
        if let Some(m) = self.msgs.pop_front() {
            self.cur_msg = m;
            self.msg_t   = 90;
            self.phase   = BattlePhase::ShowMsg;
        } else {
            let dead_enemy = self.enemy.as_ref().map_or(false, |e| !e.alive());
            let dead_player = self.player.hp <= 0;
            if dead_enemy {
                self.phase = BattlePhase::Victory;
            } else if dead_player {
                self.phase = BattlePhase::Defeat;
            } else {
                self.phase = BattlePhase::SelectCmd;
            }
        }
    }

    // ── Update ──────────────────────────────────────────────────────────────
    fn update(&mut self) {
        self.timer += 1;
        self.player.timer += 1;
        if let Some(ref mut e) = self.enemy {
            e.timer += 1;
            if e.flash > 0 { e.flash -= 1; }
        }
        if self.player.flash > 0 { self.player.flash -= 1; }

        match self.state {
            GameState::Title  => self.upd_title(),
            GameState::Map    => self.upd_map(),
            GameState::Battle => self.upd_battle(),
        }
    }

    fn upd_title(&mut self) {
        if btnp(KEY_Z) || btnp(KEY_RETURN) || btnp(KEY_SPACE) {
            self.state = GameState::Map;
        }
    }

    fn upd_map(&mut self) {
        self.enc_t += 1;
        let (mut nx, mut ny) = (self.player.map_x, self.player.map_y);
        let mut moved = false;

        if btnp_hold(KEY_UP,    10, 4) { ny -= 1; moved = true; }
        if btnp_hold(KEY_DOWN,  10, 4) { ny += 1; moved = true; }
        if btnp_hold(KEY_LEFT,  10, 4) { nx -= 1; moved = true; }
        if btnp_hold(KEY_RIGHT, 10, 4) { nx += 1; moved = true; }

        if moved {
            let cx = nx.clamp(0, MAP_W as i32 - 1) as usize;
            let cy = ny.clamp(0, MAP_H as i32 - 1) as usize;
            if MAP_DATA[cy][cx] == 0 {
                self.player.map_x = cx as i32;
                self.player.map_y = cy as i32;
                if self.enc_t > 30 && rnd(0.0, None) < 0.2 {
                    self.enc_t = 0;
                    let idx = rnd_int(0, Some(4)) as usize;
                    self.start_battle(idx);
                }
            }
        }
    }

    fn upd_battle(&mut self) {
        match self.phase {
            BattlePhase::SelectCmd => {
                if btnp(KEY_UP)    && self.cursor >= 2 { self.cursor -= 2; }
                if btnp(KEY_DOWN)  && self.cursor < 2  { self.cursor += 2; }
                if btnp(KEY_LEFT)  && self.cursor % 2 == 1 { self.cursor -= 1; }
                if btnp(KEY_RIGHT) && self.cursor % 2 == 0 { self.cursor += 1; }

                if btnp(KEY_Z) || btnp(KEY_RETURN) {
                    match self.cursor {
                        0 => self.do_attack(false),
                        1 => self.do_attack(true),
                        2 => self.do_item(),
                        _ => self.do_run(),
                    }
                }
            }
            BattlePhase::ShowMsg => {
                self.msg_t -= 1;
                if self.msg_t <= 0 || btnp(KEY_Z) || btnp(KEY_RETURN) {
                    self.next_msg();
                }
            }
            BattlePhase::Victory | BattlePhase::Defeat | BattlePhase::Fled => {
                if btnp(KEY_Z) || btnp(KEY_RETURN) {
                    if self.phase == BattlePhase::Defeat {
                        self.player.hp = self.player.max_hp;
                        self.player.mp = self.player.max_mp;
                    }
                    self.enemy = None;
                    self.state = GameState::Map;
                }
            }
        }
    }

    fn do_attack(&mut self, skill: bool) {
        if skill {
            if self.player.mp >= 10 {
                self.player.mp -= 10;
                let dmg = self.player.atk * 2 + rnd_int(0, Some(5));
                self.push_msg("アリサのキャットビーム！");
                self.apply_enemy_dmg(dmg);
            } else {
                self.push_msg("MPが足りない！");
                self.next_msg();
                return;
            }
        } else {
            let dmg = (self.player.atk + rnd_int(-2, Some(5))).max(1);
            self.push_msg("アリサのこうげき！");
            self.apply_enemy_dmg(dmg);
        }

        // Enemy counter attack
        if self.enemy.as_ref().map_or(false, |e| e.alive()) {
            self.do_enemy_turn();
        }
        self.next_msg();
    }

    fn apply_enemy_dmg(&mut self, dmg: i32) {
        if let Some(ref mut e) = self.enemy {
            e.hp = (e.hp - dmg).max(0);
            e.flash = 20;
            let msg = format!("{}ダメージ！", dmg);
            self.msgs.push_back(msg);
            if !e.alive() {
                let name = e.name.clone();
                let exp  = e.exp;
                self.msgs.push_back(format!("{}を倒した！", name));
                self.msgs.push_back(format!("{}EXPを得た！", exp));
                self.player.exp += exp;
            }
        }
    }

    fn do_item(&mut self) {
        let heal = 30.min(self.player.max_hp - self.player.hp);
        self.player.hp += heal;
        self.push_msg(&format!("ポーションを使った！HP+{}", heal));
        self.do_enemy_turn();
        self.next_msg();
    }

    fn do_run(&mut self) {
        if rnd(0.0, None) < 0.5 {
            self.cur_msg = "うまく逃げられた！".to_string();
            self.msg_t   = 90;
            self.phase   = BattlePhase::Fled;
        } else {
            self.push_msg("逃げられなかった！");
            self.do_enemy_turn();
            self.next_msg();
        }
    }

    fn do_enemy_turn(&mut self) {
        // Extract values without holding borrow
        let (atk, name) = match self.enemy.as_ref() {
            Some(e) => (e.atk, e.name.clone()),
            None    => return,
        };
        let dmg = (atk + rnd_int(-2, Some(4))).max(1);
        self.player.hp    = (self.player.hp - dmg).max(0);
        self.player.flash = 20;
        self.msgs.push_back(format!("{}のこうげき！", name));
        self.msgs.push_back(format!("{}ダメージ！", dmg));
        if self.player.hp <= 0 {
            self.msgs.push_back("アリサはたおれた...".to_string());
        }
    }

    // ── Draw ────────────────────────────────────────────────────────────────
    fn draw(&self) {
        match self.state {
            GameState::Title  => self.draw_title(),
            GameState::Map    => self.draw_map(),
            GameState::Battle => self.draw_battle(),
        }
    }

    fn draw_title(&self) {
        cls(BLACK);
        // Stars
        for i in 0u32..25 {
            let sx = (i * 37 % W) as f32;
            let sy = (i * 19 % 70) as f32;
            let bright = (self.timer / 30 + i as i32) % 2 == 0;
            pset(sx, sy, if bright { WHITE } else { DARK_BLUE });
        }
        // Moon
        circfill(140.0, 15.0, 8.0, YELLOW);
        circfill(143.0, 13.0, 6.0, BLACK);

        // Title text
        text(38.0, 22.0, "ARISA QUEST", YELLOW);
        text(34.0, 31.0, "アリサクエスト", PINK);

        // Characters
        draw_arisa(100.0, 70.0, self.timer);
        draw_slime(45.0, 72.0, BLUE, WHITE, self.timer);

        // Prompt
        if (self.timer / 30) % 2 == 0 {
            text(28.0, 100.0, "Zキーでスタート！", WHITE);
        }
        text(10.0, 110.0, "2026 ARISA QUEST", DARK_GRAY);
    }

    fn draw_map(&self) {
        cls(DARK_GRN);
        draw_map_tiles();
        self.player.draw_map();
        // HUD
        rectfill(0.0, 0.0, W as f32, 8.0, BLACK);
        text(2.0, 1.0, "アリサクエスト", WHITE);
        text(100.0, 1.0, &format!("Lv{} HP{}", self.player.lv, self.player.hp), YELLOW);
    }

    fn draw_battle(&self) {
        draw_battle_bg(self.timer);

        // Enemy
        if let Some(ref e) = self.enemy {
            e.draw(45.0, 48.0);
            draw_enemy_hud(e);
        }

        // Player (Arisa on right side)
        self.player.draw_battle(118.0, 58.0);

        match self.phase {
            BattlePhase::SelectCmd => {
                draw_battle_ui(&self.player, self.cursor);
            }
            BattlePhase::Victory => {
                draw_battle_ui(&self.player, self.cursor);
                rectfill(20.0, 42.0, 120.0, 22.0, BLACK);
                rect(20.0, 42.0, 120.0, 22.0, YELLOW);
                text(38.0, 47.0, "★ しょうり！ ★", YELLOW);
                text(28.0, 57.0, "Zキーでもどる", WHITE);
            }
            BattlePhase::Defeat => {
                draw_battle_ui(&self.player, self.cursor);
                rectfill(20.0, 42.0, 120.0, 22.0, BLACK);
                rect(20.0, 42.0, 120.0, 22.0, RED);
                text(48.0, 47.0, "GAME OVER", RED);
                text(28.0, 57.0, "Zキーでもどる", WHITE);
            }
            BattlePhase::Fled => {
                draw_battle_ui(&self.player, self.cursor);
                draw_msg_box(&self.cur_msg);
            }
            BattlePhase::ShowMsg => {
                draw_battle_ui(&self.player, self.cursor);
                draw_msg_box(&self.cur_msg);
            }
        }
    }
}

// ── Main ─────────────────────────────────────────────────────────────────────
fn main() {
    init(W, H, "Arisa Quest", 60);

    let game = std::rc::Rc::new(std::cell::RefCell::new(Game::new()));
    let gu = std::rc::Rc::clone(&game);
    let gd = std::rc::Rc::clone(&game);

    run(
        move || gu.borrow_mut().update(),
        move || gd.borrow().draw(),
    );
}
