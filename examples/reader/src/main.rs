use leptos::prelude::*;
use leptos_pdf::components::{PdfDocument, PdfiumProvider};

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
