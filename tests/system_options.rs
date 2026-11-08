#![cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
use pyxel_rust::api::*;
#[test]
fn headless_initialization_exposes_official_shapes_and_validates_before_mutation() {
    let options = InitOptions {
        headless: true,
        display_scale: Some(2),
        capture_scale: 3,
        capture_sec: 1,
        quit_key: KEY_Q,
    };
    assert!(init_with_options(32, 24, "bad", 0, options.clone()).is_err());
    init_with_options(32, 24, "Headless compatibility test", 30, options.clone()).unwrap();
    cls(1);
    rect(0., 0., 8., 8., 8);
    rectb(8., 0., 8., 8., 8);
    circ(4., 16., 3., 11);
    circb(12., 16., 3., 11);
    assert_eq!(pget(4., 4.), 8, "rect fills its center");
    assert_eq!(pget(12., 4.), 1, "rectb keeps its center empty");
    assert_eq!(pget(8., 4.), 8);
    assert_eq!(pget(4., 16.), 11, "circ fills its center");
    assert_eq!(pget(12., 16.), 1, "circb keeps its center empty");
    assert_eq!(pget(15., 16.), 11);
    assert!(init_with_options(64, 48, "bad", 0, options).is_err());
    assert_eq!((width(), height()), (32, 24));
    assert_eq!(
        pget(4., 4.),
        8,
        "failed initialization keeps existing pixels"
    );
    #[cfg(feature = "wasm-backend")]
    {
        pyxel_rust::backend::wasm_backend::recording::capture();
        let gif = configured_screencast_gif(None).unwrap().unwrap();
        assert_eq!(u16::from_le_bytes([gif[6], gif[7]]), 96);
        assert_eq!(u16::from_le_bytes([gif[8], gif[9]]), 72);
    }
}
