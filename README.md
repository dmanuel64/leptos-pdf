<!-- markdownlint-disable MD041 -->
[![crates.io version](https://img.shields.io/crates/v/leptos-pdf.svg)](https://crates.io/crates/leptos-pdf)
![Leptos Support](https://img.shields.io/badge/Leptos%20Support-v0.7%20to%20v0.8-informational)
[![docs.rs](https://img.shields.io/docsrs/leptos-pdf)](https://docs.rs/leptos-pdf)
[![crates.io downloads](https://img.shields.io/crates/d/leptos-pdf.svg)](https://crates.io/crates/leptos-pdf)
[![license](https://img.shields.io/crates/l/leptos-pdf.svg)](https://github.com/dmanuel64/leptos-pdf/blob/master/LICENSE)  
![Maintenance](https://img.shields.io/maintenance/yes/2025)


# `leptos-pdf`

`leptos-pdf` is a lightweight [Leptos](https://leptos.dev/) component library for rendering and viewing PDF files directly in your browser using [PDF.js](https://mozilla.github.io/pdf.js/).

It provides an idiomatic Leptos interface for embedding PDFs in your Rust + WebAssembly applications - complete with canvas-based renderings, text selection, and reactive props for paging and scaling.

## Installation and Supported Leptos Versions

Add `leptos-pdf` to your project using the version that matches your Leptos version:

| **Leptos Version** |        **Command**       |
|:------------------:|:------------------------:|
| 0.8                | `cargo add leptos-pdf@0.8` |
| 0.7                | `cargo add leptos-pdf@0.7` |

> Each `leptos-pdf` release tracks the same minor version of Leptos (e.g. `leptos-pdf` 0.8.x works with `leptos` 0.8.x).

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

## `PdfRenderer`

The main component, `PdfRenderer`, handles fetching, rendering, and optionally displaying a selectable text layer on top of the PDF page.

### Props

| **Prop**    | **Type** | **Reactivity**     | **Description**                                                                      |
|-------------|----------|--------------------|--------------------------------------------------------------------------------------|
| **`url`**   | `String` | Reactive or static | The source URL or path of the PDF to render.                                         |
| **`page`**  | `usize`  | Reactive or static | *(optional)* The current page number to display.                                     |
| **`scale`** | `f32`    | Reactive or static | *(optional)* Scale factor for zooming in/out of the page.                            |
| **`text`**  | `bool`   | Reactive or static | *(optional)* Enables a selectable text layer overlay using PDF.js's text extraction. |
