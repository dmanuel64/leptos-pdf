use leptos::prelude::*;
use leptos_pdf::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <div style:width="100vw" style:height="100vh">
                <PdfiumProvider src="/public/pdfium/pdfium.js">
                    <PdfViewer
                        url="/public/sample.pdf"
                        loading_fallback=move || view! { <p>"Loading..."</p> }
                        error_fallback=move |e| {
                            log::error!("Error loading PDF document: {:?}", e.get());
                            view! { <p>"An error occurred..."</p> }
                        }
                        enable_text_layer=true
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
