use astra_canvas::canvas::CanvasDocument;
use astra_canvas::shapes::Shape;
use crate::preferences::UserPreferences;
use std::cell::RefCell;

thread_local! {
    static PENDING_DOCUMENT: RefCell<Option<CanvasDocument>> = const { RefCell::new(None) };
    static PENDING_DOCUMENT_LIST: RefCell<Option<Vec<String>>> = const { RefCell::new(None) };
    static PENDING_IMAGE: RefCell<Option<Shape>> = const { RefCell::new(None) };
    static PENDING_SHAPES: RefCell<Option<(Vec<Shape>, kurbo::Point)>> = const { RefCell::new(None) };
    static PENDING_CLIPBOARD_TEXT: RefCell<Option<String>> = const { RefCell::new(None) };
    static PENDING_MATH_CLIPBOARD: RefCell<Option<String>> = const { RefCell::new(None) };
    static PENDING_DROPPED_DOCUMENT: RefCell<Option<String>> = const { RefCell::new(None) };
    static PENDING_DROPPED_IMAGES: RefCell<Vec<Shape>> = const { RefCell::new(Vec::new()) };
    static PENDING_INSERT_IMAGE: RefCell<Option<Shape>> = const { RefCell::new(None) };
    static PENDING_PREFERENCES: RefCell<Option<UserPreferences>> = const { RefCell::new(None) };
}

pub fn take_pending_document() -> Option<CanvasDocument> {
    PENDING_DOCUMENT.with(|c| c.borrow_mut().take())
}

pub fn set_pending_document(doc: CanvasDocument) {
    PENDING_DOCUMENT.with(|c| *c.borrow_mut() = Some(doc));
}

pub fn take_pending_document_list() -> Option<Vec<String>> {
    PENDING_DOCUMENT_LIST.with(|c| c.borrow_mut().take())
}

pub fn set_pending_document_list(list: Vec<String>) {
    PENDING_DOCUMENT_LIST.with(|c| *c.borrow_mut() = Some(list));
}

pub fn take_pending_image() -> Option<Shape> {
    PENDING_IMAGE.with(|c| c.borrow_mut().take())
}

pub fn set_pending_image(shape: Shape) {
    PENDING_IMAGE.with(|c| *c.borrow_mut() = Some(shape));
}

pub fn take_pending_shapes() -> Option<(Vec<Shape>, kurbo::Point)> {
    PENDING_SHAPES.with(|c| c.borrow_mut().take())
}

pub fn set_pending_shapes(shapes: Vec<Shape>, cursor: kurbo::Point) {
    PENDING_SHAPES.with(|c| *c.borrow_mut() = Some((shapes, cursor)));
}

pub fn take_pending_clipboard_text() -> Option<String> {
    PENDING_CLIPBOARD_TEXT.with(|c| c.borrow_mut().take())
}

pub fn set_pending_clipboard_text(text: String) {
    PENDING_CLIPBOARD_TEXT.with(|c| *c.borrow_mut() = Some(text));
}

pub fn take_pending_math_clipboard() -> Option<String> {
    PENDING_MATH_CLIPBOARD.with(|c| c.borrow_mut().take())
}

pub fn set_pending_math_clipboard(text: String) {
    PENDING_MATH_CLIPBOARD.with(|c| *c.borrow_mut() = Some(text));
}

pub fn take_pending_dropped_document() -> Option<String> {
    PENDING_DROPPED_DOCUMENT.with(|c| c.borrow_mut().take())
}

pub fn set_pending_dropped_document(json: String) {
    PENDING_DROPPED_DOCUMENT.with(|c| *c.borrow_mut() = Some(json));
}

pub fn take_pending_dropped_images() -> Vec<Shape> {
    PENDING_DROPPED_IMAGES.with(|c| std::mem::take(&mut *c.borrow_mut()))
}

pub fn push_pending_dropped_image(shape: Shape) {
    PENDING_DROPPED_IMAGES.with(|c| c.borrow_mut().push(shape));
}

pub fn take_pending_insert_image() -> Option<Shape> {
    PENDING_INSERT_IMAGE.with(|c| c.borrow_mut().take())
}

pub fn set_pending_insert_image(shape: Shape) {
    PENDING_INSERT_IMAGE.with(|c| *c.borrow_mut() = Some(shape));
}

pub fn take_pending_preferences() -> Option<UserPreferences> {
    PENDING_PREFERENCES.with(|c| c.borrow_mut().take())
}

pub fn set_pending_preferences(prefs: UserPreferences) {
    PENDING_PREFERENCES.with(|c| *c.borrow_mut() = Some(prefs));
}
