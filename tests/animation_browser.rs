#![cfg(feature = "wasm-backend")]

use pyxel_rust::api::MeshAnimationPlayer;
use pyxel_rust::cube::{Mesh, RcMesh};

const ANIMATED: &[u8] = include_bytes!("../docs/examples/retro3d/assets/animated-pyramid.glb");

fn player() -> (RcMesh, MeshAnimationPlayer) {
    let mesh = Mesh::from_glb_bytes(ANIMATED, None, 30.0).unwrap();
    let player = MeshAnimationPlayer::new(mesh.clone(), 30.0).unwrap();
    (mesh, player)
}

fn node_y(mesh: &RcMesh) -> f32 {
    mesh.borrow().transforms[1].borrow().pos_value().y
}

fn close(actual: f32, expected: f32) {
    // GLB stores f32 keys and linear interpolation rounds intermediate arithmetic.
    assert!((actual - expected).abs() < 1e-5, "{actual} != {expected}");
}

#[test]
fn imported_clips_keep_names_and_seconds() {
    let (_, player) = player();
    assert_eq!(
        player.clips(),
        vec![("Bounce".into(), 2.0), ("Turn".into(), 2.0)]
    );
    assert_eq!(player.selected_clip(), None);
    assert!(!player.is_playing());
}

#[test]
fn interpolates_playback_and_freezes_when_paused() {
    let (mesh, mut player) = player();
    player.select_clip(Some(0)).unwrap();
    player.advance(0.5).unwrap();
    close(player.elapsed(), 0.5);
    close(node_y(&mesh), 3.4);
    player.set_playing(false);
    player.advance(1.0).unwrap();
    close(player.elapsed(), 0.5);
    close(node_y(&mesh), 3.4);
}

#[test]
fn clip_switch_and_rest_pose_remove_previous_channels() {
    let (mesh, mut player) = player();
    let rest: Vec<_> = mesh
        .borrow()
        .transforms
        .iter()
        .map(|m| m.borrow().data)
        .collect();
    player.select_clip(Some(0)).unwrap();
    player.seek(1.0).unwrap();
    close(node_y(&mesh), 3.8);
    player.select_clip(Some(1)).unwrap();
    player.seek(1.0).unwrap();
    close(node_y(&mesh), 3.0);
    close(mesh.borrow().transforms[0].borrow().get(0, 0), 0.0);
    player.select_clip(None).unwrap();
    assert!(!player.is_playing());
    for (part, expected) in mesh.borrow().transforms.iter().zip(rest) {
        assert_eq!(part.borrow().data, expected);
    }
}

#[test]
fn loops_or_stops_at_the_endpoint_and_can_restart() {
    let (_, mut player) = player();
    player.select_clip(Some(0)).unwrap();
    player.advance(2.5).unwrap();
    close(player.elapsed(), 0.5);
    player.set_looping(false);
    player.advance(3.0).unwrap();
    close(player.elapsed(), 2.0);
    assert!(!player.is_playing());
    player.set_playing(true);
    assert!(player.is_playing());
    close(player.elapsed(), 0.0);
}

#[test]
fn speed_seek_and_invalid_inputs_preserve_a_valid_pose() {
    let (mesh, mut player) = player();
    player.select_clip(Some(0)).unwrap();
    player.set_speed(2.0).unwrap();
    player.advance(0.25).unwrap();
    close(player.elapsed(), 0.5);
    player.seek(1.0).unwrap();
    close(node_y(&mesh), 3.8);
    assert!(player.select_clip(Some(99)).is_err());
    assert_eq!(player.selected_clip(), Some(0));
    for value in [f32::NAN, f32::INFINITY] {
        assert!(player.seek(value).is_err());
    }
    for value in [0.0, -1.0, f32::NAN] {
        assert!(player.set_speed(value).is_err());
    }
    for value in [-1.0, f32::NAN] {
        assert!(player.advance(value).is_err());
    }
    close(player.elapsed(), 1.0);
    close(node_y(&mesh), 3.8);
}

#[test]
fn seeking_to_the_loop_endpoint_displays_the_endpoint() {
    let (mesh, mut player) = player();
    player.select_clip(Some(1)).unwrap();
    player.seek(2.0).unwrap();
    close(player.elapsed(), 2.0);
    // The turn clip ends at 180 degrees; wrapping the sample would show identity.
    close(mesh.borrow().transforms[0].borrow().get(0, 0), -1.0);
    player.advance(0.5).unwrap();
    close(player.elapsed(), 0.5);
}
