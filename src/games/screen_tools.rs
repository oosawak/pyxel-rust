//! Shared screen, perspective planes, recording and manual presentation exercise.
use crate::api::*;
use wasm_bindgen::prelude::*;
fn err(e: String) -> JsValue {
    JsValue::from_str(&e)
}
#[wasm_bindgen]
pub fn start_screen_tools(manual: bool) {
    init(256, 192, "Pyxel Rust / Screen and recording", 30);
    images()[2].borrow_mut().clear(8);
    for y in 0..32 {
        for x in 0..32 {
            images()[2].borrow_mut().draw_rect(
                (x * 8) as f32,
                (y * 8) as f32,
                8.,
                8.,
                if (x + y) % 2 == 0 { 8 } else { 11 },
            );
        }
    }
    tilemaps()[1].borrow_mut().imgsrc = ImageSource::Index(2);
    for y in 0..32 {
        for x in 0..32 {
            tilemaps()[1]
                .borrow_mut()
                .set_tile(x as f32, y as f32, (x as u16, y as u16));
        }
    }
    screen_demo_draw();
    if !manual {
        run(update_nodes, screen_demo_draw);
    }
}
#[wasm_bindgen]
pub fn screen_demo_draw() {
    cls(1);
    let angle = sin(frame_count() as f32 * 2.) * 25.;
    let nodes = draw_nodes();
    if !nodes {
        blt3d(
            0.,
            24.,
            128.,
            128.,
            2,
            (128., 128., 180.),
            (angle, 0., 0.),
            Some(60.),
            None,
        )
        .unwrap();
        bltm3d(
            128.,
            24.,
            128.,
            128.,
            1,
            (128., 128., 180.),
            (angle, 0., 0.),
            Some(60.),
            None,
        )
        .unwrap();
    }
    text(
        8.,
        8.,
        if nodes {
            "NODE CALLBACKS / MESH"
        } else {
            "IMAGE PLANE / TILEMAP PLANE"
        },
        7,
    );
    text(8., 174., "SHARED SCREEN / GIF", 11);
    // A direct handle edit must reach the presented screen.
    screen()
        .borrow_mut()
        .draw_rect((frame_count() % 220 + 8) as f32, 158., 12., 6., 10);
}
#[wasm_bindgen]
pub fn screen_demo_info() -> String {
    let (frame, running, mode, recorded, width, height) = {
        let s = crate::backend::wasm_backend::state();
        (
            s.frame_count,
            s.running,
            s.screen_mode,
            s.recorder.as_ref().map_or(0, |r| r.frame_count()),
            s.width,
            s.height,
        )
    };
    serde_json::json!({"frame":frame,"running":running,"mode":mode,"nodes":COUNTS.with(|v|{let c=v.borrow();serde_json::json!({"update":c[0],"draw":c[1],"collide":c[2],"destroy":c[3]})}),"node_children":NODES.with(|v|v.borrow().as_ref().map_or(0,|s|Node::children(&s.root).len())),"recorded":recorded,"screen_width":width,"screen_height":height}).to_string()
}
#[wasm_bindgen]
pub fn screen_demo_edit() {
    screen().borrow_mut().draw_rect(8., 24., 32., 24., 14);
}
#[wasm_bindgen]
pub fn screen_demo_font() {
    font().borrow_mut().draw_rect(0., 0., 4., 6., 1);
}
#[wasm_bindgen]
pub fn screen_demo_cursor(visible: bool) {
    cursor().borrow_mut().clear(11);
    mouse(visible);
}
#[wasm_bindgen]
pub fn screen_demo_pixel(x: f32, y: f32) -> u8 {
    pget(x, y)
}
#[wasm_bindgen]
pub fn screen_demo_store(value: Option<String>) -> Result<Option<String>, JsValue> {
    let store = user_data_store("pyxel-rust", "screen-tools").map_err(err)?;
    if let Some(value) = value {
        store.set("memo", &value).map_err(err)?;
    }
    store.get("memo").map_err(err)
}
#[wasm_bindgen]
pub fn screen_demo_remove_store() -> Result<(), JsValue> {
    user_data_store("pyxel-rust", "screen-tools")
        .map_err(err)?
        .remove("memo")
        .map_err(err)
}

use crate::cube::{Camera, Collider, Mat4, Mesh, Node, Primitive, Vec3};
use std::cell::RefCell;
thread_local! {
    static NODES:RefCell<Option<NodeScene>>=const{RefCell::new(None)};
    static COUNTS:RefCell<[u32;4]>=const{RefCell::new([0;4])};
}
pub fn update_nodes() {
    NODES.with(|scene| {
        if let Some(scene) = scene.borrow_mut().as_mut() {
            scene.update(1. / 30.).unwrap();
        }
    });
}
fn draw_nodes() -> bool {
    NODES.with(|scene| {
        if let Some(scene) = scene.borrow_mut().as_mut() {
            scene.draw([0., 24., 256., 128.], None).unwrap();
            true
        } else {
            false
        }
    })
}
fn count(index: usize) {
    COUNTS.with(|v| v.borrow_mut()[index] += 1);
}
#[wasm_bindgen]
pub fn screen_demo_nodes(enabled: bool) {
    COUNTS.with(|v| *v.borrow_mut() = [0; 4]);
    if !enabled {
        NODES.with(|v| *v.borrow_mut() = None);
        return;
    }
    let mesh = Mesh::new();
    {
        let mut m = mesh.borrow_mut();
        m.primitives = vec![
            Some(Primitive::r#box(&Vec3 {
                x: 2.,
                y: 2.,
                z: 2.,
            })),
            Some(Primitive::r#box(&Vec3 {
                x: 2.,
                y: 2.,
                z: 2.,
            })),
        ];
        m.transforms = vec![
            Mat4::from_translation(&Vec3 {
                x: -1.,
                y: 0.,
                z: -8.,
            }),
            Mat4::from_translation(&Vec3 {
                x: 1.,
                y: 0.,
                z: -9.,
            }),
        ];
        m.parents = vec![-1, -1];
        m.names = vec!["left".into(), "right".into()];
        use crate::cube::mesh::{ColImage, Material};
        m.materials = vec![
            Material {
                col_img: ColImage::Color(8),
                colkey: None,
            },
            Material {
                col_img: ColImage::Color(11),
                colkey: None,
            },
        ];
        m.material_indices = vec![Some(0), Some(1)];
    }
    let mut scene = NodeScene::from_mesh(&mesh).unwrap();
    let root = scene.root.clone();
    root.borrow_mut().camera = Some(Camera::new());
    scene.on_update(&root, |_, _| {
        count(0);
        Ok(())
    });
    for node in Node::children(&root) {
        node.borrow_mut().collider = Some(Collider::new(
            Vec3::zero(),
            1.5,
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
        ));
        scene.on_draw(&node, |_, _, _| {
            count(1);
            Ok(())
        });
        scene.on_collide(&node, |_, _, _, _| {
            count(2);
            Ok(())
        });
        scene.on_destroy(&node, |_, _| {
            count(3);
            Ok(())
        });
    }
    let left = Node::children(&root)[0].clone();
    scene.on_update(&left, |_, node| {
        node.borrow_mut().transform = Mat4::from_translation(&Vec3 {
            x: -1. + sin(frame_count() as f32 * 3.) * 0.75,
            y: 0.,
            z: -8.,
        });
        Ok(())
    });
    NODES.with(|v| *v.borrow_mut() = Some(scene));
}
#[wasm_bindgen]
pub fn screen_demo_node_flags(active: bool, visible: bool, opacity: f32) {
    NODES.with(|s| {
        if let Some(scene) = s.borrow().as_ref() {
            let mut root = scene.root.borrow_mut();
            root.active = active;
            root.visible = visible;
            root.opacity = opacity.clamp(0., 1.);
        }
    });
}
#[wasm_bindgen]
pub fn screen_demo_destroy_node() {
    NODES.with(|s| {
        if let Some(scene) = s.borrow().as_ref() {
            if let Some(node) = Node::children(&scene.root).first() {
                Node::destroy(node);
            }
        }
    });
}
