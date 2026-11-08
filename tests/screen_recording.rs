#![cfg(feature = "wasm-backend")]
use pyxel_cube_core::Screencast;
#[test]
fn gif_ring_buffer_scales_resets_and_preserves_frames_on_invalid_export() {
    let mut recording = Screencast::new(2, 1);
    assert!(recording.gif_bytes(2).unwrap().is_none());
    let colors = [0x000000, 0xff0000, 0x00ff00];
    recording.capture(2, 1, &[1, 0], &colors, 0);
    recording.capture(2, 1, &[0, 1], &colors, 1);
    recording.capture(2, 1, &[2, 0], &colors, 2);
    assert_eq!(recording.frame_count(), 2);
    assert!(recording.gif_bytes(0).is_err());
    assert_eq!(recording.frame_count(), 2);
    let gif = recording.gif_bytes(2).unwrap().unwrap();
    assert_eq!(&gif[..6], b"GIF89a");
    assert_eq!(u16::from_le_bytes([gif[6], gif[7]]), 4);
    assert_eq!(u16::from_le_bytes([gif[8], gif[9]]), 2);
    assert_eq!(recording.frame_count(), 0);
    recording.capture(2, 1, &[0, 2], &colors, 3);
    recording.reset();
    assert!(recording.gif_bytes(1).unwrap().is_none());
}
