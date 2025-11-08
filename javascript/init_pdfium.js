// Initialize Pdfium + Leptos binding
PDFiumModule().then(async pdfiumModule => {
    if (window.wasmBindings.initialize_pdfium_render(pdfiumModule, window.wasmBindings, false)) {
        window.dispatchEvent(new Event("PdfiumRenderInitialized"));
    } else {
        console.error("Initialization of pdfium-render failed");
    }

});