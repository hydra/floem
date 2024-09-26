use floem::View;
use floem::views::{dyn_view, label, v_stack};
use super::TabKey;

#[derive(Clone)]
pub struct HomeTab {
}

pub struct HomeContainer {}

impl HomeContainer {
    pub fn build_view(tab_key: TabKey) -> impl View {
        v_stack((
            label(|| "Home Tab Content"),
            dyn_view(move || format!("tab_id: {:?}", &tab_key))
        ))
    }
}
