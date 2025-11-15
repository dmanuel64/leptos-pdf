use std::rc::Rc;

use async_once_cell::OnceCell as AsyncOnceCell;
use futures::channel::oneshot;
use leptos::{context::Provider, ev, prelude::*};
use leptos_meta::Script;
use pdfium_render::prelude::Pdfium;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::js_sys;

static PDFIUM_INIT_CELL: AsyncOnceCell<()> = AsyncOnceCell::new();

async fn init_pdfium() {
    // This is the one-time, lazy-executed async block.
    // We do not need to store anything, just await the signal.
    // Create a oneshot channel to signal when Pdfium is loaded
    let (tx, rx) = oneshot::channel::<()>();

    // Wrap the sender in an Rc so we can move it into the closure
    let tx = std::rc::Rc::new(std::cell::RefCell::new(Some(tx)));

    // Create the closure for the event listener
    let closure = {
        let tx = tx.clone();
        Closure::wrap(Box::new(move |_event: ev::Event| {
            if let Some(tx) = tx.borrow_mut().take() {
                let _ = tx.send(());
            }
        }) as Box<dyn FnMut(_)>)
    };

    // Add the event listener
    window()
        .add_event_listener_with_callback(
            "PdfiumRenderInitialized",
            closure.as_ref().unchecked_ref(),
        )
        .expect("failed to add event listener");

    // Keep closure alive
    closure.forget();

    // Wait for the signal
    rx.await
        .expect("failed to receive PdfiumRenderInitialized signal");
}

#[derive(Debug, Clone, Copy)]
pub struct PdfiumInjection;

impl PdfiumInjection {
    pub fn use_context() -> Option<Self> {
        use_context::<Self>()
    }

    pub async fn create_pdfium(&self) -> Rc<Pdfium> {
        PDFIUM_INIT_CELL.get_or_init(init_pdfium()).await;
        Rc::new(Pdfium::default())
    }
}

#[component]
pub fn PdfiumProvider(#[prop(into)] src: String, children: Children) -> impl IntoView {
    view! {
        <Script
            src
            on:load=move |_| {
                let _ = js_sys::eval(include_str!("../../javascript/init_pdfium.js"));
            }
        />
        <Provider value=PdfiumInjection>{children()}</Provider>
    }
}
