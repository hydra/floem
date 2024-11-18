use std::sync::Arc;
use slotmap::SlotMap;
use floem::action::open_file;
use floem::event::Event;
use floem::file::{FileDialogOptions, FileInfo, FileSpec};
use floem::IntoView;
use floem::peniko::Color;
use floem::reactive::{create_effect, create_rw_signal, provide_context, RwSignal, SignalGet, SignalUpdate, SignalWith, use_context};
use floem::views::{button, Decorators, dyn_container, dyn_stack, empty, h_stack, v_stack};
use crate::config::Config;
use crate::documents::{DocumentKey, DocumentKind};
use crate::documents::image::ImageDocument;
use crate::documents::text::TextDocument;
use crate::tabs::document::{DocumentContainer, DocumentTab};
use crate::tabs::home::{HomeContainer, HomeTab};
use crate::tabs::{TabKey, TabKind};

pub mod config;
pub mod documents;
pub mod tabs;

fn main() {
    let config = config::load();

    let app_state = ApplicationState {
        documents: create_rw_signal(Default::default()),
        tabs: create_rw_signal(Default::default()),
        active_tab: create_rw_signal(None),
        config,
    };

    if app_state.config.show_home_on_startup {
        show_home_tab(&app_state);
    }

    let app_state_arc = Arc::new(app_state);

    provide_context(app_state_arc.clone());

    floem::launch(app_view);

    config::save(&app_state_arc.config);
}

struct ApplicationState {
    documents: RwSignal<SlotMap<DocumentKey, DocumentKind>>,
    tabs: RwSignal<SlotMap<TabKey, TabKind>>,
    active_tab: RwSignal<Option<TabKey>>,
    config: Config,
}

fn app_view() -> impl IntoView {
    let toolbar = h_stack((
        button("Add home").action(add_home_pressed),
        button("New").action(new_pressed),
        button("Open").action(open_pressed),
    ))
        .style(|s| s
            .width_full()
            .background(Color::parse("#eeeeee").unwrap())
        );

    let tab_bar = dyn_stack(
        move || {
            let app_state: Option<Arc<ApplicationState>> = use_context();

            app_state.unwrap().tabs.get()
        },
        move |(tab_key, _tab_kind)| tab_key.clone(),
        move |(tab_key, tab_kind)| {
            println!("adding tab. tab_id: {:?}", tab_key);

            match tab_kind {
                TabKind::Home(_home_tab) => {
                    button("Home")
                        .action(move |_event| {
                            println!("Home tab pressed");
                            let app_state: Option<Arc<ApplicationState>> = use_context();
                            app_state.unwrap().active_tab.set(Some(tab_key))
                        }).into_any()
                }
                TabKind::Document(_document_tab) => {
                    button("Document")
                        .action(move |_event| {
                            println!("Document tab pressed");
                            let app_state: Option<Arc<ApplicationState>> = use_context();
                            app_state.unwrap().active_tab.set(Some(tab_key))
                        }).into_any()
                }
            }
        }
    )
        .style(|s| s
            .width_full()
            .background(Color::parse("#dddddd").unwrap())
        );

    let document_container = dyn_container(
        move || {
            let app_state: Option<Arc<ApplicationState>> = use_context();
            app_state.unwrap().active_tab.get()
        },
        move |active_tab| {
            let app_state: Arc<ApplicationState> = use_context().unwrap();
            if let Some(tab_key) = active_tab {
                println!("displaying tab. tab_id: {:?}", &tab_key);

                let tabs_signal = app_state.tabs;
                let view = tabs_signal.with_untracked(|tabs| {
                    let tab = tabs.get(tab_key).unwrap().clone();

                    match tab {
                        TabKind::Home(_home_tab) => {
                            HomeContainer::build_view(tab_key).into_any()
                        }
                        TabKind::Document(document_tab) => {
                            DocumentContainer::build_view(document_tab.document_key).into_any()
                        }
                    }
                });

                view
            } else {
                empty().into_any()
            }
        }
    )
        .style(|s| s
            .width_full()
            .height_full()
            .background(Color::DIM_GRAY)
        );

    v_stack((
        toolbar,
        tab_bar,
        document_container,
    ))
        .style(|s| s
            .width_full()
            .height_full()
            .background(Color::LIGHT_GRAY)
        )
}

fn add_home_pressed(_event: &Event) {
    println!("Add home pressed");

    let app_state: Arc<ApplicationState> = use_context().unwrap();

    app_state.tabs.update(|tabs|{
        tabs.insert(
            TabKind::Home(HomeTab {})
        );
    });
}

fn new_pressed(_event: &Event) {
    println!("New pressed");
}

fn open_pressed(_event: &Event) {
    println!("Open pressed");

    let opened_file: RwSignal<Option<FileInfo>> = RwSignal::new(None);

    create_effect(move |_|{
        let Some(file_info) = opened_file.get() else {
            return;
        };

        println!("Selected file: {:?}", file_info.path);

        let app_state: Arc<ApplicationState> = use_context().unwrap();

        let path = file_info.path.first().unwrap();

        let document = match path.extension().unwrap().to_str().unwrap() {
            "txt" => {
                let text_document = TextDocument::new(path.clone());

                DocumentKind::TextDocument(text_document)
            },
            "bmp" => {
                let image_document = ImageDocument::new(path.clone());

                DocumentKind::ImageDocument(image_document)
            },
            _ => unreachable!()
        };

        let document_key = app_state.documents.try_update(|documents| {
            documents.insert(document)
        }).unwrap();

        app_state.tabs.update(|tabs| {
            let tab_key = tabs.insert(
                TabKind::Document(DocumentTab { document_key })
            );

            app_state.active_tab.set(Some(tab_key));
        });
    });

    open_file(
        FileDialogOptions::new()
            .title("Select a file")
            .allowed_types(vec![
                FileSpec {
                    name: "text",
                    extensions: &["txt"],
                },
                FileSpec {
                    name: "image",
                    extensions: &["bmp"],
                }
            ]),
        move |file_info| {
            if file_info.is_some() {
                opened_file.set(file_info);
            }
        },
    );
}

fn show_home_tab(app_state: &ApplicationState) {
    app_state.tabs.update(|tabs| {
        let tab_key = tabs.insert(
            TabKind::Home(HomeTab {})
        );

        app_state.active_tab.set(Some(tab_key));
    })
}
