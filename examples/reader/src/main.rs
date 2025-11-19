use leptos::prelude::*;
use leptos_pdf::components::{PdfDocument, PdfiumProvider, TextLayerConfig};

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <div style:width="100vw" style:height="100vh">
                <PdfiumProvider src="/public/pdfium/pdfium.js">
                    <PdfDocument
                        url="/public/sample.pdf"
                        loading_fallback=move || view! { <p>"Loading..."</p> }
                        error_fallback=move |_| view! { <p>"An error occurred..."</p> }
                        text_layer_config=TextLayerConfig::default()
                    />
                </PdfiumProvider>
            </div>
        </main>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
