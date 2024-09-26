use slotmap::new_key_type;
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
