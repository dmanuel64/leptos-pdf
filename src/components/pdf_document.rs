use leptos::prelude::*;
use pdfium_render::prelude::Pdfium;
use wasm_bindgen_futures::js_sys;
use web_sys::{Request, RequestInit, RequestMode};

use crate::components::pdfium::PdfiumInjection;

#[component]
pub fn PdfDocument(
    #[prop(into)] url: Signal<String>,
    #[prop(optional, into)] password: MaybeProp<String>,
    #[prop(optional, into)] fallback: ViewFnOnce,
) -> impl IntoView {
    let pdfium = LocalResource::new(move || {
        let injection = PdfiumInjection::use_context().expect("PdfiumProvider is missing");
        async move { injection.create_pdfium().await }
    });
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    view! {
        <Transition fallback>
            {move || { pdfium.with(|maybe_pdfium| {
                if let Some(p) = maybe_pdfium {
                    // let request = Request::new_with_str_and_init(&url, &opts)?;
                    log::warn!("{:?}", p.load_pdf_from_byte_slice(&[1], password.get().as_deref()));
                    true
                } else {
                    false
                }
            }).then(|| view! { <p>"Loaded PDF"</p> }) }}
        </Transition>
    }
}
