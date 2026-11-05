#![cfg(feature = "wgpu-backend")]

use pyxel_rust::backend::wgpu_backend::WgpuState;

#[test]
fn image_banks_blit_with_transparency_and_horizontal_flip() {
    let mut state = WgpuState::new(16, 16, "test", 30);
    state.clear(1);
    state.image_pset(0, 0.0, 0.0, 8);
    state.image_pset(0, 1.0, 0.0, 10);
    state.draw_image(2.0, 2.0, 0, 0.0, 0.0, -3.0, 1.0, Some(0));
    assert_eq!(state.pget(2.0, 2.0), 1);
    assert_eq!(state.pget(3.0, 2.0), 10);
    assert_eq!(state.pget(4.0, 2.0), 8);
}

#[test]
fn tilemap_uses_image_bank_and_honors_camera_and_clipping() {
    let mut state = WgpuState::new(16, 16, "test", 30);
    state.image_rect(0, 8.0, 0.0, 8.0, 8.0, 7);
    state.tilemap_pset(0, 0, 0, (1, 0));
    assert_eq!(state.tilemap_pget(0, 0, 0), (1, 0));
    state.camera_x = 4.0;
    state.clip_rect = Some([0, 0, 4, 8]);
    state.draw_tilemap(4.0, 0.0, 0, 0.0, 0.0, 8.0, 8.0, None);
    assert_eq!(state.pixel_buffer[0], 7);
    assert_eq!(state.pixel_buffer[3], 7);
    assert_eq!(state.pixel_buffer[4], 0);
}
