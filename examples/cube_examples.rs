//! Run the six official Cube ports on SDL2: cargo run --example cube_examples -- 1
use pyxel_rust::{
    games::cube_examples::{Controls, CubeExample},
    prelude::*,
};
use std::{cell::RefCell, rc::Rc};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id = std::env::args()
        .nth(1)
        .unwrap_or("1".into())
        .parse::<usize>()?
        .checked_sub(1)
        .ok_or("Choose example 1..6")?;
    let game = Rc::new(RefCell::new(CubeExample::new(id)?));
    let resolution = std::env::args()
        .nth(2)
        .unwrap_or("1".into())
        .parse::<u32>()?;
    game.borrow_mut().set_resolution(resolution)?;
    let (w, h) = game.borrow().output_size();
    init(w, h, "Official Cube example — Rust", 30);
    let sounds: serde_json::Value = serde_json::from_str(include_str!(
        "../src/games/cube_examples/assets/sounds.json"
    ))?;
    for entry in sounds
        .as_array()
        .unwrap()
        .iter()
        .filter(|s| s["example"].as_u64() == Some(id as u64))
    {
        pyxel::sounds()[entry["sound"].as_u64().unwrap() as usize]
            .lock()
            .unwrap()
            .set_mml(entry["mml"].as_str().unwrap())?;
    }
    let update_game = game.clone();
    run(
        move || {
            if btnp(KEY_Q) {
                quit();
            }
            let mut input = Controls {
                left: btn(KEY_LEFT),
                right: btn(KEY_RIGHT),
                up: btn(KEY_UP),
                down: btn(KEY_DOWN),
                action: btn(KEY_SPACE) || btn(MOUSE_BUTTON_LEFT),
                pressed: btnp(KEY_SPACE) || btnp(MOUSE_BUTTON_LEFT),
                released: btnr(KEY_SPACE) || btnr(MOUSE_BUTTON_LEFT),
                shade: btnp(KEY_S),
                reset: btnp(KEY_R),
                motion: [KEY_1, KEY_2, KEY_3].iter().position(|key| btnp(*key)),
                mouse_x: mouse_x() as f32 / resolution as f32,
                mouse_y: mouse_y() as f32 / resolution as f32,
                pointer: true,
                ..Controls::default()
            };
            update_game.borrow_mut().update(&mut input);
            for effect in input.sound_events {
                play(effect as u32, effect as u32, None, Some(false), Some(false));
            }
        },
        move || {
            let image = game.borrow_mut().draw();
            blit_3d(&image, 0., 0.);
        },
    );
    Ok(())
}
