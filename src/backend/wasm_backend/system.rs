//! Browser equivalents of Pyxel window operations.
use super::{get_canvas, state};
use std::cell::RefCell;
use wasm_bindgen::{prelude::*, JsCast};
thread_local! {static WIDTH_STYLE:RefCell<Option<String>>=const{RefCell::new(None)};static ERROR:RefCell<Option<String>>=const{RefCell::new(None)};}
pub fn title(text: &str) {
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        document.set_title(text);
    }
}
pub fn resize(width: u32, height: u32) -> Result<(), String> {
    state().resize(width, height)?;
    if let Some(canvas) = get_canvas() {
        canvas.set_width(width * state().display_scale);
        canvas.set_height(height * state().display_scale);
    }
    apply_integer_scale();
    Ok(())
}
pub fn integer_scale(enabled: bool) {
    if let Some(canvas) = get_canvas() {
        if enabled && !state().integer_scale {
            WIDTH_STYLE.with(|s| {
                *s.borrow_mut() = Some(
                    canvas
                        .style()
                        .get_property_value("width")
                        .unwrap_or_default(),
                )
            });
        }
        if !enabled && state().integer_scale {
            WIDTH_STYLE.with(|s| {
                if let Some(width) = s.borrow_mut().take() {
                    let _ = canvas.style().set_property("width", &width);
                }
            });
        }
    }
    state().integer_scale = enabled;
    apply_integer_scale();
}
fn apply_integer_scale() {
    if !state().integer_scale {
        return;
    }
    if let Some(canvas) = get_canvas() {
        let available = canvas
            .parent_element()
            .map_or(state().width, |p| p.client_width().max(1) as u32);
        let scale = (available / state().width).max(1);
        let _ = canvas
            .style()
            .set_property("width", &format!("{}px", state().width * scale));
    }
}
pub fn perf_monitor(enabled: bool) {
    state().perf_monitor = enabled;
    if !enabled {
        if let Some(el) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("pyxel_perf"))
        {
            el.remove();
        }
    }
}
pub fn update_monitor() {
    if !state().perf_monitor {
        return;
    }
    let Some(document) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    let element=document.get_element_by_id("pyxel_perf").or_else(||{
        let el=document.create_element("output").ok()?;el.set_id("pyxel_perf");
        let _=el.set_attribute("aria-label","性能表示");let _=el.set_attribute("aria-live","off");
        let _=el.set_attribute("style","position:fixed;right:12px;top:12px;padding:6px 10px;background:#080c14;color:#70c6a9;border:1px solid #495477;border-radius:6px;font:12px monospace;pointer-events:none;z-index:10");
        document.body()?.append_child(&el).ok()?;Some(el)
    });
    if let Some(el) = element {
        let s = state();
        el.set_text_content(Some(&format!(
            "FPS {:.1} | UPDATE {:.2}ms | DRAW {:.2}ms",
            s.actual_fps, s.update_ms, s.draw_ms
        )));
    }
}
#[wasm_bindgen]
pub async fn browser_fullscreen(enabled: bool) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|w| w.document())
        .ok_or_else(|| JsValue::from_str("No document"))?;
    let (target, name): (JsValue, &str) = if enabled {
        (
            get_canvas()
                .ok_or_else(|| JsValue::from_str("No canvas"))?
                .into(),
            "requestFullscreen",
        )
    } else {
        if document.fullscreen_element().is_none() {
            return Ok(());
        }
        (document.into(), "exitFullscreen")
    };
    let method = js_sys::Reflect::get(&target, &name.into())?.dyn_into::<js_sys::Function>()?;
    let result = method.call0(&target)?;
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(&result)).await?;
    Ok(())
}
pub fn fullscreen(enabled: bool) {
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(error) = browser_fullscreen(enabled).await {
            ERROR.with(|s| *s.borrow_mut() = Some(format!("{error:?}")));
        }
    });
}
#[wasm_bindgen]
pub fn browser_window_error() -> Option<String> {
    ERROR.with(|s| s.borrow().clone())
}
pub fn register() {
    if let Some(window) = web_sys::window() {
        let resize = Closure::<dyn FnMut(web_sys::Event)>::new(|_| apply_integer_scale());
        let _ = window.add_event_listener_with_callback("resize", resize.as_ref().unchecked_ref());
        resize.forget();
    }
}
pub fn icon<S: AsRef<str>>(data: &[S], scale: u32, transparent: Option<u8>) -> Result<(), String> {
    if data.is_empty() || scale == 0 || scale > 16 {
        return Err("Icon data/scale is invalid".into());
    }
    let rows: Vec<String> = data
        .iter()
        .map(|s| s.as_ref().chars().filter(|c| !c.is_whitespace()).collect())
        .collect();
    let w = rows[0].len();
    let h = rows.len();
    if w == 0 || w > 256 || h > 256 || rows.iter().any(|r| r.len() != w) {
        return Err("Icon dimensions are invalid".into());
    }
    let pixels = rows
        .iter()
        .map(|r| {
            r.chars()
                .map(|c| c.to_digit(16).map(|n| n as u8).ok_or("Invalid icon color"))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let document = web_sys::window()
        .and_then(|w| w.document())
        .ok_or("No document")?;
    let canvas = document
        .create_element("canvas")
        .map_err(|e| format!("{e:?}"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| "No canvas")?;
    canvas.set_width(w as u32 * scale);
    canvas.set_height(h as u32 * scale);
    let ctx = canvas
        .get_context("2d")
        .map_err(|e| format!("{e:?}"))?
        .ok_or("No context")?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|_| "No context")?;
    let colors = pyxel::colors();
    for (y, row) in pixels.iter().enumerate() {
        for (x, &color) in row.iter().enumerate() {
            if Some(color) == transparent {
                continue;
            }
            let rgb = colors
                .get(color as usize)
                .ok_or("Icon color exceeds palette")?;
            ctx.set_fill_style_str(&format!("#{rgb:06x}"));
            ctx.fill_rect(
                (x as u32 * scale) as f64,
                (y as u32 * scale) as f64,
                scale as f64,
                scale as f64,
            );
        }
    }
    let url = canvas
        .to_data_url_with_type("image/png")
        .map_err(|e| format!("{e:?}"))?;
    let link = document
        .query_selector("link[data-pyxel-icon]")
        .map_err(|e| format!("{e:?}"))?
        .unwrap_or(
            document
                .create_element("link")
                .map_err(|e| format!("{e:?}"))?,
        );
    link.set_attribute("rel", "icon")
        .map_err(|e| format!("{e:?}"))?;
    link.set_attribute("data-pyxel-icon", "")
        .map_err(|e| format!("{e:?}"))?;
    link.set_attribute("href", &url)
        .map_err(|e| format!("{e:?}"))?;
    document
        .query_selector("head")
        .map_err(|e| format!("{e:?}"))?
        .ok_or("No head")?
        .append_child(&link)
        .map_err(|e| format!("{e:?}"))?;
    Ok(())
}
#[wasm_bindgen]
pub async fn browser_screenshot(scale: u32) -> Result<Vec<u8>, JsValue> {
    if scale == 0 || scale > 16 {
        return Err(JsValue::from_str("Screenshot scale must be 1..16"));
    }
    let source = get_canvas().ok_or_else(|| JsValue::from_str("No canvas"))?;
    let document = web_sys::window().unwrap().document().unwrap();
    let output = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let (w, h) = (state().width * scale, state().height * scale);
    output.set_width(w);
    output.set_height(h);
    let ctx = output
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    ctx.set_image_smoothing_enabled(false);
    ctx.draw_image_with_html_canvas_element_and_dw_and_dh(&source, 0., 0., w as f64, h as f64)?;
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let reject_encoding = reject.clone();
        let callback = Closure::once_into_js(move |blob: Option<web_sys::Blob>| match blob {
            Some(blob) => {
                let _ = resolve.call1(&JsValue::NULL, &blob);
            }
            None => {
                let _ = reject_encoding
                    .call1(&JsValue::NULL, &JsValue::from_str("PNG encoding failed"));
            }
        });
        if let Err(error) = output.to_blob_with_type(callback.unchecked_ref(), "image/png") {
            let _ = reject.call1(&JsValue::NULL, &error);
        }
    });
    let blob = wasm_bindgen_futures::JsFuture::from(promise)
        .await?
        .dyn_into::<web_sys::Blob>()?;
    let buffer = wasm_bindgen_futures::JsFuture::from(blob.array_buffer()).await?;
    Ok(js_sys::Uint8Array::new(&buffer).to_vec())
}
