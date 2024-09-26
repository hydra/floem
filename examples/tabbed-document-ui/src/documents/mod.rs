use slotmap::new_key_type;
use floem::{IntoView, View};
use image::ImageDocument;
use text::TextDocument;

pub mod text;
pub mod image;

pub enum DocumentKind {
    TextDocument(TextDocument),
    ImageDocument(ImageDocument),
}

new_key_type! {
    pub struct DocumentKey;
}


pub struct DocumentContainer {}

impl DocumentContainer {
    pub fn build_view(document_kind: &DocumentKind) -> impl View {
        match document_kind {
            DocumentKind::TextDocument(text_document) => {
                text_document.build_view().into_any()
            },
            DocumentKind::ImageDocument(image_document) => {
                image_document.build_view().into_any()
            },
        }
    }
}

