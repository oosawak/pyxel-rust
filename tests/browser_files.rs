#![cfg(feature = "wasm-backend")]
use pyxel_rust::api::*;
const TMX: &str = r#"<map tilewidth="8" tileheight="8"><tileset firstgid="1" columns="32"/><layer width="2" height="1"><data encoding="csv">2,33</data></layer></map>"#;
#[test]
fn official_archives_palette_and_tmx_roundtrip_without_partial_failure() {
    images()[1].borrow_mut().set_pixel(3., 4., 200);
    tilemaps()[0].borrow_mut().set_tile(1., 2., (3, 4));
    sounds()[0]
        .lock()
        .unwrap()
        .set("c2d2", "p", "7", "n", 5)
        .unwrap();
    musics()[0].lock().unwrap().set(vec![vec![0]]);
    let bytes = save_bytes(ResourceOptions::default()).unwrap();
    assert_eq!(&bytes[..2], b"PK");
    images()[1].borrow_mut().clear(0);
    sounds()[0].lock().unwrap().notes.clear();
    load_bytes(&bytes, ResourceOptions::default()).unwrap();
    assert_eq!(images()[1].borrow().pixel(3., 4.), 200);
    assert_eq!(tilemaps()[0].borrow().tile(1., 2.), (3, 4));
    assert_eq!(sounds()[0].lock().unwrap().notes.len(), 2);
    assert!(load_bytes(b"invalid", ResourceOptions::default()).is_err());
    assert_eq!(images()[1].borrow().pixel(3., 4.), 200);
    let excluding = ResourceOptions {
        exclude_images: true,
        exclude_tilemaps: true,
        exclude_sounds: true,
        exclude_musics: true,
    };
    let empty = save_bytes(excluding).unwrap();
    load_bytes(&empty, ResourceOptions::default()).unwrap();
    assert_eq!(images()[1].borrow().pixel(3., 4.), 200);
    let before = save_pal_text();
    assert!(load_pal_text("ff0000\nzzzzzz").is_err());
    assert_eq!(save_pal_text(), before);
    load_pal_text("123456\nabcdef\n").unwrap();
    assert_eq!(&*colors(), &[0x123456, 0xabcdef]);
    load_pal_text(&before).unwrap();
    with_images(|banks| banks.push(Image::new(2, 2)));
    assert_eq!(images().len(), 4);
    with_images(|banks| {
        banks.pop();
    });
    let map = Tilemap::from_tmx_text(TMX, 0).unwrap();
    assert_eq!(map.borrow().tile(0., 0.), (1, 0));
    assert_eq!(map.borrow().tile(1., 0.), (0, 1));
    let target = Tilemap::new(4, 4, ImageSource::Index(1));
    target.borrow_mut().load_tmx_text(1., 1., TMX, 0).unwrap();
    assert_eq!(target.borrow().tile(2., 1.), (0, 1));
    assert!(target
        .borrow_mut()
        .load_tmx_text(1., 1., "invalid", 0)
        .is_err());
    assert_eq!(target.borrow().tile(2., 1.), (0, 1));
    let mml = gen_bgm(0, 0, 3, 123, false).unwrap();
    assert_eq!(mml, pyxel_cube_core::generate_bgm_mml(0, 0, 3, 123));
    assert!(mml.iter().all(|part| !part.is_empty()));
    assert!(gen_bgm(99, 0, 3, 0, false).is_err());
    gen_bgm(0, 0, 3, 123, true).unwrap();
    assert!(is_playing(0));
    stop(None);
}
