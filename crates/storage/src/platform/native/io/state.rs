use astra_canvas::canvas::CanvasDocument;
use astra_canvas::shapes::Shape;
use std::sync::Mutex;

static PENDING_DOCUMENT: Mutex<Option<CanvasDocument>> = Mutex::new(None);
static PENDING_IMAGE: Mutex<Option<Shape>> = Mutex::new(None);

pub(super) fn set_pending_document(document: CanvasDocument) {
    if let Ok(mut guard) = PENDING_DOCUMENT.lock() {
        *guard = Some(document);
    }
}

pub(super) fn set_pending_image(shape: Shape) {
    if let Ok(mut guard) = PENDING_IMAGE.lock() {
        *guard = Some(shape);
    }
}

pub fn take_pending_document() -> Option<CanvasDocument> {
    PENDING_DOCUMENT
        .lock()
        .ok()
        .and_then(|mut pending| pending.take())
}

pub fn take_pending_insert_image() -> Option<Shape> {
    PENDING_IMAGE
        .lock()
        .ok()
        .and_then(|mut pending| pending.take())
}
