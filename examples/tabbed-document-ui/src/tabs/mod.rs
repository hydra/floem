use std::ops::Deref;
use crate::tabs::document::DocumentTab;
use crate::tabs::home::HomeTab;
use crate::ui::tab_bar::TabKeyFactory;

pub mod home;
pub mod document;

#[derive(Clone)]
pub enum TabKind {
    Home(HomeTab),
    Document(DocumentTab),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TabKey(usize);

impl TabKeyFactory<Self> for TabKey {
    fn new(index: usize) -> Self {
        Self(index)
    }
}

impl Deref for TabKey {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
