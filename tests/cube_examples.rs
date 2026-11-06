#![cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
use pyxel_rust::games::cube_examples::{Controls, CubeExample};

#[test]
fn all_six_examples_boot_animate_and_render_their_native_resolutions() {
    for id in 0..6 {
        let mut game = CubeExample::new(id).unwrap();
        let mut input = Controls {
            mouse_x: 160.,
            mouse_y: 120.,
            ..Controls::default()
        };
        for _ in 0..3 {
            game.update(&mut input);
        }
        let image = game.draw();
        let (w, h) = game.size();
        assert_eq!((image.borrow().width(), image.borrow().height()), (w, h));
        let colors: std::collections::HashSet<_> = (0..h)
            .flat_map(|y| (0..w).map(move |x| (x, y)))
            .map(|(x, y)| image.borrow().pixel(x as f32, y as f32))
            .collect();
        assert!(
            colors.len() > 3,
            "example {id} must render actual geometry: {colors:?}"
        );
        assert_eq!(game.info()["frame"], 3);
    }
}
#[test]
fn shapes_switch_wireframe_and_gazelle_switches_clips() {
    let mut shapes = CubeExample::new(1).unwrap();
    shapes.update(&mut Controls {
        action: true,
        ..Controls::default()
    });
    assert_eq!(shapes.info()["wire"], true);
    let mut motion = CubeExample::new(3).unwrap();
    motion.update(&mut Controls {
        motion: Some(2),
        ..Controls::default()
    });
    assert_eq!(motion.info()["motion"], 2);
    let before = motion.info()["time"].as_f64().unwrap();
    motion.update(&mut Controls::default());
    assert!(motion.info()["time"].as_f64().unwrap() > before);
}
#[test]
fn collision_player_jumps_moves_and_platform_moves() {
    let mut game = CubeExample::new(4).unwrap();
    let initial = game.info();
    for _ in 0..10 {
        game.update(&mut Controls::default());
    }
    let floor = game.info();
    assert!(floor["on_floor"].as_bool().unwrap());
    game.update(&mut Controls {
        pressed: true,
        action: true,
        ..Controls::default()
    });
    let jump = game.info();
    assert_eq!(jump["jumps"], 1);
    assert!(jump["player"][1].as_f64().unwrap() > floor["player"][1].as_f64().unwrap());
    game.update(&mut Controls {
        pressed: true,
        action: true,
        right: true,
        ..Controls::default()
    });
    assert_eq!(game.info()["jumps"], 2);
    assert_ne!(game.info()["platform"], initial["platform"]);
}
#[test]
fn physics_integrates_shoots_and_resets() {
    let mut game = CubeExample::new(5).unwrap();
    let initial = game.info();
    for _ in 0..20 {
        game.update(&mut Controls::default());
    }
    assert_ne!(game.info()["dynamic_y"], initial["dynamic_y"]);
    game.update(&mut Controls {
        action: true,
        pressed: true,
        ..Controls::default()
    });
    assert_eq!(game.info()["shots"], 1);
    assert!(game.info()["bodies"].as_u64().unwrap() > initial["bodies"].as_u64().unwrap());
    game.update(&mut Controls {
        reset: true,
        ..Controls::default()
    });
    assert_eq!(game.info()["shots"], 0);
    assert_eq!(game.info()["bodies"], initial["bodies"]);
}
#[test]
fn custom_shapes_lock_and_fire_at_projected_targets() {
    let mut game = CubeExample::new(2).unwrap();
    game.update(&mut Controls {
        mouse_y: 96.,
        ..Controls::default()
    });
    let info = game.info();
    let x = info["centers"][0][0].as_f64().unwrap() as f32;
    let y = info["centers"][0][1].as_f64().unwrap() as f32;
    game.update(&mut Controls {
        mouse_x: x,
        mouse_y: y,
        action: true,
        pressed: true,
        ..Controls::default()
    });
    assert!(game.info()["locked"]
        .as_array()
        .unwrap()
        .contains(&serde_json::json!(0)));
    game.update(&mut Controls {
        released: true,
        ..Controls::default()
    });
    assert!(game.info()["shots"].as_u64().unwrap() > 0);
    for _ in 0..15 {
        game.update(&mut Controls::default());
        game.draw();
    }
}
