//! Interactive exercise of the public input, drawing, and math APIs.
use crate::api::*;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
thread_local! {static INFO:RefCell<serde_json::Value>=RefCell::new(serde_json::json!({}));}
#[wasm_bindgen]
pub fn api_coverage_info() -> String {
    INFO.with(|v| v.borrow().to_string())
}
#[wasm_bindgen]
pub fn start_api_coverage() {
    init(320, 200, "Pyxel API coverage", 30);
    mouse(true);
    nseed(42);
    rseed(42);
    let mut presses = 0;
    let mut releases = 0;
    let mut repeats = 0;
    let mut clicks = 0;
    let mut wheel = 0;
    let mut repeat_frames = Vec::new();
    run(
        move || {
            if btnp(KEY_A) {
                presses += 1;
            }
            if btnr(KEY_A) {
                releases += 1;
            }
            if btnp_hold(KEY_A, 6, 3) {
                repeats += 1;
                repeat_frames.push(frame_count());
            }
            if btnp(MOUSE_BUTTON_LEFT) {
                clicks += 1;
            }
            wheel += mouse_wheel();
            let axes = [
                GAMEPAD1_AXIS_LEFTX,
                GAMEPAD2_AXIS_LEFTX,
                GAMEPAD3_AXIS_LEFTX,
                GAMEPAD4_AXIS_LEFTX,
            ]
            .map(btnv);
            let pads = [
                GAMEPAD1_BUTTON_A,
                GAMEPAD2_BUTTON_A,
                GAMEPAD3_BUTTON_A,
                GAMEPAD4_BUTTON_A,
            ]
            .map(btn);
            INFO.with(|v| *v.borrow_mut()=serde_json::json!({
            "frame":frame_count(),"a":btn(KEY_A),"shift":btn(KEY_SHIFT),"keys":input_keys(),
            "presses":presses,"releases":releases,"repeats":repeats,"repeat_frames":repeat_frames,
            "clicks":clicks,"wheel":wheel,"mouse":[mouse_x(),mouse_y()],
            "left":btn(MOUSE_BUTTON_LEFT),"right":btn(MOUSE_BUTTON_RIGHT),"axes":axes,"pads":pads,
            "ceil":ceil(-1.2),"angle":atan2(1.,0.),"noise":noise(0.5,Some(0.25),None)
        }));
        },
        || {
            cls(1);
            text(8., 8., "PUBLIC API / DRAWING + INPUT + MATH", 7);
            elli(12., 28., 64., 40., 8);
            ellib(90., 28., 64., 40., 10);
            tri(175., 28., 164., 68., 224., 60., 11);
            trib(250., 28., 240., 68., 304., 60., 7);
            rectb(12., 88., 64., 40., 7);
            fill(20., 100., 3);
            circb(122., 108., 20., 14);
            text(170., 92., "ELLIPSE / TRIANGLE", 7);
            text(170., 104., "OUTLINE / FILL", 7);
            for x in 0..300 {
                let y = 160. + noise(x as f32 / 30., None, None) * 20.;
                pset(10. + x as f32, y, 11);
            }
            let x = mouse_x() as f32;
            let y = mouse_y() as f32;
            line(x - 4., y, x + 4., y, 7);
            line(x, y - 4., x, y + 4., 7);
        },
    );
}
#[wasm_bindgen]
pub fn api_coverage_cursor(visible: bool) {
    mouse(visible);
}
