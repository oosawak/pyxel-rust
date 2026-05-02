use pyxel_rust::prelude::*;

const W: u32 = 128;
const H: u32 = 128;
const TILE_SIZE: u32 = 8;

// Colors
const COLOR_BG: u32 = 0;
const COLOR_WALL: u32 = 1;
const COLOR_PLAYER_READY: u32 = 7;
const COLOR_PLAYER_SPENT: u32 = 14;
const COLOR_PARTICLE: u32 = 12;
const COLOR_ORB: u32 = 10;
const COLOR_SPIKE: u32 = 7;

// Game states
const STATE_PLAY: u32 = 1;
const STATE_BOSS: u32 = 2;
const STATE_GAMECLEAR: u32 = 3;

// ============================================================================
// Particle
// ============================================================================

#[derive(Clone, Copy)]
struct Particle {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    col: u32,
    life: i32,
}

impl Particle {
    fn new(x: f32, y: f32, dx: f32, dy: f32, col: u32, life: i32) -> Self {
        Particle { x, y, dx, dy, col, life }
    }

    fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.life -= 1;
    }

    fn draw(&self) {
        pset(self.x, self.y, self.col as u8);
    }

    fn is_alive(&self) -> bool {
        self.life > 0
    }
}

// ============================================================================
// Room (Tilemap)
// ============================================================================

struct Room {
    tiles: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Room {
    fn new(width: usize, height: usize) -> Self {
        Room {
            tiles: vec![vec![0; width]; height],
            width,
            height,
        }
    }

    fn set_tile(&mut self, x: usize, y: usize, tile: u32) {
        if x < self.width && y < self.height {
            self.tiles[y][x] = tile;
        }
    }

    fn get_tile(&self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            self.tiles[y][x]
        } else {
            1 // Wall outside bounds
        }
    }

    fn is_solid(&self, x: usize, y: usize) -> bool {
        self.get_tile(x, y) == 1
    }

    fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.get_tile(x, y);
                let px = (x as u32 * TILE_SIZE) as f32;
                let py = (y as u32 * TILE_SIZE) as f32;
                
                match tile {
                    0 => {} // Empty
                    1 => rectfill(px, py, TILE_SIZE as f32, TILE_SIZE as f32, COLOR_WALL as u8),
                    _ => {} // Other tiles
                }
            }
        }
    }
}

// ============================================================================
// Player
// ============================================================================

struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    width: f32,
    height: f32,
    is_on_ground: bool,
    is_on_wall: i32,
    can_dash: bool,
    dash_time: i32,
    dash_dir: (f32, f32),
    coyote_timer: i32,
    jump_buffer: i32,
    stretch_x: f32,
    stretch_y: f32,
    facing: i32,
    is_dead: bool,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            width: 6.0,
            height: 6.0,
            is_on_ground: false,
            is_on_wall: 0,
            can_dash: true,
            dash_time: 0,
            dash_dir: (0.0, 0.0),
            coyote_timer: 0,
            jump_buffer: 0,
            stretch_x: 1.0,
            stretch_y: 1.0,
            facing: 1,
            is_dead: false,
        }
    }

    fn is_wall(&self, x: f32, y: f32, room: &Room) -> bool {
        let x1 = (x / TILE_SIZE as f32).floor() as i32;
        let y1 = (y / TILE_SIZE as f32).floor() as i32;
        let x2 = ((x + self.width - 0.1) / TILE_SIZE as f32).floor() as i32;
        let y2 = ((y + self.height - 0.1) / TILE_SIZE as f32).floor() as i32;

        for ty in y1..=y2 {
            if ty < 0 || ty >= 16 {
                continue;
            }
            for tx in x1..=x2 {
                if tx < 0 || tx >= 16 {
                    continue;
                }
                if room.is_solid(tx as usize, ty as usize) {
                    return true;
                }
            }
        }
        false
    }

    fn get_input(&self) -> (i32, i32) {
        let mut dx = 0i32;
        let mut dy = 0i32;

        if btn(KEY_LEFT) || btn(KEY_A) {
            dx -= 1;
        }
        if btn(KEY_RIGHT) || btn(KEY_D) {
            dx += 1;
        }
        if btn(KEY_UP) || btn(KEY_W) {
            dy -= 1;
        }
        if btn(KEY_DOWN) || btn(KEY_S) {
            dy += 1;
        }

        (dx, dy)
    }

    fn update(&mut self, room: &Room, particles: &mut Vec<Particle>) {
        // Timers
        if self.coyote_timer > 0 {
            self.coyote_timer -= 1;
        }
        if self.jump_buffer > 0 {
            self.jump_buffer -= 1;
        }

        // Dash logic
        if self.dash_time > 0 {
            self.vx = self.dash_dir.0 * 5.0;
            self.vy = self.dash_dir.1 * 5.0;
            self.dash_time -= 1;

            particles.push(Particle::new(
                self.x + 3.0,
                self.y + 3.0,
                rnd(-1.0, Some(1.0)),
                rnd(-1.0, Some(1.0)),
                COLOR_PARTICLE,
                10,
            ));

            if self.dash_time == 0 {
                self.vx *= 0.5;
                self.vy *= 0.5;
            }
        } else {
            // Horizontal movement
            let (dx, _dy) = self.get_input();

            if dx != 0 {
                let target_vx = dx as f32 * 2.5;
                self.vx += (target_vx - self.vx) * 0.2;
                self.facing = dx;
            } else {
                self.vx *= 0.7;
            }

            // Gravity
            let grav = if (btn(KEY_SPACE) || btn(KEY_C)) && self.vy < 0.0 {
                0.3
            } else {
                0.5
            };

            if self.is_on_wall != 0 && self.vy > 0.0 {
                self.vy = clamp(self.vy + 0.1, 0.0, 0.8); // Wall slide
            } else {
                self.vy += grav;
            }

            // Jump input buffer
            if btnp(KEY_SPACE) || btnp(KEY_Z) || btnp(KEY_W) || btnp(KEY_UP) || btnp(KEY_C) {
                self.jump_buffer = 4;
            }

            // Jump logic
            if self.jump_buffer > 0 {
                if self.coyote_timer > 0 {
                    self.vy = -4.5;
                    self.stretch_x = 0.6;
                    self.stretch_y = 1.4;
                    self.coyote_timer = 0;
                    self.jump_buffer = 0;
                    play(3, 0, None, None, None); // Jump sound
                } else if self.is_on_wall != 0 {
                    self.vy = -4.2;
                    self.vx = -(self.is_on_wall as f32) * 3.5;
                    self.stretch_x = 0.6;
                    self.stretch_y = 1.4;
                    self.jump_buffer = 0;
                    play(3, 0, None, None, None); // Jump sound
                }
            }

            // Dash input
            if (btnp(KEY_X) || btnp(KEY_V) || btnp(KEY_LCTRL) || btnp(KEY_RCTRL)) && self.can_dash {
                let (idx, idy) = self.get_input();
                let direction_x = if idx == 0 { self.facing as f32 } else { idx as f32 };
                let direction_y = idy as f32;

                let mag = (direction_x * direction_x + direction_y * direction_y).sqrt();
                if mag > 0.0 {
                    self.dash_dir = (direction_x / mag, direction_y / mag);
                } else {
                    self.dash_dir = (self.facing as f32, 0.0);
                }

                self.dash_time = 6;
                self.can_dash = false;
                self.vy = 0.0;
                play(3, 1, None, None, None); // Dash sound
            }
        }

        // Collision & Movement (Axis Separated)
        // Move X
        let steps_x = ((self.vx.abs() / 0.5) as i32 + 1).max(1);
        let step_x = self.vx / steps_x as f32;
        for _ in 0..steps_x {
            if !self.is_wall(self.x + step_x, self.y, room) {
                self.x += step_x;
            } else {
                self.vx = 0.0;
                break;
            }
        }

        // Move Y
        let steps_y = ((self.vy.abs() / 0.5) as i32 + 1).max(1);
        let step_y = self.vy / steps_y as f32;
        for _ in 0..steps_y {
            if !self.is_wall(self.x, self.y + step_y, room) {
                self.y += step_y;
            } else {
                self.vy = 0.0;
                break;
            }
        }

        // Ground detection
        self.is_on_ground = self.is_wall(self.x, self.y + self.height + 0.1, room);
        if self.is_on_ground {
            self.coyote_timer = 6;
            self.stretch_y += (1.0 - self.stretch_y) * 0.1;
        } else {
            self.stretch_y += (1.0 - self.stretch_y) * 0.05;
        }

        self.stretch_x += (1.0 - self.stretch_x) * 0.1;

        // Wall detection
        let left_wall = self.is_wall(self.x - 0.1, self.y, room);
        let right_wall = self.is_wall(self.x + self.width + 0.1, self.y, room);

        self.is_on_wall = if left_wall { -1 } else if right_wall { 1 } else { 0 };

        if self.is_on_ground || self.is_on_wall != 0 {
            self.can_dash = true;
        }
    }

    fn draw(&self) {
        let x = self.x;
        let y = self.y;
        let w = self.width * self.stretch_x;
        let h = self.height * self.stretch_y;

        let color = if self.can_dash {
            COLOR_PLAYER_READY
        } else {
            COLOR_PLAYER_SPENT
        };

        rectfill(
            x - (w - self.width) / 2.0,
            y - (h - self.height) / 2.0,
            w,
            h,
            color as u8,
        );
    }
}

// ============================================================================
// Boss
// ============================================================================

struct Boss {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Boss {
    fn new(x: f32, y: f32) -> Self {
        Boss {
            x,
            y,
            width: 8.0,
            height: 8.0,
        }
    }

    fn update(&mut self) {
        // Simple AI - move towards center
        if self.x < W as f32 / 2.0 {
            self.x += 1.0;
        } else if self.x > W as f32 / 2.0 {
            self.x -= 1.0;
        }
    }

    fn draw(&self) {
        circ(self.x, self.y, 4.0, 8u8);
    }
}

// ============================================================================
// Game
// ============================================================================

struct Game {
    player: Player,
    boss: Boss,
    particles: Vec<Particle>,
    room: Room,
    state: u32,
    collected_orbs: u32,
    boss_countdown: i32,
}

impl Game {
    fn new() -> Self {
        let mut game = Game {
            player: Player::new((W / 2) as f32, (H / 2) as f32),
            boss: Boss::new(-100.0, -100.0),
            particles: Vec::new(),
            room: Room::new(16, 16),
            state: STATE_PLAY,
            collected_orbs: 0,
            boss_countdown: 0,
        };
        game.generate_room();
        game
    }

    fn generate_room(&mut self) {
        // Simple room generation
        for y in 0..16 {
            for x in 0..16 {
                let tile = if x == 0 || x == 15 || y == 0 || y == 15 {
                    1 // Walls on edges
                } else {
                    0 // Empty
                };
                self.room.set_tile(x, y, tile);
            }
        }
        
        // Add some random walls
        for _ in 0..10 {
            let x = (rnd(0.0, Some(14.0)) as usize).clamp(1, 14);
            let y = (rnd(0.0, Some(14.0)) as usize).clamp(1, 14);
            self.room.set_tile(x, y, 1);
        }
    }

    fn update(&mut self) {
        match self.state {
            STATE_PLAY => {
                self.player.update(&self.room, &mut self.particles);
                self.particles.retain_mut(|p| {
                    p.update();
                    p.is_alive()
                });
            }
            STATE_BOSS => {
                self.player.update(&self.room, &mut self.particles);
                self.boss.update();
            }
            _ => {}
        }
    }

    fn draw(&self) {
        cls(COLOR_BG as u8);
        
        self.room.draw();
        
        match self.state {
            STATE_PLAY => {
                self.player.draw();
                
                for particle in &self.particles {
                    particle.draw();
                }
                
                text(5.0, 5.0, &format!("Orbs: {}", self.collected_orbs), 7u8);
            }
            STATE_BOSS => {
                self.player.draw();
                self.boss.draw();
                text(40.0, 60.0, "BOSS!", 8u8);
            }
            _ => {}
        }
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    init(W, H, "Cubeboy Rust", 60);
    
    let game = std::rc::Rc::new(std::cell::RefCell::new(Game::new()));
    
    let game_update = std::rc::Rc::clone(&game);
    let game_draw = std::rc::Rc::clone(&game);
    
    run(
        move || {
            game_update.borrow_mut().update();
        },
        move || {
            game_draw.borrow().draw();
        },
    );
}
