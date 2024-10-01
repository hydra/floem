use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;
use floem::{IntoView, View, ViewId};
use floem::peniko::Color;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::views::{button, Decorators, dyn_stack, label};

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

pub fn tab_bar<IF, I, T, K>(active_tab: RwSignal<Option<K>>, each_fn: IF) -> TabBar<T, K>
where
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = (usize, TabItem<T>)>,
    K: Eq + Hash + TabKeyFactory<K> + 'static,
    T: Clone + 'static,
{
    let id = ViewId::new();

    let key_fn = move |(index, TabItem { kind: _kind, name: _name }): &(usize, TabItem<T>) | K::new(*index);

    let view_fn = move |(index, TabItem { kind: _kind, name }): (usize, TabItem<T>)| {
        println!("adding tab. tab_id: {:?}", index);

        let tab_name_label = label(move || name.get());

        button(tab_name_label)
            .action(move || {
                active_tab.set(Some(K::new(index)))
            })
            .into_any()
    };


    let dyn_stack = dyn_stack(each_fn, key_fn, view_fn)
        .style(|s| s
            .background(Color::parse("#dddddd").unwrap())
        );

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
