//! PDFium initialization for [`leptos-pdf`](crate).
//!
//! This module provides the [`PdfiumProvider`] component and the internal plumbing required to
//! bootstrap the Emscripten-wrapped PDFium WASM runtime before any PDF rendering components are
//! used.

use leptos::{prelude::*, task::spawn_local};
use leptos_meta::{Script, Style};
use wasm_bindgen::{JsCast, prelude::*};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys;

use crate::errors::{PdfError, js_error_string_or};

/// Bootstraps PDFium's Emscripten-wrapped WASM module.
///
/// This performs the same initialization as the first `<script>` in the
/// `pdfium-render` WASM example `index.html`, but does it from Rust so consumers do
/// **not** need to modify their own `index.html`.
///
/// Reference:
/// <https://github.com/ajrcarey/pdfium-render/blob/master/examples/index.html>
async fn init_pdfium() -> Result<(), PdfError> {
    let window = window();

    // Get global PDFiumModule function
    let pdfium_module =
        js_sys::Reflect::get(&window, &JsValue::from_str("PDFiumModule")).map_err(|e| {
            PdfError::PdfiumInitError(js_error_string_or(
                e,
                "Could not locate PDFiumModule in JavaScript".to_string(),
            ))
        })?;
    let pdfium_module: js_sys::Function = pdfium_module.dyn_into().map_err(|e| {
        PdfError::PdfiumInitError(js_error_string_or(
            e,
            "Located PDFiumModule in JavaScript, but it is not of type Function".to_string(),
        ))
    })?;

    // Call PDFiumModule() -> Promise
    let promise_val = pdfium_module.call0(&JsValue::UNDEFINED).map_err(|e| {
        PdfError::PdfiumInitError(js_error_string_or(
            e,
            "PDFiumModule raised an exception".to_string(),
        ))
    })?;
    let promise: js_sys::Promise = promise_val.dyn_into().map_err(|e| {
        PdfError::PdfiumInitError(js_error_string_or(
            e,
            "Called the PDFiumModule function, but it's return value is not of type Promise"
                .to_string(),
        ))
    })?;

    // Await the Promise to get pdfiumModule
    let pdfium_module = JsFuture::from(promise)
        .await
        .map_err(|e| PdfError::PdfiumInitError(e.as_string().unwrap_or_default()))?;

    // Get window.wasmBindings
    let wasm_bindings = js_sys::Reflect::get(&window, &JsValue::from_str("wasmBindings"))
        .expect("window.wasmBindings should be present in a Leptos app");

    // Get wasmBindings.initialize_pdfium_render
    let init_fn_val = js_sys::Reflect::get(
        &wasm_bindings,
        &JsValue::from_str("initialize_pdfium_render"),
    )
    .map_err(|e| {
        PdfError::PdfiumInitError(js_error_string_or(
            e,
            "Could not locate initialize_pdfium_render in JavaScript".to_string(),
        ))
    })?;
    let init_fn: js_sys::Function = init_fn_val.dyn_into().map_err(|e| {
        PdfError::PdfiumInitError(js_error_string_or(
            e,
            "Located initialize_pdfium_render in JavaScript, but it is not of type Function"
                .to_string(),
        ))
    })?;

    // Call initialize_pdfium_render(pdfiumModule, wasmBindings, false)
    let _result = init_fn
        .call3(
            &wasm_bindings,             // this = wasmBindings
            &pdfium_module,             // arg1: pdfiumModule
            &wasm_bindings,             // arg2: wasmBindings
            &JsValue::from_bool(false), // arg3: false
        )
        .map_err(|e| {
            PdfError::PdfiumInitError(js_error_string_or(
                e,
                "initialize_pdfium_render raised an exception".to_string(),
            ))
        })?;

    Ok(())
}

/// Internal shared implementation for [`PdfiumProvider`] variants.
#[component]
fn PdfiumProviderCore(#[prop(into)] src: String, children: ChildrenFn) -> impl IntoView {
    // Optional: track readiness in Leptos instead of DOM events
    let initialized = RwSignal::new(false);

    let on_load = move |_| {
        // Script finished loading, so PDFiumModule should now be on window.
        spawn_local({
            // let initialized = initialized;
            async move {
                match init_pdfium().await {
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

/// Initializes PDFium using a URL to `pdfium.js`.
///
/// Call this once near the root of your Leptos app. This provider **must** be mounted
/// before using [`PdfViewer`](crate::components::pdf_viewer::PdfViewer) or
/// [`PdfDocument`](crate::components::pdf_document::PdfDocument).
#[cfg(not(feature = "pdfium-bundled"))]
#[component]
pub fn PdfiumProvider(
    /// URL to the `pdfium.js` file to load (Emscripten wrapper for the PDFium WASM runtime).
    #[prop(into)]
    src: String,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <PdfiumProviderCore src>
            {children()}
        </PdfiumProviderCore>
    }
}

/// Initializes PDFium using the bundled `pdfium.js` and `pdfium.wasm` assets.
///
/// Call this once near the root of your Leptos app. This provider **must** be mounted
/// before using [`PdfViewer`](crate::components::pdf_viewer::PdfViewer) or
/// [`PdfDocument`](crate::components::pdf_document::PdfDocument).
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
