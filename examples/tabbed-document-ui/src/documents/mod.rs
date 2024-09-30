use std::fmt::{Debug, Formatter};
use slotmap::new_key_type;
use floem::{IntoView, View};
use image::ImageDocument;
use text::TextDocument;
use crate::documents::new_document_form::NewDocumentForm;

pub mod text;
pub mod image;
pub mod new_document_form;

pub enum DocumentKind {
    TextDocument(TextDocument),
    ImageDocument(ImageDocument),
    NewDocumentForm(NewDocumentForm),
}

impl Debug for DocumentKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TextDocument(_) => f.write_str("TextDocument"),
            Self::ImageDocument(_) => f.write_str("ImageDocument"),
            Self::NewDocumentForm(_) => f.write_str("NewDocumentForm"),
        }
    }
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
            DocumentKind::NewDocumentForm(new_document_form) => {
                new_document_form.build_view().into_any()
            }
        }
    }
}

