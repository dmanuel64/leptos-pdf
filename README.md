# `leptos-pdf`

`leptos-pdf` is a lightweight component library for rendering and viewing PDF files directly in your Leptos applications using [PDF.js](https://mozilla.github.io/pdf.js/).

You can add `leptos-pdf` to your Leptos project with:

```shell
cargo add leptos-pdf
```

## Example

```rust
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
```

Will render the `/public/sample.pdf` PDF file in your browser.
