#![cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
use pyxel_rust::{
    api::{with_node_draw_state, NodeScene},
    cube::{draw, Camera, Collider, Mat4, Mesh, Node, Primitive, Vec3},
    Image,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
fn sphere() -> pyxel_rust::cube::RcCollider {
    Collider::new(
        Vec3::zero(),
        1.,
        None,
        true,
        false,
        1.,
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
fn update_snapshots_membership_and_allows_callback_replacement() {
    let root = Node::new();
    let child = Node::new();
    Node::add_child(&root, &child);
    let hits = Rc::new(Cell::new(0));
    let count = hits.clone();
    let mut scene = NodeScene::new(root.clone());
    scene.on_update(&root, move |scene, node| {
        count.set(count.get() + 1);
        let count = count.clone();
        scene.on_update(node, move |_, _| {
            count.set(count.get() + 10);
            Ok(())
        });
        Ok(())
    });
    let c = hits.clone();
    scene.on_update(&child, move |_, _| {
        c.set(c.get() + 100);
        Ok(())
    });
    child.borrow_mut().active = false;
    scene.update(0.).unwrap();
    assert_eq!(hits.get(), 1);
    child.borrow_mut().active = true;
    scene.update(0.).unwrap();
    assert_eq!(hits.get(), 111);
    let child_to_detach = child.clone();
    scene.on_update(&root, move |_, _| {
        Node::detach(&child_to_detach);
        Ok(())
    });
    scene.update(0.).unwrap();
    assert_eq!(hits.get(), 111);
    let born = Rc::new(Cell::new(false));
    let c = hits.clone();
    scene.on_update(&root, move |scene, node| {
        if !born.replace(true) {
            let new = Node::new();
            Node::add_child(node, &new);
            let c = c.clone();
            scene.on_update(&new, move |_, _| {
                c.set(c.get() + 1000);
                Ok(())
            });
        }
        Ok(())
    });
    scene.update(0.).unwrap();
    assert_eq!(hits.get(), 111);
    scene.update(0.).unwrap();
    assert_eq!(hits.get(), 1111);
}
#[test]
fn motion_contacts_and_leaf_first_one_shot_destruction_follow_update_phase() {
    let root = Node::new();
    let a = Node::new();
    let leaf = Node::new();
    let b = Node::new();
    Node::add_child(&root, &a);
    Node::add_child(&a, &leaf);
    Node::add_child(&root, &b);
    a.borrow_mut().collider = Some(sphere());
    b.borrow_mut().collider = Some(sphere());
    b.borrow_mut().transform = Mat4::from_translation(&Vec3 {
        x: 1.,
        y: 0.,
        z: 0.,
    });
    a.borrow()
        .collider
        .as_ref()
        .unwrap()
        .borrow()
        .velocity
        .borrow_mut()
        .x = 0.1;
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut scene = NodeScene::new(root.clone());
    for (node, label) in [(&root, "u-root"), (&a, "u-a"), (&b, "u-b")] {
        let log = log.clone();
        scene.on_update(node, move |_, _| {
            log.borrow_mut().push(label);
            Ok(())
        });
    }
    let log_a = log.clone();
    scene.on_collide(&a, move |_, node, _, _| {
        assert!((Node::world_transform_value(node).pos_value().x - 0.1).abs() < 1e-5);
        log_a.borrow_mut().push("c-a");
        Node::destroy(node);
        Ok(())
    });
    let log_b = log.clone();
    scene.on_collide(&b, move |_, _, other, _| {
        assert!(other.borrow().destroyed);
        log_b.borrow_mut().push("c-b");
        Ok(())
    });
    for (node, label) in [(&a, "d-a"), (&leaf, "d-leaf")] {
        let log = log.clone();
        scene.on_destroy(node, move |_, node| {
            assert!(Node::parent(node).is_some());
            log.borrow_mut().push(label);
            Ok(())
        });
    }
    scene.update(0.1).unwrap();
    assert_eq!(
        &*log.borrow(),
        &["u-root", "u-a", "u-b", "c-a", "c-b", "d-leaf", "d-a"]
    );
    assert!(Node::parent(&a).is_none());
    assert!(Node::parent(&leaf).is_none());
    log.borrow_mut().clear();
    scene.update(0.1).unwrap();
    assert_eq!(&*log.borrow(), &["u-root", "u-b"]);
}
#[test]
fn destruction_error_does_not_repeat_notification_and_reentrant_update_is_rejected() {
    let root = Node::new();
    let child = Node::new();
    Node::add_child(&root, &child);
    let mut scene = NodeScene::new(root.clone());
    scene.on_update(&root, |scene, _| {
        assert!(scene.update(0.).is_err());
        Ok(())
    });
    let calls = Rc::new(Cell::new(0));
    let c = calls.clone();
    scene.on_destroy(&child, move |_, _| {
        c.set(c.get() + 1);
        Err("expected".into())
    });
    Node::destroy(&child);
    assert_eq!(scene.update(0.).unwrap_err(), "expected");
    scene.update(0.).unwrap();
    assert_eq!(calls.get(), 1);
    assert!(Node::parent(&child).is_none());
    assert!(scene.update(f32::NAN).is_err());
}
#[test]
fn draw_inherits_camera_visibility_opacity_and_resets_modifiers_between_siblings() {
    let root = Node::new();
    let a = Node::new();
    let b = Node::new();
    Node::add_child(&root, &a);
    Node::add_child(&root, &b);
    let camera = Camera::new();
    camera.borrow_mut().clear_color = Some(1);
    root.borrow_mut().camera = Some(camera.clone());
    b.borrow_mut().transform = Mat4::from_translation(&Vec3 {
        x: 0.,
        y: 0.,
        z: -4.,
    });
    let mut scene = NodeScene::new(root.clone());
    scene.on_draw(&a, |_, _, ctx| {
        ctx.dither_alpha = 0.;
        ctx.depth_test = false;
        ctx.depth_write = false;
        ctx.shaded = false;
        Ok(())
    });
    scene.on_draw(&b, |_, node, ctx| {
        assert_eq!(ctx.dither_alpha, 1.);
        assert!(ctx.depth_test && ctx.depth_write && ctx.shaded);
        with_node_draw_state(ctx, node, |ctx, world, state| {
            draw::rect(ctx, world, 2., 2., 8, state)
        });
        Ok(())
    });
    let target = Image::new(128, 64);
    target.borrow_mut().clear(12);
    scene.draw([32., 16., 64., 32.], Some(&target)).unwrap();
    assert_eq!(target.borrow().pixel(64., 32.), 8);
    assert_eq!(target.borrow().pixel(0., 0.), 12);
    assert_eq!(camera.borrow().depth.len(), 128 * 64);
    b.borrow_mut().visible = false;
    scene.draw([32., 16., 64., 32.], Some(&target)).unwrap();
    assert_eq!(target.borrow().pixel(64., 32.), 1);
    b.borrow_mut().visible = true;
    root.borrow_mut().opacity = 0.;
    scene.draw([32., 16., 64., 32.], Some(&target)).unwrap();
    assert_eq!(target.borrow().pixel(64., 32.), 1);
    assert!(scene.draw([0., 0., 0., 10.], Some(&target)).is_err());
}
#[test]
fn mesh_conversion_preserves_named_hierarchy_shared_transforms_and_material() {
    let mesh = Mesh::new();
    {
        let mut m = mesh.borrow_mut();
        m.primitives = vec![None, Some(Primitive::plane(2., 2.))];
        m.parents = vec![-1, 0];
        m.names = vec!["parent".into(), "face".into()];
        m.transforms = vec![
            Mat4::from_translation(&Vec3 {
                x: 0.,
                y: 0.,
                z: -4.,
            }),
            Mat4::identity(),
        ];
        m.col_img = pyxel_rust::cube::mesh::ColImage::Color(11);
    }
    let mut scene = NodeScene::from_mesh(&mesh).unwrap();
    scene.root.borrow_mut().camera = Some(Camera::new());
    let parent = Node::find_by_name(&scene.root, "parent").pop().unwrap();
    let face = Node::find_by_name(&scene.root, "face").pop().unwrap();
    assert!(Rc::ptr_eq(&Node::parent(&face).unwrap(), &parent));
    assert!(Rc::ptr_eq(
        &face.borrow().transform,
        &mesh.borrow().transforms[1]
    ));
    let target = Image::new(64, 64);
    scene.draw([0., 0., 64., 64.], Some(&target)).unwrap();
    assert_eq!(target.borrow().pixel(32., 32.), 11);
    mesh.borrow_mut().parents[1] = 99;
    let count = Node::children(&scene.root).len();
    let root = scene.root.clone();
    assert!(scene.attach_mesh(&root, &mesh).is_err());
    assert_eq!(Node::children(&root).len(), count);
}
#[test]
fn draw_errors_and_panics_return_camera_buffers_and_release_lifecycle_guard() {
    let root = Node::new();
    let camera = Camera::new();
    root.borrow_mut().camera = Some(camera.clone());
    let mut scene = NodeScene::new(root.clone());
    let target = Image::new(32, 32);
    scene.on_draw(&root, |_, _, _| Err("draw failed".into()));
    assert_eq!(
        scene.draw([0., 0., 32., 32.], Some(&target)).unwrap_err(),
        "draw failed"
    );
    assert_eq!(camera.borrow().depth.len(), 1024);
    scene.on_draw(&root, |_, _, _| panic!("test callback"));
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        scene.draw([0., 0., 32., 32.], Some(&target))
    }));
    assert!(panic.is_err());
    assert_eq!(camera.borrow().depth.len(), 1024);
    scene.remove_callbacks(&root);
    scene.draw([0., 0., 32., 32.], Some(&target)).unwrap();
    scene.update(0.).unwrap();
}
