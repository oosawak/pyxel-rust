#![cfg(feature = "pyxel-core-backend")]
use pyxel_rust::api::*;
#[test]
fn native_helpers_edit_the_public_resource_banks() {
    let old = get_palette_color(8);
    set_palette_color(8, 0x123456);
    assert_eq!(get_palette_color(8), 0x123456);
    set_palette_color(8, old);
    image_rect(1, 0., 0., 8., 8., 8);
    assert_eq!(images()[1].borrow().pixel(2., 2.), 8);
    image_pset(1, 2., 2., 11);
    assert_eq!(images()[1].borrow().pixel(2., 2.), 11);
    image_line(1, 0., 12., 8., 12., 10);
    assert_eq!(images()[1].borrow().pixel(3., 12.), 10);
    image_tri(1, 16., 16., 24., 16., 20., 24., 7);
    assert_eq!(images()[1].borrow().pixel(20., 19.), 7);
    tilemap_cls(0, (2, 3));
    assert_eq!(tilemap_pget(0, 1, 1), (2, 3));
    tilemap_pset(0, 1, 1, (4, 5));
    assert_eq!(tilemaps()[0].borrow().tile(1., 1.), (4, 5));
    let font = Font::new(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/games/assets/check-font.bdf"
        ),
        None,
    )
    .unwrap();
    assert_eq!(font.borrow_mut().text_width("AA"), 8);
}
