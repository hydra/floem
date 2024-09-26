use slotmap::new_key_type;
use crate::tabs::document::DocumentTab;
use crate::tabs::home::HomeTab;

pub mod home;
pub mod document;

#[derive(Clone)]
pub enum TabKind {
    Home(HomeTab),
    Document(DocumentTab),
}

new_key_type! {
    pub struct TabKey;
}
