//! Two independent Node instances using clips from one shared Mesh.
use crate::{
    api::*,
    cube::{Camera, Mat4, Mesh, Node, RcMesh, RcNode, Vec3},
};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
struct Demo {
    scene: NodeScene,
    mesh: RcMesh,
    actors: [RcNode; 2],
    updates: u32,
    error: Option<String>,
}
thread_local! {static DEMO:RefCell<Option<Demo>>=const{RefCell::new(None)};}
fn err(e: String) -> JsValue {
    JsValue::from_str(&e)
}
#[wasm_bindgen]
pub fn start_node_motion() -> Result<(), JsValue> {
    init(320, 192, "Pyxel Rust / Node motion", 30);
    let mesh = Mesh::from_glb_bytes(
        include_bytes!("../../docs/examples/retro3d/assets/animated-pyramid.glb"),
        None,
        30.,
    )
    .map_err(err)?;
    let root = Node::new();
    let camera = Camera::new();
    camera.borrow_mut().clear_color = Some(1);
    root.borrow_mut().camera = Some(camera);
    let mut scene = NodeScene::new(root.clone());
    let actors = [
        scene.attach_mesh(&root, &mesh).map_err(err)?,
        scene.attach_mesh(&root, &mesh).map_err(err)?,
    ];
    for (i, actor) in actors.iter().enumerate() {
        actor.borrow_mut().transform = Mat4::from_translation(&Vec3 {
            x: -13. + 6. * i as f32,
            y: -4.,
            z: -6.,
        });
    }
    let clips = mesh.borrow().motions.clone();
    scene
        .play_motion(&actors[0], &clips[0], true, 1., 0.)
        .map_err(err)?;
    scene
        .play_motion(&actors[1], &clips[1], true, 0.5, 0.)
        .map_err(err)?;
    DEMO.with(|v| {
        *v.borrow_mut() = Some(Demo {
            scene,
            mesh,
            actors,
            updates: 0,
            error: None,
        })
    });
    run(
        || {
            DEMO.with(|v| {
                if let Some(d) = v.borrow_mut().as_mut() {
                    if let Err(e) = d.scene.update(1. / 30.) {
                        d.error = Some(e);
                    } else {
                        d.updates += 1;
                    }
                }
            })
        },
        || {
            cls(1);
            DEMO.with(|v| {
                if let Some(d) = v.borrow_mut().as_mut() {
                    if let Err(e) = d.scene.draw([0., 24., 320., 144.], None) {
                        d.error = Some(e);
                    }
                }
            });
            text(8., 8., "NODE MOTION / SHARED MESH", 7);
            text(84., 170., "ACTOR A", 10);
            text(208., 170., "ACTOR B", 12);
        },
    );
    Ok(())
}
fn operation(f: impl FnOnce(&mut Demo) -> Result<(), String>) -> Result<(), JsValue> {
    DEMO.with(|v| {
        f(v.borrow_mut()
            .as_mut()
            .ok_or("Demo is not initialized")
            .map_err(str::to_owned)?)
    })
    .map_err(err)
}
#[wasm_bindgen]
pub fn node_motion_demo_play(
    actor: u32,
    clip: u32,
    looping: bool,
    speed: f32,
    frame: f32,
) -> Result<(), JsValue> {
    operation(|d| {
        let node = d.actors.get(actor as usize).ok_or("Invalid actor")?.clone();
        let motion = d
            .mesh
            .borrow()
            .motions
            .get(clip as usize)
            .cloned()
            .ok_or("Invalid clip")?;
        d.scene.play_motion(&node, &motion, looping, speed, frame)
    })
}
#[wasm_bindgen]
pub fn node_motion_demo_stop(actor: u32) -> Result<(), JsValue> {
    operation(|d| {
        let node = d.actors.get(actor as usize).ok_or("Invalid actor")?.clone();
        d.scene.stop_motion(&node);
        Ok(())
    })
}
#[wasm_bindgen]
pub fn node_motion_demo_sample(actor: u32, clip: u32, frame: f32) -> Result<(), JsValue> {
    operation(|d| {
        let node = d.actors.get(actor as usize).ok_or("Invalid actor")?.clone();
        let motion = d
            .mesh
            .borrow()
            .motions
            .get(clip as usize)
            .cloned()
            .ok_or("Invalid clip")?;
        d.scene.apply_motion(&node, &motion, frame, false)?;
        d.scene.stop_motion(&node);
        Ok(())
    })
}
#[wasm_bindgen]
pub fn node_motion_demo_active(actor: u32, active: bool) -> Result<(), JsValue> {
    operation(|d| {
        d.actors
            .get(actor as usize)
            .ok_or("Invalid actor")?
            .borrow_mut()
            .active = active;
        Ok(())
    })
}
#[wasm_bindgen]
pub fn node_motion_demo_info() -> String {
    DEMO.with(|v|v.borrow().as_ref().map_or_else(||"{}".into(),|d|serde_json::json!({"updates":d.updates,"error":d.error,"source_y":d.mesh.borrow().transforms[1].borrow().pos_value().y,"actors":d.actors.iter().map(|node|{
        let state=d.scene.motion_state(node);let part=Node::find_by_name(node,"Textured pyramid").pop().unwrap();let root=Node::find_by_name(node,"Offset root").pop().unwrap();
        serde_json::json!({"playing":state.is_some(),"frame":state.as_ref().map(|s|s.frame),"clip":state.as_ref().map(|s|s.name.clone()),"speed":state.as_ref().map(|s|s.speed),"looping":state.as_ref().map(|s|s.looping),"active":node.borrow().active,"part_y":part.borrow().transform.borrow().pos_value().y,"root_xx":root.borrow().transform.borrow().get(0,0)})
    }).collect::<Vec<_>>()} ).to_string()))
}
