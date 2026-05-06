use wasm_bindgen::prelude::*;
use aes::Aes256;
use ctr::Ctr64BE;
use cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};

type Aes256Ctr = Ctr64BE<Aes256>;

/// AES-256-CTR でデータを復号する（先頭から）
#[wasm_bindgen]
pub fn decrypt_chunk(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, JsValue> {
    decrypt_chunk_at(data, key, iv, 0)
}

/// AES-256-CTR でデータをオフセット指定で復号する（ストリーミング用）
///
/// # Arguments
/// * `data`        - 暗号化されたバイト列
/// * `key`         - 32バイトのAESキー
/// * `iv`          - 16バイトのIV（ノンス）
/// * `byte_offset` - ストリーム上のバイトオフセット（前チャンクの合計バイト数）
#[wasm_bindgen]
pub fn decrypt_chunk_at(data: &[u8], key: &[u8], iv: &[u8], byte_offset: u64) -> Result<Vec<u8>, JsValue> {
    if key.len() != 32 {
        return Err(JsValue::from_str("key must be 32 bytes"));
    }
    if iv.len() != 16 {
        return Err(JsValue::from_str("iv must be 16 bytes"));
    }

    let key_arr: &[u8; 32] = key.try_into()
        .map_err(|_| JsValue::from_str("invalid key length"))?;
    let iv_arr: &[u8; 16] = iv.try_into()
        .map_err(|_| JsValue::from_str("invalid iv length"))?;

    let mut cipher = Aes256Ctr::new(key_arr.into(), iv_arr.into());

    // オフセット分だけキーストリームをスキップ（CTRモードはシーク可能）
    if byte_offset > 0 {
        cipher.seek(byte_offset);
    }

    let mut buf = data.to_vec();
    cipher.apply_keystream(&mut buf);
    Ok(buf)
}
