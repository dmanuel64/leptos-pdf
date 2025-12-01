use crate::components::{
    pdf_document::{DocumentViewerLayout, TextLayerConfig},
    PdfDocument,
};
use leptos::prelude::*;
use web_sys::RequestMode;

#[component]
pub fn PdfViewer<FalFn, Fal>(
    /// URL to the PDF file
    #[prop(into)]
    url: Signal<String>,
    /// Password to access the PDF if required
    #[prop(optional, into)]
    password: MaybeProp<String>,
    /// View to display while the PDF is loading
    #[prop(optional, into)]
    loading_fallback: ViewFnOnce,
    /// View to display if an error is encountered and the PDF cannot be loaded
    error_fallback: FalFn,
    /// Options for fetching the PDF file
    #[prop(default=RequestMode::SameOrigin)]
    mode: RequestMode,
    /// Configuration options for the selectable text layer. Not providing a value will not render a text layer
    #[prop(optional, into)]
    text_layer_config: MaybeProp<TextLayerConfig>,
    /// Optional signal to capture all text in the document
    // #[prop(optional, into)]
    // set_captured_document_text: Option<WriteSignal<Vec<String>>>,
    /// Layout options for the document viewer
    #[prop(optional, into)]
    viewer_layout: Signal<DocumentViewerLayout>,
    /// Whether to show a scrollbar when the content overflows
    #[prop(default=true.into(), into)]
    scrollbar: Signal<bool>,
) -> impl IntoView
where
    FalFn: FnMut(ArcRwSignal<Errors>) -> Fal + Send + 'static,
    Fal: IntoView + Send + 'static,
{
    view! {
        <PdfDocument
            url=url
            password=password
            loading_fallback=loading_fallback
            error_fallback=error_fallback
            mode=mode
            text_layer_config=text_layer_config
            // set_captured_document_text=set_captured_document_text
            viewer_layout=viewer_layout
            scrollbar=scrollbar
        />
    }
}
