use leptos::{prelude::*, task::spawn_local};
use leptos_meta::{Script, Style};
use wasm_bindgen::{JsCast, prelude::*};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys;

/// Async initializer that does what your JS snippet does.
async fn init_pdfium_in_rust() -> Result<(), JsValue> {
    let window = window();

    // 1) Get global PDFiumModule function
    let pdfium_ctor_val = js_sys::Reflect::get(&window, &JsValue::from_str("PDFiumModule"))?;
    let pdfium_ctor: js_sys::Function = pdfium_ctor_val.dyn_into()?;

    // 2) Call PDFiumModule() -> Promise
    let promise_val = pdfium_ctor.call0(&JsValue::UNDEFINED)?;
    let promise: js_sys::Promise = promise_val.dyn_into()?;

    // 3) Await the Promise to get pdfiumModule
    let pdfium_module = JsFuture::from(promise).await?;

    // 4) Get window.wasmBindings
    let wasm_bindings = js_sys::Reflect::get(&window, &JsValue::from_str("wasmBindings"))?;

    // 5) Get wasmBindings.initialize_pdfium_render
    let init_fn_val = js_sys::Reflect::get(
        &wasm_bindings,
        &JsValue::from_str("initialize_pdfium_render"),
    )?;
    let init_fn: js_sys::Function = init_fn_val.dyn_into()?;

    // 6) Call initialize_pdfium_render(pdfiumModule, wasmBindings, false)
    let result = init_fn.call3(
        &wasm_bindings,             // this = wasmBindings
        &pdfium_module,             // arg1: pdfiumModule
        &wasm_bindings,             // arg2: wasmBindings
        &JsValue::from_bool(false), // arg3: false
    )?;

    let ok = result.as_bool().unwrap_or(false);
    if !ok {
        return Err(JsValue::from_str(
            "initialize_pdfium_render did not return ok",
        ));
    }

    Ok(())
}

#[component]
fn PdfiumProviderCore(#[prop(into)] src: String, children: ChildrenFn) -> impl IntoView {
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
                        log::error!("Pdfium initialization failed in Rust: {:?}", err);
                    }
                }
            }
        });
    };

    view! {
        <Script src on:load=on_load />
        <Style>{include_str!("../../style.css")}</Style>
        <Show when=move || initialized.get()>
            {children()}
        </Show>
    }
}

#[cfg(not(feature = "pdfium-bundled"))]
#[component]
pub fn PdfiumProvider(#[prop(into)] src: String, children: ChildrenFn) -> impl IntoView {
    view! {
        <PdfiumProviderCore src>
            {children()}
        </PdfiumProviderCore>
    }
}

#[cfg(feature = "pdfium-bundled")]
#[component]
pub fn PdfiumProvider(children: ChildrenFn) -> impl IntoView {
    use web_sys::{Blob, Url};

    // WASM blob
    let wasm_u8 = js_sys::Uint8Array::from(leptos_pdf_pdfium_bundle::PDFIUM_WASM);
    let wasm_blob = Blob::new_with_u8_array_sequence(&js_sys::Array::of1(&wasm_u8.buffer()))
        .expect("The Pdfium WASM blob should be created successfully");
    let wasm_url = Url::create_object_url_with_blob(&wasm_blob)
        .expect("The URL to the Pdfium WASM blob should be created successfully");

    // JS blob
    let js_blob =
        Blob::new_with_str_sequence(&js_sys::Array::of1(&wasm_bindgen::JsValue::from_str(
            &leptos_pdf_pdfium_bundle::PDFIUM_JS.replacen("pdfium.wasm", &wasm_url, 1),
        )))
        .expect("The Pdfium JavaScript blob should be created successfully");
    let js_url = Url::create_object_url_with_blob(&js_blob)
        .expect("The URL to the Pdfium JavaScript blob should be created successfully");

    view! {
        <PdfiumProviderCore src=js_url>
            {children()}
        </PdfiumProviderCore>
    }
}
