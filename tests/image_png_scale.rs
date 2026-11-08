#![cfg(feature = "wasm-backend")]
use pyxel_rust::api::*;
#[test]
fn scaled_png_preserves_every_palette_pixel_and_ignores_drawing_state() {
    let image = Image::new(3, 2);
    {
        let mut i = image.borrow_mut();
        for (n, c) in [8, 11, 7, 10, 2, 14].into_iter().enumerate() {
            i.set_pixel((n % 3) as f32, (n / 3) as f32, c);
        }
        i.set_clip_rect(0., 0., 1., 1.);
        i.set_camera(50., 50.);
        i.map_color(8, 0);
        i.set_dithering(0.);
    }
    for scale in [1, 2, 3, 16] {
        let png = image.borrow().png_bytes_scaled(scale).unwrap();
        let decoded = Image::from_image_bytes(&png, Some(false)).unwrap();
        assert_eq!(
            (decoded.borrow().width(), decoded.borrow().height()),
            (3 * scale, 2 * scale)
        );
        for y in 0..2 * scale {
            for x in 0..3 * scale {
                let expected = [8, 11, 7, 10, 2, 14][((y / scale) * 3 + x / scale) as usize];
                assert_eq!(decoded.borrow().pixel(x as f32, y as f32), expected);
            }
        }
    }
    assert!(image.borrow().png_bytes_scaled(0).is_err());
    assert!(image.borrow().png_bytes_scaled(u32::MAX).is_err());
    assert!(image.borrow().png_bytes_scaled(100_000).is_err());
    assert_eq!(
        image.borrow().png_bytes().unwrap(),
        image.borrow().png_bytes_scaled(1).unwrap()
    );
}
