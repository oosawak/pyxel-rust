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

#[test]
fn high_resolution_rasterizes_geometry_and_preserves_simulation() {
    for id in 0..6 {
        let mut standard = CubeExample::new(id).unwrap();
        let mut high = CubeExample::new(id).unwrap();
        high.set_resolution(2).unwrap();
        for _ in 0..3 {
            standard.update(&mut Controls::default());
            high.update(&mut Controls::default());
        }
        let mut expected = standard.info();
        let actual = high.info();
        for key in ["resolution", "render_width", "render_height"] {
            expected[key] = actual[key].clone();
        }
        assert_eq!(expected, actual, "simulation differs for {id}");
        let image = high.draw();
        let image = image.borrow();
        let (w, h) = standard.size();
        assert_eq!((image.width(), image.height()), (w * 2, h * 2));
        // A nearest-neighbor upscale would have identical colors in every 2×2 block.
        let detail = (0..h).any(|y| {
            (0..w).any(|x| {
                let color = image.pixel((x * 2) as f32, (y * 2) as f32);
                [(1, 0), (0, 1), (1, 1)]
                    .iter()
                    .any(|(dx, dy)| image.pixel((x * 2 + dx) as f32, (y * 2 + dy) as f32) != color)
            })
        });
        assert!(detail, "example {id} must rasterize additional detail");
        assert!(image.pixel(0., 0.) < 255);
        drop(image);
        assert!(high.set_resolution(3).is_err());
        assert_eq!(high.output_size(), (w * 2, h * 2));
        high.set_resolution(4).unwrap();
        let large = high.draw();
        assert_eq!(
            (large.borrow().width(), large.borrow().height()),
            (w * 4, h * 4)
        );
        high.set_resolution(1).unwrap();
        assert_eq!(high.output_size(), (w, h));
        high.draw();
    }
}
