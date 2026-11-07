//! DOM text composition and drop events, without granting filesystem access.
use super::{get_canvas, state};
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CompositionEvent, DragEvent, HtmlTextAreaElement, InputEvent};
fn editor() -> Option<HtmlTextAreaElement> {
    web_sys::window()?
        .document()?
        .get_element_by_id("pyxel_text_input")?
        .dyn_into()
        .ok()
}
#[wasm_bindgen]
pub fn browser_text_input(enabled: bool) -> Result<(), JsValue> {
    let target = editor().ok_or_else(|| JsValue::from_str("Text input is not initialized"))?;
    if enabled {
        target.focus()?;
    } else if let Some(canvas) = get_canvas() {
        canvas.focus()?;
    }
    Ok(())
}
pub fn register() {
    let Some(document) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    let Ok(element) = document.create_element("textarea") else {
        return;
    };
    let Ok(target) = element.dyn_into::<HtmlTextAreaElement>() else {
        return;
    };
    target.set_id("pyxel_text_input");
    target.set_tab_index(-1);
    let _ = target.set_attribute("aria-label", "ゲームへの文字入力");
    let _ = target.set_attribute(
        "style",
        "position:fixed;left:0;top:0;width:1px;height:1px;opacity:0;pointer-events:none",
    );
    let _ = document.body().and_then(|b| b.append_child(&target).ok());
    let composing = Rc::new(Cell::new(false));
    let flag = composing.clone();
    let start = Closure::<dyn FnMut(CompositionEvent)>::new(move |_| flag.set(true));
    let _ =
        target.add_event_listener_with_callback("compositionstart", start.as_ref().unchecked_ref());
    start.forget();
    let flag = composing.clone();
    let input_target = target.clone();
    let input = Closure::<dyn FnMut(InputEvent)>::new(move |event: InputEvent| {
        if flag.get() || event.is_composing() {
            return;
        }
        state().input.input_text.push_str(&input_target.value());
        input_target.set_value("");
    });
    let _ = target.add_event_listener_with_callback("input", input.as_ref().unchecked_ref());
    input.forget();
    let input_target = target.clone();
    let end = Closure::<dyn FnMut(CompositionEvent)>::new(move |event: CompositionEvent| {
        composing.set(false);
        let value = event.data().unwrap_or_else(|| input_target.value());
        state().input.input_text.push_str(&value);
        input_target.set_value("");
    });
    let _ = target.add_event_listener_with_callback("compositionend", end.as_ref().unchecked_ref());
    end.forget();
    if let Some(canvas) = get_canvas() {
        let over = Closure::<dyn FnMut(DragEvent)>::new(|event: DragEvent| event.prevent_default());
        let _ = canvas.add_event_listener_with_callback("dragover", over.as_ref().unchecked_ref());
        over.forget();
        let drop = Closure::<dyn FnMut(DragEvent)>::new(|event: DragEvent| {
            event.prevent_default();
            let Some(files) = event.data_transfer().and_then(|d| d.files()) else {
                return;
            };
            let files: Vec<_> = (0..files.length()).filter_map(|i| files.get(i)).collect();
            wasm_bindgen_futures::spawn_local(async move {
                let mut payload = Vec::new();
                for file in files {
                    let Ok(buffer) =
                        wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await
                    else {
                        continue;
                    };
                    payload.push((file.name(), js_sys::Uint8Array::new(&buffer).to_vec()));
                }
                state().input.dropped_files =
                    payload.iter().map(|(name, _)| name.clone()).collect();
                state().dropped_bytes = payload;
            });
        });
        let _ = canvas.add_event_listener_with_callback("drop", drop.as_ref().unchecked_ref());
        drop.forget();
    }
}
