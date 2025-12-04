use leptos::{prelude::*, task::spawn_local};
use leptos_meta::{Script, Style};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{self, Function, Promise, Reflect};

/// Async initializer that does what your JS snippet does.
pub async fn init_pdfium_in_rust() -> Result<(), JsValue> {
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
