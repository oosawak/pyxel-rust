#![cfg(feature = "wasm-backend")]
use pyxel_cube_core::AudioRenderer;
use pyxel_rust::api::*;
const BDF: &[u8] = include_bytes!("../src/games/assets/check-font.bdf");
#[test]
fn resource_handles_memory_files_and_official_audio_share_live_state() {
    let image = images()[1].clone();
    image.borrow_mut().clear(0);
    image.borrow_mut().draw_rect(0., 0., 8., 8., 8);
    assert_eq!(images()[1].borrow().pixel(2., 2.), 8);
    let map = tilemaps()[0].clone();
    map.borrow_mut().imgsrc = ImageSource::Index(1);
    map.borrow_mut().set_tile(0., 0., (0, 0));
    let target = Image::new(16, 16);
    target
        .borrow_mut()
        .draw_tilemap(0., 0., &map, 0., 0., 8., 8., None, None, None);
    assert_eq!(
        target.borrow().pixel(2., 2.),
        8,
        "tile (0,0) must render from imgsrc=1"
    );
    image.borrow_mut().draw_rect(0., 0., 8., 8., 11);
    target
        .borrow_mut()
        .draw_tilemap(0., 0., &map, 0., 0., 8., 8., None, None, None);
    assert_eq!(target.borrow().pixel(2., 2.), 11);
    target.borrow_mut().set_clip_rect(0., 0., 1., 1.);
    let png = target.borrow().png_bytes().unwrap();
    let loaded = Image::from_image_bytes(&png, Some(false)).unwrap();
    assert_eq!(
        loaded.borrow().pixel(2., 2.),
        11,
        "PNG export must ignore drawing clip"
    );
    let before = images()[1].borrow().pixel(0., 0.);
    assert!(images()[1]
        .borrow_mut()
        .load_bytes(0., 0., b"invalid")
        .is_err());
    assert_eq!(images()[1].borrow().pixel(0., 0.), before);
    let font = Font::from_bytes(BDF, "bdf", None).unwrap();
    assert_eq!(font.borrow_mut().text_width("AA"), 8);
    target.borrow_mut().reset_clip_rect();
    target.borrow_mut().clear(0);
    target.borrow_mut().draw_text(0., 0., "A", 7, Some(&font));
    assert!((0..6).any(|y| (0..4).any(|x| target.borrow().pixel(x as f32, y as f32) == 7)));
    assert!(Font::from_bytes(b"invalid", "ttf", None).is_err());
    sounds()[0]
        .lock()
        .unwrap()
        .set_mml("t240 o4 l16 c e g > c")
        .unwrap();
    let wav = sounds()[0].lock().unwrap().wav_bytes(0.25).unwrap();
    assert_eq!(&wav[..4], b"RIFF");
    assert!(wav[44..].iter().any(|v| *v != 0));
    sounds()[2]
        .lock()
        .unwrap()
        .load_pcm_bytes(&wav, "wav")
        .unwrap();
    assert!(sounds()[2].lock().unwrap().total_seconds().unwrap() > 0.2);
    assert!(sounds()[2]
        .lock()
        .unwrap()
        .load_pcm_bytes(b"bad", "wav")
        .is_err());
    assert!(sounds()[2].lock().unwrap().total_seconds().unwrap() > 0.2);
    try_play(0, 0, None, Some(false), None).unwrap();
    assert!(is_playing(0));
    let mut renderer = AudioRenderer::new();
    let mut samples = vec![0; 1024];
    let mut nonzero = false;
    for _ in 0..20 {
        renderer.render(&mut samples);
        nonzero |= samples.iter().any(|s| *s != 0);
    }
    assert!(nonzero);
    assert!(!is_playing(0));
    try_play(0, 2, None, Some(false), None).unwrap();
    let mut pcm_nonzero = false;
    for _ in 0..15 {
        renderer.render(&mut samples);
        pcm_nonzero |= samples.iter().any(|s| *s != 0);
    }
    assert!(pcm_nonzero);
    assert!(!is_playing(0));
    try_play(0, 0, Some(0.1), Some(true), None).unwrap();
    assert!(play_pos(0).unwrap().1 >= 0.09);
    for _ in 0..30 {
        renderer.render(&mut samples);
    }
    assert!(is_playing(0));
    musics()[0].lock().unwrap().set(vec![vec![0], vec![2]]);
    channels()[0].lock().unwrap().gain = 0.25;
    try_playm(0, None, Some(true)).unwrap();
    assert!(is_playing(0) && is_playing(1));
    assert_eq!(channels()[0].lock().unwrap().gain, 0.25);
    musics()[1].lock().unwrap().set(vec![vec![0], vec![9999]]);
    assert!(try_playm(1, None, None).is_err());
    assert!(is_playing(0) && is_playing(1));
    stop(Some(0));
    assert!(!is_playing(0) && is_playing(1));
    stop(None);
    assert!(!is_playing(1));
    assert!(try_play(999, 0, None, None, None).is_err());
    assert!(try_play(0, 999, None, None, None).is_err());
    assert!(try_play(0, 0, Some(f32::NAN), None, None).is_err());
}
