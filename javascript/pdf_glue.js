import * as pdfjsLib from "pdfjs-dist/build/pdf.mjs";
import pdfjsWorker from "pdfjs-dist/build/pdf.worker.mjs?url";
import { SimpleLinkService } from "pdfjs-dist/web/pdf_viewer.mjs";

pdfjsLib.GlobalWorkerOptions.workerSrc = pdfjsWorker;
export class PdfHandle {
  constructor(pdfProxy, canvasId, textLayerId, annotationLayerId) {
    this.pdfProxy = pdfProxy;
    this.canvasId = canvasId;
    this.textLayerId = textLayerId;
    this.annotationLayerId = annotationLayerId;
  }

  async renderPage(pageNumber, scale = 1.0, text = false, annotations = false) {
    const page = await this.pdfProxy.getPage(pageNumber);
    const viewport = page.getViewport({ scale });
    const canvas = document.getElementById(this.canvasId);
    const textLayer = document.getElementById(this.textLayerId);
    const annotationLayer = document.getElementById(this.annotationLayerId);
    const ctx = canvas.getContext('2d');
    canvas.width = viewport.width;
    canvas.height = viewport.height;
    await page.render({ canvasContext: ctx, viewport }).promise
    if (text && textLayer !== null) {
      page.getTextContent().then((content) => {
        textLayer.style.setProperty("--total-scale-factor", `${scale}`);
        textLayer.style.setProperty("--scale-factor", `${scale}`);
        textLayer.style.setProperty("--scale-round-x", "1px");
        textLayer.style.setProperty("--scale-round-y", "1px");
        const textRenderTask = new pdfjsLib.TextLayer({
          container: textLayer,
          textContentSource: content,
          viewport: viewport.clone({ dontFlip: true })
        });
        textRenderTask.render();
      });
      if (annotations && annotationLayer !== null) {
        // https://javascript.plainenglish.io/understanding-pdf-js-layers-and-how-to-use-them-in-react-js-6e761d796c2f
        page.getAnnotations({ intent: "display" }).then((annotations) => {
          const annotationRenderTask = new pdfjsLib.AnnotationLayer({
            div: annotationLayer,
            accessibilityManager: undefined,
            annotationCanvasMap: undefined,
            annotationEditorUIManager: undefined,
            page,
            viewport,
            structTreeLayer: null
          });
          annotationRenderTask.render({
            div: annotationLayer,
            viewport,
            page,
            annotations,
            imageResourcesPath: undefined,
            renderForms: false,
            linkService: new SimpleLinkService(),
            downloadManager: null,
            annotationStorage: undefined,
            enableScripting: false,
            hasJSActions: undefined,
            fieldObjects: undefined
          });
        });
      }
    }
  }

}

export async function loadPdf(canvas_id, url, textLayerId = null, annotationLayerId = null) {
  const resp = await fetch(url);
  const data = new Uint8Array(await resp.arrayBuffer());
  const loadingTask = pdfjsLib.getDocument({ data });
  const pdfProxy = await loadingTask.promise;
  const pdfViewer = new PdfHandle(pdfProxy, canvas_id, textLayerId, annotationLayerId);
  return pdfViewer;
}
