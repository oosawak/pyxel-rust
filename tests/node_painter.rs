#![cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
use pyxel_rust::{
    api::NodeScene,
    cube::{Camera, Mat4, Node, Vec3},
    Image,
};
fn translation(x: f32, y: f32, z: f32) -> Mat4 {
    *Mat4::from_translation(&Vec3 { x, y, z }).borrow()
}
fn setup() -> (NodeScene, pyxel_rust::cube::RcNode, pyxel_rust::RcImage) {
    let root = Node::new();
    let camera = Camera::new();
    camera.borrow_mut().ortho_size = Some(8.);
    camera.borrow_mut().clear_color = Some(1);
    root.borrow_mut().camera = Some(camera);
    let child = Node::new();
    Node::add_child(&root, &child);
    (NodeScene::new(root), child, Image::new(64, 64))
}
#[test]
fn local_matrices_compose_with_ancestors_and_repeated_commands_keep_local_alpha() {
    let (mut scene, node, image) = setup();
    scene.root.borrow_mut().transform = Mat4::from_translation(&Vec3 {
        x: -2.,
        y: 0.,
        z: -8.,
    });
    scene.root.borrow_mut().opacity = 0.5;
    node.borrow_mut().transform = Mat4::from_translation(&Vec3 {
        x: 1.,
        y: 0.,
        z: 0.,
    });
    node.borrow_mut().opacity = 0.5;
    scene.on_paint(&node, |_, p| {
        p.shaded(false).dither(0.5);
        p.rect(&translation(-2., 0., 0.), 1., 2., 8);
        p.rect(&translation(0., 0., 0.), 1., 2., 8);
        p.rect(&translation(2., 0., 0.), 1., 2., 8);
        Ok(())
    });
    scene.draw([0., 0., 64., 64.], Some(&image)).unwrap();
    let pixels = image.borrow();
    let counts: Vec<_> = [8, 24, 40]
        .map(|center| {
            (center - 4..center + 4)
                .flat_map(|x| (24..40).map(move |y| (x, y)))
                .filter(|&(x, y)| pixels.pixel(x as f32, y as f32) == 8)
                .count()
        })
        .into();
    assert!(
        counts[0] > 0 && counts[0] < 40,
        "effective alpha should be 0.125: {counts:?}"
    );
    assert_eq!(
        counts,
        vec![counts[0]; 3],
        "later commands must not multiply opacity again"
    );
    assert_eq!(
        pixels.pixel(56., 32.),
        1,
        "node and local translations must both apply"
    );
}
#[test]
fn depth_modifiers_control_overdraw_and_reset_before_the_next_node() {
    let (mut scene, node, image) = setup();
    node.borrow_mut().transform = Mat4::from_translation(&Vec3 {
        x: 0.,
        y: 0.,
        z: -6.,
    });
    scene.on_paint(&node, |_, p| {
        p.shaded(false);
        // Front surface, then a farther surface with ordinary depth testing.
        p.rect(&translation(0., 0., 1.), 4., 4., 8);
        p.rect(&Mat4::identity_value(), 4., 4., 11);
        Ok(())
    });
    scene.draw([0., 0., 64., 64.], Some(&image)).unwrap();
    assert_eq!(image.borrow().pixel(32., 32.), 8);
    scene.on_paint(&node, |_, p| {
        p.shaded(false).depth_write(false);
        p.rect(&translation(0., 0., 1.), 4., 4., 8);
        p.depth_write(true);
        p.rect(&Mat4::identity_value(), 4., 4., 11);
        p.depth_test(false);
        p.rect(&translation(0., 0., -1.), 2., 2., 10);
        p.dither(0.).depth_offset(99.).decal(Some(4.));
        Ok(())
    });
    let sibling = Node::new();
    Node::add_child(&scene.root, &sibling);
    sibling.borrow_mut().transform = Mat4::from_translation(&Vec3 {
        x: 0.,
        y: 0.,
        z: -7.,
    });
    scene.on_paint(&sibling, |_, p| {
        p.rect(&Mat4::identity_value(), 1., 1., 14);
        Ok(())
    });
    scene.draw([0., 0., 64., 64.], Some(&image)).unwrap();
    assert_eq!(
        image.borrow().pixel(20., 32.),
        11,
        "disabled depth writes permit farther draws"
    );
    assert_eq!(
        image.borrow().pixel(32., 32.),
        10,
        "sibling depth test must reset to enabled"
    );
    scene.on_paint(&sibling, |_, p| {
        p.pset(
            &Vec3 {
                x: 3.,
                y: 0.,
                z: 0.,
            },
            14,
        );
        Ok(())
    });
    scene.draw([0., 0., 64., 64.], Some(&image)).unwrap();
    assert_eq!(
        image.borrow().pixel(56., 32.),
        14,
        "sibling dither, decal and offset must reset"
    );
}
#[test]
fn billboard_circle_survives_edge_on_parent_and_callbacks_can_be_replaced() {
    let (mut scene, node, image) = setup();
    let rotation = *Mat4::from_euler(&Vec3 {
        x: 0.,
        y: 90.,
        z: 0.,
    })
    .borrow();
    node.borrow_mut().transform = std::rc::Rc::new(std::cell::RefCell::new(
        translation(0., 0., -6.).mul_mat_value(&rotation),
    ));
    scene.on_paint(&node, |scene, p| {
        p.circ(
            &Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            1.,
            11,
        );
        scene.on_paint(p.node(), |_, p| {
            p.rect(&Mat4::identity_value(), 2., 2., 8);
            Ok(())
        });
        Ok(())
    });
    scene.draw([0., 0., 64., 64.], Some(&image)).unwrap();
    assert_eq!(image.borrow().pixel(32., 32.), 11);
    assert_eq!(image.borrow().pixel(36., 32.), 11);
    scene.draw([0., 0., 64., 64.], Some(&image)).unwrap();
    assert_eq!(
        image.borrow().pixel(36., 32.),
        1,
        "ordinary rectangle remains edge-on"
    );
}
#[test]
fn decal_projects_on_existing_depth_then_none_restores_ordinary_drawing() {
    let (mut scene, node, image) = setup();
    node.borrow_mut().transform = Mat4::from_translation(&Vec3 {
        x: 0.,
        y: 0.,
        z: -6.,
    });
    scene.on_paint(&node, |_, p| {
        p.shaded(false);
        p.rect(&Mat4::identity_value(), 4., 4., 8);
        p.decal(Some(2.));
        p.rect(&translation(0., 0., 0.5), 2., 2., 11);
        p.decal(None);
        p.rect(&translation(2., 0., 1.), 1., 1., 10);
        Ok(())
    });
    scene.draw([0., 0., 64., 64.], Some(&image)).unwrap();
    assert_eq!(image.borrow().pixel(32., 32.), 11);
    assert_eq!(image.borrow().pixel(20., 32.), 8);
    assert_eq!(image.borrow().pixel(48., 32.), 10);
    assert_eq!(image.borrow().pixel(8., 8.), 1);
}

#[test]
fn inherited_shading_can_be_disabled_and_depth_offset_changes_occlusion() {
    use pyxel_rust::cube::Shading;
    let (mut scene, node, image) = setup();
    let shading = Shading::new(&[0, 0xffffff]);
    // A controlled ramp makes inherited shading observable regardless of normal/light.
    for level in 0..8 {
        shading.borrow_mut().set(1, level, (14, 14));
    }
    scene.root.borrow_mut().shading = Some(shading);
    node.borrow_mut().transform = Mat4::from_translation(&Vec3 {
        x: 0.,
        y: 0.,
        z: -6.,
    });
    scene.on_paint(&node, |_, p| {
        p.rect(&Mat4::identity_value(), 4., 4., 1);
        p.shaded(false).depth_test(false);
        p.rect(&translation(2., 0., 0.), 1., 1., 8);
        Ok(())
    });
    scene.draw([0., 0., 64., 64.], Some(&image)).unwrap();
    assert_eq!(image.borrow().pixel(32., 32.), 14);
    assert_eq!(image.borrow().pixel(48., 32.), 8);
    scene.on_paint(&node, |_, p| {
        p.shaded(false).depth_offset(2.);
        p.rect(&translation(0., 0., 1.), 4., 4., 8);
        p.depth_offset(0.);
        p.rect(&Mat4::identity_value(), 2., 2., 11);
        Ok(())
    });
    scene.draw([0., 0., 64., 64.], Some(&image)).unwrap();
    assert_eq!(
        image.borrow().pixel(32., 32.),
        11,
        "offset pushes the physically closer face behind the second face"
    );
    assert_eq!(image.borrow().pixel(20., 32.), 8);
}
