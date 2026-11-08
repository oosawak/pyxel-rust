#![cfg(any(feature = "wasm-backend", feature = "pyxel-core-backend"))]
use pyxel_rust::cube::{Mat4, Quat, Vec3};
#[test]
fn decomposed_getters_return_independent_values_without_modifying_the_matrix() {
    let rotation = Quat::identity();
    let matrix = Mat4::compose(
        &Vec3 {
            x: 2.,
            y: 3.,
            z: 4.,
        },
        &rotation.borrow(),
        &Vec3 {
            x: 5.,
            y: 6.,
            z: 7.,
        },
    );
    let original = matrix.borrow().data;
    let pos = matrix.borrow().pos();
    let rot = matrix.borrow().rot();
    let scale = matrix.borrow().scale_vec();
    assert_eq!(
        (pos.borrow().x, pos.borrow().y, pos.borrow().z),
        (2., 3., 4.)
    );
    assert_eq!(
        (scale.borrow().x, scale.borrow().y, scale.borrow().z),
        (5., 6., 7.)
    );
    assert!((rot.borrow().w - 1.).abs() < 1e-6);
    pos.borrow_mut().x = 99.;
    rot.borrow_mut().w = 0.;
    scale.borrow_mut().y = 99.;
    assert_eq!(matrix.borrow().data, original);
}
