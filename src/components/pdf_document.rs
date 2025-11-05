use leptos::prelude::*;
use pdfium_render::prelude::Pdfium;

use crate::components::pdfium::PdfiumInjection;

#[component]
pub fn PdfDocument(#[prop(optional, into)] fallback: ViewFnOnce) -> impl IntoView {
    let pdfium = LocalResource::new(move || {
        let injection = PdfiumInjection::use_context().expect("PdfiumProvider is missing");
        async move { injection.create_pdfium().await }
    });
    view! {
        <Transition fallback>
            {move || { pdfium.with(|maybe_pdfium| {
                if let Some(p) = maybe_pdfium {
                    log::warn!("{:?}", p.create_new_pdf());
                    true
                } else {
                    false
                }
            }).then(|| view! { <p>"Loaded PDF"</p> }) }}
        </Transition>
    }
}
