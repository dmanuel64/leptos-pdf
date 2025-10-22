# `leptos-pdf`

`leptos-pdf` is a lightweight Leptos component library for rendering and viewing PDF files directly in your browser using [PDF.js](https://mozilla.github.io/pdf.js/).

It provides an idiomatic Leptos interface for embedding PDFs in your Rust + WebAssembly applications - complete with canvas-based renderings, text selection, and reactive props for paging and scaling.

---

## Installation

Add `leptos-pdf` to your Leptos project:

```shell
cargo add leptos-pdf
```

---

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

---

## `PdfRenderer`

The main component, `PdfRenderer`, handles fetching, rendering, and optionally displaying a selectable text layer on top of the PDF page.

### Props

| Prop        | Type                 | Description                                                                          |
| ----------- | -------------------- | ------------------------------------------------------------------------------------ |
| **`url`**   | `String`             | The source URL or path of the PDF to render.                                         |
| **`page`**  | `MaybeSignal<usize>` | *(optional)* The current page number to display.                |
| **`scale`** | `MaybeSignal<f32>`   | *(optional)* Scale factor for zooming in/out of the page.                            |
| **`text`**  | `MaybeSignal<bool>`  | *(optional)* Enables a selectable text layer overlay using PDF.js's text extraction. |
