use leptos::prelude::*;
use leptos_pdf::PdfRenderer;

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <div style:width="100vw" style:height="100vh">
            <PdfRenderer url="/public/sample.pdf"/>
            </div>
        </main>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
