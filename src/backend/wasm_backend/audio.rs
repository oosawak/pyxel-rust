//! Web Audio output for the official, stateful Pyxel synthesizer.
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{AudioContext, AudioContextOptions, AudioProcessingEvent, ScriptProcessorNode};

struct Output {
    context: AudioContext,
    _node: ScriptProcessorNode,
    blocks: Rc<RefCell<u64>>,
    peak: Rc<RefCell<f32>>,
}
thread_local! {static OUTPUT:RefCell<Option<Output>>=const{RefCell::new(None)};static ERROR:RefCell<Option<String>>=const{RefCell::new(None)};}
fn ensure() -> Result<AudioContext, JsValue> {
    OUTPUT.with(|slot| {
        if let Some(output)=slot.borrow().as_ref() {return Ok(output.context.clone());}
        let options=AudioContextOptions::new();options.set_sample_rate(pyxel::AUDIO_SAMPLE_RATE as f32);
        let context=AudioContext::new_with_context_options(&options)?;
        if context.sample_rate()!=pyxel::AUDIO_SAMPLE_RATE as f32 {return Err(JsValue::from_str("The browser did not accept the Pyxel sample rate"));}
        let node=context.create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels(1024,0,1)?;
        let blocks=Rc::new(RefCell::new(0));let peak=Rc::new(RefCell::new(0f32));
        let count=blocks.clone();let level=peak.clone();
        let mut renderer=pyxel::AudioRenderer::new();let mut samples=vec![0i16;1024];let mut floats=vec![0f32;1024];
        let callback=Closure::<dyn FnMut(AudioProcessingEvent)>::new(move |event:AudioProcessingEvent| {
            renderer.render(&mut samples);
            for (dst,src) in floats.iter_mut().zip(&samples) {*dst=*src as f32/32768.;}
            if let Ok(buffer)=event.output_buffer() {let _=buffer.copy_to_channel(&floats,0);}
            *count.borrow_mut()+=1;
            let measured=floats.iter().map(|v| v.abs()).fold(0f32,f32::max);
            let previous=*level.borrow();
            *level.borrow_mut()=previous.max(measured);
        });
        node.set_onaudioprocess(Some(callback.as_ref().unchecked_ref()));
        node.connect_with_audio_node(&context.destination())?;
        callback.forget();
        *slot.borrow_mut()=Some(Output {context:context.clone(),_node:node,blocks,peak});
        Ok(context)
    })
}
/// Call from a user gesture to satisfy browser audio autoplay policy.
#[wasm_bindgen]
pub async fn resume_browser_audio() -> Result<(), JsValue> {
    let context = ensure()?;
    wasm_bindgen_futures::JsFuture::from(context.resume()?).await?;
    Ok(())
}
pub fn request_resume() {
    #[cfg(target_arch = "wasm32")]
    match ensure().and_then(|c| c.resume().map(|_| ())) {
        Ok(()) => {}
        Err(error) => ERROR.with(|slot| *slot.borrow_mut() = Some(format!("{error:?}"))),
    }
}
#[wasm_bindgen]
pub fn browser_audio_status() -> String {
    OUTPUT.with(|slot| {
        let slot=slot.borrow();
        let (state,blocks,peak)=slot.as_ref().map_or(("not-started".to_string(),0,0.),|o|(format!("{:?}",o.context.state()),*o.blocks.borrow(),*o.peak.borrow()));
        serde_json::json!({"state":state,"blocks":blocks,"peak":peak,"error":ERROR.with(|s|s.borrow().clone())}).to_string()
    })
}
