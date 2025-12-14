use leptos::{prelude::*, task::spawn_local};
use leptos_meta::{Script, Style};
use wasm_bindgen::{JsCast, prelude::*};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{self, Function, Promise, Reflect};

/// Async initializer that does what your JS snippet does.
async fn init_pdfium_in_rust() -> Result<(), JsValue> {
    let window = window();

    // 1) Get global PDFiumModule function
    let pdfium_ctor_val = Reflect::get(&window, &JsValue::from_str("PDFiumModule"))?;
    let pdfium_ctor: Function = pdfium_ctor_val.dyn_into()?;

    // 2) Call PDFiumModule() -> Promise
    let promise_val = pdfium_ctor.call0(&JsValue::UNDEFINED)?;
    let promise: Promise = promise_val.dyn_into()?;

    // 3) Await the Promise to get pdfiumModule
    let pdfium_module = JsFuture::from(promise).await?;

    // 4) Get window.wasmBindings
    let wasm_bindings = Reflect::get(&window, &JsValue::from_str("wasmBindings"))?;

    // 5) Get wasmBindings.initialize_pdfium_render
    let init_fn_val = Reflect::get(
        &wasm_bindings,
        &JsValue::from_str("initialize_pdfium_render"),
    )?;
    let init_fn: Function = init_fn_val.dyn_into()?;

    // 6) Call initialize_pdfium_render(pdfiumModule, wasmBindings, false)
    let result = init_fn.call3(
        &wasm_bindings,             // this = wasmBindings
        &pdfium_module,             // arg1: pdfiumModule
        &wasm_bindings,             // arg2: wasmBindings
        &JsValue::from_bool(false), // arg3: false
    )?;

    let ok = result.as_bool().unwrap_or(false);
    if !ok {
        web_sys::console::error_1(&JsValue::from_str("Initialization of pdfium-render failed"));
        return Err(JsValue::from_str("pdfium init failed"));
    }

    Ok(())
}

#[cfg(feature = "pdfium-bundled")]
fn pdfium_blob_urls() -> Result<(String, String), wasm_bindgen::JsValue> {
    use js_sys::{Array, Uint8Array};
    use web_sys::{Blob, Url};

    // JS blob
    let js_blob = Blob::new_with_str_sequence(&wasm_bindgen::JsValue::from_str(
        leptos_pdf_pdfium_bundle::PDFIUM_JS,
    ))?;
    // let js_blob = js_blob.slice_with_i32_and_f64_and_content_type(0, js_blob.size(), "text/javascript")?;
    let js_url = Url::create_object_url_with_blob(&js_blob)?;

    // WASM blob
    // let wasm_u8 = Uint8Array::from(leptos_pdf_pdfium_bundle::PDFIUM_WASM);
    let wasm_blob =
        Blob::new_with_u8_slice_sequence(&JsValue::from(leptos_pdf_pdfium_bundle::PDFIUM_JS))?;
    // let wasm_blob = wasm_blob.slice_with_i32_and_f64_and_content_type(0, wasm_blob.size(), "application/wasm")?;
    let wasm_url = Url::create_object_url_with_blob(&wasm_blob)?;

    Ok((js_url, wasm_url))
}

#[component]
pub fn PdfiumProvider(#[prop(into)] src: String, mut children: ChildrenFnMut) -> impl IntoView {
    // Optional: track readiness in Leptos instead of DOM events
    let initialized = RwSignal::new(false);

    let on_load = move |_| {
        // Script finished loading, so PDFiumModule should now be on window.
        spawn_local({
            // let initialized = initialized;
            async move {
                match init_pdfium_in_rust().await {
                    Ok(()) => {
                        initialized.set(true);
                    }
                    Err(err) => {
                        web_sys::console::error_1(&JsValue::from_str(
                            "Pdfium initialization failed in Rust",
                        ));
                        web_sys::console::error_1(&err);
                    }
                }
            }
        });
    };

    view! {
        <Script src on:load=on_load />
        <Style>{include_str!("../../style.css")}</Style>
        {move || {
            initialized.get().then(|| children())
        }}
    }
}
