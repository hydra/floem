use crate::signal::{create_rw_signal, RwSignal};

pub struct Trigger {
    signal: RwSignal<()>,
}

impl Copy for Trigger {}

impl Clone for Trigger {
    fn clone(&self) -> Self {
        Self {
            signal: self.signal,
        }
    }
}

impl Trigger {
    pub fn notify(&self) {
        self.signal.set(());
    }

    pub fn track(&self) {
        self.signal.with(|_| {});
    }
}

pub fn create_trigger() -> Trigger {
    Trigger {
        signal: create_rw_signal(()),
    }
}