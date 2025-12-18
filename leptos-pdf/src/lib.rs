//! # [`leptos-pdf`](crate)
//!
//! [`leptos-pdf`](crate) is a lightweight Leptos component library for rendering and viewing PDF files
//! in a Leptos app using [PDFium](https://pdfium.googlesource.com/pdfium/) via the
//! [`pdfium-render`](https://crates.io/crates/pdfium-render) crate.
//!
//! It provides an idiomatic Leptos interface for embedding PDFs in Rust + WebAssembly apps, with
//! canvas-based rendering.
//!
//! ## Installation and Supported Leptos Versions
//! 
//! Add [`leptos-pdf`](crate) to your project using the version that matches your Leptos version:
//! 
//! | **Leptos Version** |        **Command**         |
//! |:------------------:|:--------------------------:|
//! | 0.8                | `cargo add leptos-pdf@0.8` |
//! | 0.7                | `cargo add leptos-pdf@0.7` |
//! 
//! Each [`leptos-pdf`](crate) release tracks the same minor version of Leptos (e.g. [`leptos-pdf`](crate) 0.8.x works with [`leptos`] 0.8.x).
//! 
//! ## Example
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_pdf::prelude::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <main>
//!             <div style:width="100vw" style:height="100vh">
//!                 <PdfiumProvider>
//!                     // If the pdfium-bundled feature is not enabled, you must provide your own URL:
//!                     //
//!                     // <PdfiumProvider src="/public/pdfium/pdfium.js">
//!                     //
//!                     // With this setup, make sure that /public/pdfium/pdfium.wasm also exists, and
//!                     // you modify your index.html to include these files
//!                     <PdfViewer
//!                         url="/public/sample.pdf"
//!                         loading_fallback=move || view! { <p>"Loading..."</p> }
//!                         error_fallback=move |e| {
//!                             view! { <p>"An error occurred..."</p> }
//!                         }
//!                     />
//!                 </PdfiumProvider>
//!             </div>
//!         </main>
//!     }
//! }
//!
//! fn main() {
//!     mount_to_body(App)
//! }
//!
//! ```
//!
//! See the [`components`] module for more information on PDF rendering.
//!
//! ## Getting Started
//!
//! PDFium is a vendor dependency (JavaScript + WASM). You can provide it in one of two ways:
//!
//! 1. **Manual assets (recommended if you want full control).**
//!
//!    Download the PDFium WASM distribution from the
//!    [pdfium-binaries repository](https://github.com/bblanchon/pdfium-binaries) (grab the latest
//!    WASM release), and place the files in your app's assets directory (for example, `public/`).
//!
//!    If you use Trunk, ensure the directory is copied into the build output:
//!
//!    ```html
//!    <link data-trunk rel="copy-dir" href="public/" />
//!    ```
//!
//!    Then initialize PDFium with [`PdfiumProvider`](crate::components::PdfiumProvider) and point it
//!    at your `pdfium.js`:
//!
//!    ```rust
//!    use leptos::prelude::*;
//!    use leptos_pdf::prelude::*;
//!
//!    #[component]
//!    fn Demo() -> impl IntoView {
//!         view! {
//!             <PdfiumProvider src="/public/pdfium/pdfium.js">
//!                 // ...
//!             </PdfiumProvider>
//!         }
//!    }
//!    ```
//!
//!    Make sure the referenced `pdfium.js` can locate its accompanying `pdfium.wasm` (typically by
//!    keeping them in the same directory).
//! 
//!    For more information on how to bundle PDFium, see
//!    [`pdfium-render`'s guide on compiling to WASM](https://github.com/ajrcarey/pdfium-render/tree/master?tab=readme-ov-file#compiling-to-wasm).
//!
//! 2. **Bundled assets (zero setup).**
//!
//!    Enable the `pdfium-bundled` feature when adding [`leptos-pdf`](crate). This embeds PDFium's
//!    JavaScript and WASM bytes into your binary and serves them via `blob:` URLs at runtime.
//!    This is the easiest option, but the embedded PDFium version are not updated on a fixed schedule.
//!
//! ## Features
//!
//! - `pdfium-bundled`: Embed PDFium's JavaScript and WASM and serve them via `blob:` URLs at runtime.

pub mod components;
pub mod errors;
pub mod prelude;
