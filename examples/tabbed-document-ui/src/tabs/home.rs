use floem::View;
use floem::views::{label, TupleStackExt};
use super::TabKey;

#[derive(Clone)]
pub struct HomeTab {
}

pub struct HomeContainer {}

impl HomeContainer {
    pub fn build_view(tab_key: TabKey) -> impl View {
        (
            "Home Tab Content",
            label(move || format!("tab_id: {:?}", &tab_key))
        )
            .v_stack()
    }
}
