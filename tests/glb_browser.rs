#![cfg(feature = "wasm-backend")]

use pyxel_rust::cube::{draw::DrawState, Camera, ColImage, Mat4, Mesh, Vec3};
use pyxel_rust::{
    api::{draw_mesh_3d, render_3d},
    Image,
};
use serde_json::Value;

const PYRAMID: &[u8] = include_bytes!("../docs/examples/retro3d/assets/pyramid.glb");

fn rewrite_document(change: impl FnOnce(&mut Value)) -> Vec<u8> {
    let json_len = u32::from_le_bytes(PYRAMID[12..16].try_into().unwrap()) as usize;
    let mut document: Value = serde_json::from_slice(&PYRAMID[20..20 + json_len]).unwrap();
    change(&mut document);
    let mut json = serde_json::to_vec(&document).unwrap();
    while json.len() % 4 != 0 {
        json.push(b' ');
    }
    let binary_chunk = &PYRAMID[20 + json_len..];
    let size = 12 + 8 + json.len() + binary_chunk.len();
    let mut bytes = b"glTF".to_vec();
    bytes.extend_from_slice(&2u32.to_le_bytes());
    bytes.extend_from_slice(&(size as u32).to_le_bytes());
    bytes.extend_from_slice(&(json.len() as u32).to_le_bytes());
    bytes.extend_from_slice(b"JSON");
    bytes.extend_from_slice(&json);
    bytes.extend_from_slice(binary_chunk);
    bytes
}

#[test]
fn imports_embedded_texture_and_composes_parent_child_transforms() {
    let mesh = Mesh::from_glb_bytes(PYRAMID, None, 30.0).unwrap();
    let mesh = mesh.borrow();
    assert_eq!(&mesh.parents[..2], &[-1, 0]);
    let primitive_part = mesh.primitives.iter().position(Option::is_some).unwrap();
    assert_eq!(mesh.parents[primitive_part], 1);
    assert_eq!(&mesh.names[..2], &["Offset root", "Textured pyramid"]);
    let bounds = mesh.local_aabb();
    assert_eq!(
        bounds.min,
        Vec3 {
            x: 9.0,
            y: 3.0,
            z: -1.0
        }
    );
    assert_eq!(
        bounds.max,
        Vec3 {
            x: 11.0,
            y: 5.0,
            z: 1.0
        }
    );
    let ColImage::Image(image) = &mesh.materials[0].col_img else {
        panic!("Texture was lost");
    };
    let image = image.borrow();
    assert_eq!((image.width(), image.height()), (2, 2));
    assert_eq!(image.pixel(0.0, 0.0), 10);
    assert_eq!(image.pixel(1.0, 0.0), 12);
}

#[test]
fn renders_imported_textured_mesh_into_an_image() {
    let mesh = Mesh::from_glb_bytes(PYRAMID, None, 30.0).unwrap();
    let target = Image::new(64, 64);
    let camera = Camera::new();
    camera.borrow_mut().clear_color = Some(1);
    // Center the independently known fixture bounds while placing it in front of the camera.
    let transform = Mat4::from_translation(&Vec3 {
        x: -10.0,
        y: -4.0,
        z: -6.0,
    });
    render_3d(&target, &camera, |ctx| {
        draw_mesh_3d(
            ctx,
            &mesh.borrow(),
            &transform.borrow(),
            DrawState::unshaded(),
        )
        .unwrap();
    });
    let image = target.borrow();
    let mut counts = [0usize; 16];
    for y in 0..64 {
        for x in 0..64 {
            counts[image.pixel(x as f32, y as f32) as usize] += 1;
        }
    }
    assert!(
        counts[10] > 10,
        "Yellow texture pixels did not reach the target"
    );
    assert!(
        counts[12] > 10,
        "Cyan texture pixels did not reach the target"
    );
    assert_eq!(image.pixel(0.0, 0.0), 1);
}

#[test]
fn rejects_truncated_data_and_invalid_animation_rate() {
    for bytes in [&PYRAMID[..8], &PYRAMID[..PYRAMID.len() - 4]] {
        assert!(Mesh::from_glb_bytes(bytes, None, 30.0).is_err());
    }
    for fps in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        assert!(Mesh::from_glb_bytes(PYRAMID, None, fps).is_err());
    }
}

#[test]
fn rejects_external_buffers_and_images_without_filesystem_access() {
    let bytes = rewrite_document(|doc| doc["buffers"][0]["uri"] = "outside.bin".into());
    let result = Mesh::from_glb_bytes(&bytes, None, 30.0);
    assert!(result.err().unwrap().contains("external buffers"));
    let bytes = rewrite_document(|doc| {
        doc["images"][0]
            .as_object_mut()
            .unwrap()
            .remove("bufferView");
        doc["images"][0]["uri"] = "https://example.invalid/outside.png".into();
    });
    let result = Mesh::from_glb_bytes(&bytes, None, 30.0);
    assert!(result.err().unwrap().contains("external images"));
}

#[test]
fn rejects_image_buffer_view_outside_the_binary_chunk() {
    let bytes = rewrite_document(|doc| doc["bufferViews"][3]["byteOffset"] = 1_000_000.into());
    assert!(Mesh::from_glb_bytes(&bytes, None, 30.0).is_err());
}
