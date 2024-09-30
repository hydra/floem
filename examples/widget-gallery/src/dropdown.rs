use dropdown::Dropdown;
use strum::IntoEnumIterator;

use floem::{prelude::*, reactive::create_effect};

use crate::form::{self, form_item};

#[derive(strum::EnumIter, Debug, PartialEq, Clone, Copy)]
enum Value {
    One,
    Two,
    Three,
    Four,
    Five,
}
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

const CHEVRON_DOWN: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" xml:space="preserve" viewBox="0 0 185.344 185.344">
  <path fill="#010002" d="M92.672 144.373a10.707 10.707 0 0 1-7.593-3.138L3.145 59.301c-4.194-4.199-4.194-10.992 0-15.18a10.72 10.72 0 0 1 15.18 0l74.347 74.341 74.347-74.341a10.72 10.72 0 0 1 15.18 0c4.194 4.194 4.194 10.981 0 15.18l-81.939 81.934a10.694 10.694 0 0 1-7.588 3.138z"/>
</svg>"##;

pub fn dropdown_view() -> impl IntoView {
    let dropdown_active_item = RwSignal::new(Value::Three);

    create_effect(move |_| {
        let active_item = dropdown_active_item.get();
        println!("Selected: {active_item}");
    });

    form::form({
        (form_item("Dropdown".to_string(), 120.0, move || {
            Dropdown::new_rw(dropdown_active_item, Value::iter())
        }),)
    })
}
