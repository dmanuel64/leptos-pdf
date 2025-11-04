// Wait for Trunk's injected script to load your module
window.addEventListener("TrunkApplicationStarted", (event) => {
    // Initialize Pdfium + Leptos binding
    PDFiumModule().then(async pdfiumModule => {
        if (window.wasmBindings.initialize_pdfium_render(pdfiumModule, event, false)) {
            window.dispatchEvent(new Event("PdfiumRenderInitialized"));
        } else {
            console.error("Initialization of pdfium-render failed");
        }

    });
});