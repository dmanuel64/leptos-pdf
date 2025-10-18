use leptos::prelude::*;
use leptos_pdf::PdfViewer;

#[component]
fn App() -> impl IntoView {
    view! {
        <main style="width: 100%; height: 100vh;">
            <PdfViewer url="/public/sample.pdf"/>
        </main>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
