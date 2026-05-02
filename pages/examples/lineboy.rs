use pyxel_rust::prelude::*;

const W: u32 = 512;
const H: u32 = 512;
const PW: f32 = 12.0;
const PH: f32 = 14.0;

// Game states
const ST_TITLE: u32 = 0;
const ST_PLAY: u32 = 1;
const ST_GAMEOVER: u32 = 2;
const ST_CLEAR: u32 = 3;

// Physics
const GRAVITY: f32 = 0.55;
const JUMP_VEL: f32 = -9.5;
const MOVE_SPEED: f32 = 3.2;

struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    grounded: bool,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player { x, y, vx: 0.0, vy: 0.0, grounded: false }
    }

    fn update(&mut self, ground_y: f32) {
        self.vy = (self.vy + GRAVITY).min(12.0);

        if btn(KEY_LEFT) {
            self.vx = -MOVE_SPEED;
        } else if btn(KEY_RIGHT) {
            self.vx = MOVE_SPEED;
        } else {
            self.vx = 0.0;
        }

        if btnp(KEY_SPACE) && self.grounded {
            self.vy = JUMP_VEL;
            self.grounded = false;
        }

        self.x += self.vx;
        self.y += self.vy;

        // Ground collision
        if self.y + PH >= ground_y && self.vy >= 0.0 {
            self.grounded = true;
            self.y = ground_y - PH;
            self.vy = 0.0;
        } else {
            self.grounded = false;
        }

        // Boundary
        self.x = self.x.max(0.0).min(W as f32 - PW);
    }

    fn draw(&self, col: u8) {
        rect(self.x, self.y, PW, PH, col);
    }
}

#[derive(Clone, Copy)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: i32,
}

impl Particle {
    fn new(x: f32, y: f32, vx: f32, vy: f32) -> Self {
        Particle { x, y, vx, vy, life: 20 }
    }

    fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.vy += 0.15;
        self.life -= 1;
    }

    fn draw(&self) {
        pset(self.x, self.y, 10);
    }
}

struct Game {
    state: u32,
    player: Player,
    particles: Vec<Particle>,
    ground_y: f32,
    collected: i32,
    total_items: i32,
    items: Vec<(f32, f32)>,
    enemies: Vec<(f32, f32)>,
}

impl Game {
    fn new() -> Self {
        let ground_y = H as f32 - 80.0;
        let mut items = Vec::new();
        let mut enemies = Vec::new();
        
        // Generate items
        for i in 0..10 {
            items.push((50.0 + i as f32 * 40.0, 250.0));
        }
        
        // Generate enemies
        for i in 0..3 {
            enemies.push((100.0 + i as f32 * 120.0, 300.0));
        }
        
        Game {
            state: ST_TITLE,
            player: Player::new(100.0, 100.0),
            particles: Vec::new(),
            ground_y,
            collected: 0,
            total_items: items.len() as i32,
            items,
            enemies,
        }
    }

    fn update(&mut self) {
        match self.state {
            ST_TITLE => {
                if btnp(KEY_SPACE) {
                    self.state = ST_PLAY;
                    self.player = Player::new(100.0, 100.0);
                    self.collected = 0;
                    self.particles.clear();
                }
            }
            ST_PLAY => {
                self.player.update(self.ground_y);

                // Check item collision
                self.items.retain(|(ix, iy)| {
                    let dx = self.player.x + PW / 2.0 - ix;
                    let dy = self.player.y + PH / 2.0 - iy;
                    if dx * dx + dy * dy < 256.0 {
                        self.collected += 1;
                        for _ in 0..5 {
                            let vx = (rnd(-2.0, Some(2.0)) as f32);
                            let vy = (rnd(-3.0, Some(1.0)) as f32);
                            self.particles.push(Particle::new(*ix, *iy, vx, vy));
                        }
                        false
                    } else {
                        true
                    }
                });

                // Check win
                if self.collected >= self.total_items {
                    self.state = ST_CLEAR;
                }

                // Check enemy collision
                for (ex, ey) in &self.enemies {
                    let dx = self.player.x + PW / 2.0 - ex;
                    let dy = self.player.y + PH / 2.0 - ey;
                    if dx * dx + dy * dy < 625.0 {
                        self.state = ST_GAMEOVER;
                        break;
                    }
                }

                // Fall check
                if self.player.y > H as f32 {
                    self.state = ST_GAMEOVER;
                }

                // Update particles
                for p in self.particles.iter_mut() {
                    p.update();
                }
                self.particles.retain(|p| p.life > 0);
            }
            ST_GAMEOVER => {
                if btnp(KEY_SPACE) {
                    *self = Game::new();
                }
            }
            ST_CLEAR => {
                if btnp(KEY_SPACE) {
                    *self = Game::new();
                }
            }
            _ => {}
        }
    }

    fn draw(&self) {
        cls(0);

        match self.state {
            ST_TITLE => {
                text(150.0, 100.0, "LINEBOY", 7);
                text(120.0, 150.0, "PRESS SPACE", 11);
            }
            ST_PLAY => {
                // Draw ground
                rect(0.0, self.ground_y, W as f32, 80.0, 3);

                // Draw items
                for (ix, iy) in &self.items {
                    circ(*ix, *iy, 3.0, 10);
                }

                // Draw enemies
                for (ex, ey) in &self.enemies {
                    circ(*ex, *ey, 6.0, 8);
                }

                // Draw particles
                for p in &self.particles {
                    p.draw();
                }

                // Draw player
                self.player.draw(11);

                // Draw HUD
                text(10.0, 10.0, &format!("Items: {}/{}", self.collected, self.total_items), 7);
            }
            ST_GAMEOVER => {
                text(150.0, 100.0, "GAME OVER", 8);
                text(120.0, 150.0, "PRESS SPACE", 11);
            }
            ST_CLEAR => {
                text(150.0, 100.0, "CLEAR!", 11);
                text(120.0, 150.0, "PRESS SPACE", 10);
            }
            _ => {}
        }
    }
}

fn main() {
    init(W, H, "Lineboy", 60);
    
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
