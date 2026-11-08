#![cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
use pyxel_rust::{
    api::NodeScene,
    cube::{Collider, Mat4, Mesh, Node, RcMesh, RcNode, Vec3},
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
fn mesh() -> RcMesh {
    #[cfg(feature = "wasm-backend")]
    {
        Mesh::from_glb_bytes(
            include_bytes!("../docs/examples/retro3d/assets/animated-pyramid.glb"),
            None,
            30.,
        )
        .unwrap()
    }
    #[cfg(feature = "pyxel-core-backend")]
    {
        Mesh::from_glb(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/docs/examples/retro3d/assets/animated-pyramid.glb"
            ),
            None,
            30.,
        )
        .unwrap()
    }
}
fn part(root: &RcNode) -> RcNode {
    Node::find_by_name(root, "Textured pyramid").pop().unwrap()
}
fn y(node: &RcNode) -> f32 {
    node.borrow().transform.borrow().pos_value().y
}
fn close(a: f32, b: f32) {
    assert!((a - b).abs() < 1e-5, "{a} != {b}");
}
fn trigger() -> pyxel_rust::cube::RcCollider {
    Collider::new(
        Vec3::zero(),
        0.05,
        None,
        true,
        false,
        0.,
        0.,
        0.,
        Vec3::zero(),
        Vec3::zero(),
        0.,
        Vec3::new(0., -1., 0.),
        0.,
        0.,
    )
}
#[test]
fn auto_playback_runs_between_update_callbacks_and_collision_detection() {
    let mesh = mesh();
    let clip = mesh.borrow().motions[0].clone();
    let mut scene = NodeScene::from_mesh(&mesh).unwrap();
    let root = scene.root.clone();
    let animated = part(&root);
    animated.borrow_mut().collider = Some(trigger());
    let other = Node::new();
    other.borrow_mut().transform = Mat4::from_translation(&Vec3 {
        x: 10.,
        y: 3.4,
        z: 0.,
    });
    other.borrow_mut().collider = Some(trigger());
    Node::add_child(&root, &other);
    let before = Rc::new(RefCell::new(Vec::new()));
    let captured = before.clone();
    let n = animated.clone();
    scene.on_update(&root, move |_, _| {
        captured.borrow_mut().push(y(&n));
        Ok(())
    });
    let hits = Rc::new(Cell::new(0));
    let captured = hits.clone();
    scene.on_collide(&animated, move |_, node, _, _| {
        close(Node::world_transform_value(node).pos_value().y, 3.4);
        captured.set(captured.get() + 1);
        Ok(())
    });
    scene.play_motion(&root, &clip, true, 15., 0.).unwrap();
    close(y(&animated), 3.);
    scene.update(0.).unwrap();
    close(y(&animated), 3.4);
    assert_eq!(&*before.borrow(), &[3.]);
    assert_eq!(hits.get(), 1);
    close(mesh.borrow().transforms[1].borrow().pos_value().y, 3.);
}
#[test]
fn independent_instances_do_not_mutate_the_source_or_each_other() {
    let mesh = mesh();
    let clip = mesh.borrow().motions[0].clone();
    let root = Node::new();
    let mut scene = NodeScene::new(root.clone());
    let a = scene.attach_mesh(&root, &mesh).unwrap();
    let b = scene.attach_mesh(&root, &mesh).unwrap();
    scene.play_motion(&a, &clip, true, 15., 0.).unwrap();
    scene.play_motion(&b, &clip, true, 0., 30.).unwrap();
    scene.update(1. / 30.).unwrap();
    close(y(&part(&a)), 3.4);
    close(y(&part(&b)), 3.8);
    close(mesh.borrow().transforms[1].borrow().pos_value().y, 3.);
    scene.stop_motion(&a);
    scene.stop_motion(&b);
    let moved = part(&a);
    Node::add_child(&b, &moved);
    scene.apply_motion(&b, &clip, 0., false).unwrap();
    close(y(&moved), 3.4);
    scene.apply_motion(&moved, &clip, 30., false).unwrap();
    close(y(&moved), 3.8);
}
#[test]
fn stopping_freezes_pose_and_sampling_does_not_replace_an_active_player() {
    let mesh = mesh();
    let clip = mesh.borrow().motions[0].clone();
    let mut scene = NodeScene::from_mesh(&mesh).unwrap();
    let root = scene.root.clone();
    let animated = part(&root);
    scene.play_motion(&root, &clip, true, 15., 0.).unwrap();
    scene.update(0.).unwrap();
    scene.stop_motion(&root);
    scene.update(0.).unwrap();
    close(y(&animated), 3.4);
    assert!(scene.motion_state(&root).is_none());
    scene.play_motion(&root, &clip, true, 15., 0.).unwrap();
    scene.update(0.).unwrap();
    scene.apply_motion(&root, &clip, 45., false).unwrap();
    close(y(&animated), 3.4);
    close(scene.motion_state(&root).unwrap().frame, 15.);
    scene.update(0.).unwrap();
    close(y(&animated), 3.8);
}
#[test]
fn switching_clips_restores_animated_parts_and_preserves_unanimated_edits() {
    let mesh = mesh();
    let clips = mesh.borrow().motions.clone();
    let mut scene = NodeScene::from_mesh(&mesh).unwrap();
    let root = scene.root.clone();
    let animated = part(&root);
    let static_part = Node::children(&animated)[0].clone();
    static_part.borrow_mut().transform = Mat4::from_translation(&Vec3 {
        x: 7.,
        y: 0.,
        z: 0.,
    });
    scene.play_motion(&root, &clips[0], true, 0., 30.).unwrap();
    close(y(&animated), 3.8);
    scene.play_motion(&root, &clips[1], true, 0., 15.).unwrap();
    close(y(&animated), 3.);
    close(static_part.borrow().transform.borrow().pos_value().x, 7.);
    let offset = Node::find_by_name(&root, "Offset root").pop().unwrap();
    close(
        offset.borrow().transform.borrow().get(0, 0),
        std::f32::consts::FRAC_1_SQRT_2,
    );
}
#[test]
fn reverse_zero_speed_and_nonlooping_endpoints_match_frame_units() {
    let mesh = mesh();
    let clips = mesh.borrow().motions.clone();
    let mut scene = NodeScene::from_mesh(&mesh).unwrap();
    let root = scene.root.clone();
    let animated = part(&root);
    scene
        .play_motion(&root, &clips[0], false, -15., 30.)
        .unwrap();
    scene.update(0.001).unwrap();
    close(y(&animated), 3.4);
    scene.update(1.).unwrap();
    close(y(&animated), 3.);
    assert!(scene.motion_state(&root).is_none());
    scene
        .play_motion(&root, &clips[1], false, 15., 45.)
        .unwrap();
    scene.update(0.).unwrap();
    assert!(scene.motion_state(&root).is_none());
    let offset = Node::find_by_name(&root, "Offset root").pop().unwrap();
    close(offset.borrow().transform.borrow().get(0, 0), -1.);
    scene.play_motion(&root, &clips[0], true, 0., 15.).unwrap();
    scene.update(0.).unwrap();
    close(y(&animated), 3.4);
    close(scene.motion_state(&root).unwrap().frame, 15.);
    scene.play_motion(&root, &clips[0], true, 0., -15.).unwrap();
    close(scene.motion_state(&root).unwrap().frame, 45.);
}
#[test]
fn active_cascade_pauses_players_while_visibility_does_not() {
    let mesh = mesh();
    let clip = mesh.borrow().motions[0].clone();
    let mut scene = NodeScene::from_mesh(&mesh).unwrap();
    let root = scene.root.clone();
    scene.play_motion(&root, &clip, true, 15., 0.).unwrap();
    root.borrow_mut().active = false;
    scene.update(0.).unwrap();
    close(scene.motion_state(&root).unwrap().frame, 0.);
    root.borrow_mut().active = true;
    root.borrow_mut().visible = false;
    scene.update(0.).unwrap();
    close(scene.motion_state(&root).unwrap().frame, 15.);
    let parent = Node::new();
    Node::add_child(&parent, &root);
    parent.borrow_mut().active = false;
    scene.update(0.).unwrap();
    close(scene.motion_state(&root).unwrap().frame, 15.);
}
#[test]
fn parent_and_child_players_override_in_preorder_and_sampling_is_subtree_scoped() {
    let mesh = mesh();
    let clip = mesh.borrow().motions[0].clone();
    let turn = mesh.borrow().motions[1].clone();
    let mut scene = NodeScene::from_mesh(&mesh).unwrap();
    let root = scene.root.clone();
    let animated = part(&root);
    scene.play_motion(&root, &clip, true, 0., 15.).unwrap();
    scene.play_motion(&animated, &clip, true, 0., 30.).unwrap();
    scene.update(0.).unwrap();
    close(y(&animated), 3.8);
    let offset = Node::find_by_name(&root, "Offset root").pop().unwrap();
    let before = offset.borrow().transform.borrow().data;
    scene.apply_motion(&animated, &turn, 30., false).unwrap();
    assert_eq!(offset.borrow().transform.borrow().data, before);
}
#[test]
fn bad_source_and_parameters_preserve_pose_and_existing_player() {
    let mesh = mesh();
    let clip = mesh.borrow().motions[0].clone();
    let mut scene = NodeScene::from_mesh(&mesh).unwrap();
    let root = scene.root.clone();
    scene.play_motion(&root, &clip, true, 0., 15.).unwrap();
    let before = part(&root).borrow().transform.borrow().data;
    let state = scene.motion_state(&root);
    let foreign = crate::mesh().borrow().motions[0].clone();
    assert!(scene.play_motion(&root, &foreign, true, 1., 0.).is_err());
    assert!(scene.apply_motion(&root, &foreign, 0., true).is_err());
    assert!(scene
        .play_motion(&Node::new(), &clip, true, 1., 0.)
        .is_err());
    for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(scene.play_motion(&root, &clip, true, bad, 0.).is_err());
        assert!(scene.play_motion(&root, &clip, true, 1., bad).is_err());
        assert!(scene.apply_motion(&root, &clip, bad, true).is_err());
    }
    assert_eq!(scene.motion_state(&root), state);
    assert_eq!(part(&root).borrow().transform.borrow().data, before);
}
#[test]
fn large_loop_playheads_keep_small_steps_and_finite_overflow_wraps() {
    let mesh = mesh();
    let clip = mesh.borrow().motions[0].clone();
    let mut scene = NodeScene::from_mesh(&mesh).unwrap();
    let root = scene.root.clone();
    scene
        .play_motion(&root, &clip, true, 1., 67_108_864.)
        .unwrap();
    scene.update(0.).unwrap();
    close(scene.motion_state(&root).unwrap().frame, 5.);
    clip.borrow_mut().length = f32::MAX;
    scene
        .play_motion(&root, &clip, true, f32::MAX, f32::MAX * 0.75)
        .unwrap();
    scene.update(0.).unwrap();
    let frame = scene.motion_state(&root).unwrap().frame;
    assert!(frame.is_finite() && frame >= 0. && frame < f32::MAX);
}
