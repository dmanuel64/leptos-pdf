use leptos::prelude::*;
use leptos_pdf::PdfRenderer;

#[component]
fn App() -> impl IntoView {
    view! {
        <div style:width="100vw" style:height="100vh">
            <PdfRenderer url="/public/sample.pdf"/>
        </div>
    }
}

fn main() {
    mount_to_body(App)
}