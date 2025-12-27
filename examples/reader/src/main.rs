use leptos::prelude::*;
use leptos_pdf::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <div style:width="100vw" style:height="100vh">
                <PdfiumProvider>
                    // If the pdfium-bundled feature is not enabled, you must provide your own URL:
                    //
                    // <PdfiumProvider src="/public/pdfium/pdfium.js">
                    //
                    // With this setup, make sure that /public/pdfium/pdfium.wasm also exists, and
                    // you modify your index.html to include these files
                    <PdfViewer
                        url="/public/sample.pdf"
                        loading_fallback=move || view! { <p>"Loading..."</p> }
                        error_fallback=move |e| {
                            log::error!("Error loading PDF document: {:?}", e.get());
                            view! { <p>"An error occurred..."</p> }
                        }
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
