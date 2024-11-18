use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;
use floem::{IntoView, style_class, View, ViewId};
use floem::peniko::Color;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::AlignItems;
use floem::view_tuple::ViewTuple;
use floem::views::{button, ButtonClass, Decorators, dyn_stack, label};

pub struct TabBar<T, K>
where
    K: Eq + Hash + 'static
{
    id: ViewId,

    active_tab: RwSignal<Option<K>>,

    phantom_data: PhantomData<T>
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

pub trait TabKeyFactory<K> {
    fn new(index: usize) -> K;
}

#[derive(Clone)]
pub struct TabItem<T: Clone> {
    pub kind: T,
    pub name: RwSignal<String>
}

style_class!(pub TabCloseButtonClass);
style_class!(pub TabItemClass);
style_class!(pub TabBarClass);

#[derive(Debug)]
pub enum TabBarEvent<K> {
    TabClosed { key: K }
}

pub type TabBarEventSignal<K> = RwSignal<Option<TabBarEvent<K>>>;

pub fn tab_bar<IF, I, T, K>(event_signal: TabBarEventSignal<K>, active_tab: RwSignal<Option<K>>, each_fn: IF) -> TabBar<T, K>
where
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = (usize, TabItem<T>)>,
    K: Clone + Eq + Hash + TabKeyFactory<K> + 'static,
    T: Clone + 'static,
{
    let id = ViewId::new();

    let key_fn = move |(index, TabItem { kind: _kind, name: _name }): &(usize, TabItem<T>) | K::new(*index);

    let view_fn = move |(index, TabItem { kind: _kind, name }): (usize, TabItem<T>)| {
        println!("adding tab. tab_id: {:?}", index);

        let tab_name_label = label(move || name.get())
            .style(|s|s
                .margin_right(5)
                .color(Color::LIGHT_GRAY)
                .padding_horiz(5)
            );
        let close_button = button(
            "x".style(|s|s
                .padding(3)
                .color(Color::TRANSPARENT)
                .hover(|s|s
                    .color(Color::LIGHT_GRAY)
                )
            )
        )
            .action(move || {
                let key = K::new(index);

                println!("close clicked");
                event_signal.set(Some(TabBarEvent::TabClosed { key }));
            })
            .class(TabCloseButtonClass)
            .remove_class(ButtonClass)
            .style(|s|s
                .border(0)
                .margin(2)
                .hover(|s|s
                    .background(Color::DARK_GRAY)
                )
            );

        let container = (
            tab_name_label,
            close_button,
        )
            .h_stack()
            .style(|s|s
                .align_items(AlignItems::Baseline)
            );

        button(container)
            .action(move || {
                println!("tab clicked");
                let key = K::new(index);
                active_tab.set(Some(key.clone()))
            })
            .remove_class(ButtonClass)
            .class(TabItemClass)
            .style(move |s| {
                let key = K::new(index);
                let is_active = active_tab.get().map_or(false, |active_key|active_key.eq(&key));

                s
                    .border(0)
                    .border_bottom(2)
                    .background(Color::TRANSPARENT)
                    .border_color(Color::TRANSPARENT)
                    .apply_if(is_active, |s| s
                        .background(Color::DIM_GRAY)
                        .border_color(Color::LIGHT_GRAY)
                    )
            }
            )
            .into_any()
    };


    let dyn_stack = dyn_stack(each_fn, key_fn, view_fn)
        .class(TabBarClass);

    id.add_child(Box::new(dyn_stack));

    TabBar {
        id,
        active_tab,
        phantom_data: PhantomData,
    }
}

impl<T, K> View for TabBar<T, K>
where
    T: 'static,
    K: Eq + Hash + 'static
{
    fn id(&self) -> ViewId {
        self.id
    }

    fn debug_name(&self) -> std::borrow::Cow<'static, str> {
        "TabBar".into()
    }
}
