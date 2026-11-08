#![cfg(feature = "wasm-backend")]
use pyxel_rust::{api::*, cube::Mesh};
#[test]
fn named_files_feed_real_resource_decoders_and_preserve_state_on_failure() {
    init_with_options(
        32,
        24,
        "Named files",
        30,
        InitOptions {
            headless: true,
            capture_sec: 0,
            ..InitOptions::default()
        },
    )
    .unwrap();
    cls(11);
    screenshot(Some("vfs:/captures/frame"), Some(2)).unwrap();
    let capture = Image::from_image("vfs:/captures/frame.png", None).unwrap();
    assert_eq!(
        (capture.borrow().width(), capture.borrow().height()),
        (64, 48)
    );
    assert_eq!(capture.borrow().pixel(63., 47.), 11);
    let file = "vfs:/assets/日本語.png";
    let image = Image::new(2, 2);
    image.borrow_mut().clear(11);
    image.borrow().save(file, 3).unwrap();
    let loaded = Image::from_image(file, Some(false)).unwrap();
    assert_eq!((loaded.borrow().width(), loaded.borrow().height()), (6, 6));
    assert_eq!(loaded.borrow().pixel(5., 5.), 11);
    let target = Image::new(8, 8);
    target.borrow_mut().load(1, 1, file, None).unwrap();
    assert_eq!(target.borrow().pixel(6., 6.), 11);
    let before = browser_files::get(file).unwrap();
    assert!(image.borrow().save(file, 0).is_err());
    assert_eq!(browser_files::get(file).unwrap(), before);
    browser_files::put("vfs:/bad.png", b"bad").unwrap();
    assert!(target
        .borrow_mut()
        .load(1, 1, "vfs:/bad.png", None)
        .is_err());
    assert_eq!(target.borrow().pixel(6., 6.), 11);
    browser_files::put(
        "vfs:/fonts/check.bdf",
        include_bytes!("../src/games/assets/check-font.bdf"),
    )
    .unwrap();
    let font = Font::new("vfs:/fonts/check.bdf", None).unwrap();
    assert_eq!(font.borrow_mut().text_width("AA"), 8);
    let tmx=b"<map tilewidth=\"8\" tileheight=\"8\"><tileset firstgid=\"1\" columns=\"32\"/><layer width=\"2\" height=\"1\"><data encoding=\"csv\">2,33</data></layer></map>";
    browser_files::put("vfs:/maps/a.tmx", tmx).unwrap();
    let map = Tilemap::from_tmx("vfs:/maps/a.tmx", 0).unwrap();
    assert_eq!(map.borrow().tile(0., 0.), (1, 0));
    let target = Tilemap::new(8, 8, ImageSource::Index(0));
    target
        .borrow_mut()
        .load(2, 3, "vfs:/maps/a.tmx", 0)
        .unwrap();
    assert_eq!(target.borrow().tile(3., 3.), (0, 1));
    sounds()[0]
        .lock()
        .unwrap()
        .set("c2", "p", "7", "n", 5)
        .unwrap();
    sounds()[0]
        .lock()
        .unwrap()
        .save("vfs:/sfx/test", 0.05, Some(false))
        .unwrap();
    let wav = browser_files::get("vfs:/sfx/test.wav").unwrap();
    assert_eq!(&wav[..4], b"RIFF");
    sounds()[1]
        .lock()
        .unwrap()
        .load_pcm("vfs:/sfx/test.wav")
        .unwrap();
    assert!(sounds()[1].lock().unwrap().total_seconds().unwrap() > 0.04);
    musics()[0].lock().unwrap().set(vec![vec![0]]);
    musics()[0]
        .lock()
        .unwrap()
        .save("vfs:/sfx/music", 0.05, Some(false))
        .unwrap();
    assert_eq!(
        &browser_files::get("vfs:/sfx/music.wav").unwrap()[..4],
        b"RIFF"
    );
    browser_files::put("vfs:/sfx/keep.wav", b"original").unwrap();
    assert!(sounds()[0]
        .lock()
        .unwrap()
        .save("vfs:/sfx/keep", 0.05, Some(true))
        .is_err());
    assert_eq!(
        browser_files::get("vfs:/sfx/keep.wav").unwrap(),
        b"original"
    );
    browser_files::put(
        "vfs:/models/a.glb",
        include_bytes!("../docs/examples/retro3d/assets/animated-pyramid.glb"),
    )
    .unwrap();
    assert!(!Mesh::from_glb("vfs:/models/a.glb", None, 30.)
        .unwrap()
        .borrow()
        .primitives
        .is_empty());
    images()[1].borrow_mut().set_pixel(2., 2., 11);
    save("vfs:/projects/game", false, false, false, false).unwrap();
    images()[1].borrow_mut().set_pixel(2., 2., 8);
    load("vfs:/projects/game.pyxres", false, false, false, false).unwrap();
    assert_eq!(images()[1].borrow().pixel(2., 2.), 11);
    let palette = save_pal_text();
    save_pal("vfs:/palettes/p").unwrap();
    colors()[0] = 0x123456;
    load_pal("vfs:/palettes/p.pyxpal").unwrap();
    assert_eq!(save_pal_text(), palette);
    assert_eq!(
        browser_files::normalize("vfs:/a/./b/../c").unwrap(),
        "vfs:/a/c"
    );
    for bad in [
        "../escape",
        "vfs:/../../escape",
        "vfs:/",
        "vfs:/a\\b",
        "https://host/a",
    ] {
        assert!(browser_files::put(bad, b"x").is_err(), "{bad}");
    }
    assert!(browser_files::remove(file).unwrap());
    assert!(!browser_files::remove(file).unwrap());
    assert!(Image::from_image(file, None).is_err());
}
