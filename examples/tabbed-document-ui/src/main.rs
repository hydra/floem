use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use image::{ImageFormat, Rgb, Rgba};
use slotmap::SlotMap;
use floem::action::open_file;
use floem::file::{FileDialogOptions, FileInfo, FileSpec};
use floem::IntoView;
use floem::peniko::Color;
use floem::reactive::{create_effect, create_rw_signal, provide_context, RwSignal, SignalGet, SignalUpdate, SignalWith, use_context};
use floem::views::{button, Decorators, dyn_stack, dyn_view, h_stack, tab, TupleStackExt};
use crate::config::Config;
use crate::documents::{DocumentContainer, DocumentKey, DocumentKind};
use crate::documents::image::ImageDocument;
use crate::documents::new_document_form::{NewDocumentForm, NewDocumentKind};
use crate::documents::text::TextDocument;
use crate::tabs::document::DocumentTab;
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
    tabs: RwSignal<Vec<TabKind>>,
    active_tab: RwSignal<Option<TabKey>>,
    config: Config,
}

fn app_view() -> impl IntoView {

    create_effect(|_|{
        let app_state: Option<Arc<ApplicationState>> = use_context();
        app_state.unwrap().documents.with(|_documents|{
            println!("with documents effect");
        })
    });

    let toolbar = h_stack((
        button("Add home").action(add_home_pressed),
        button("New").action(new_pressed),
        button("Open").action(open_pressed),
        button("Close all").action(close_all_pressed),
    ))
        .style(|s| s
            .width_full()
            .background(Color::parse("#eeeeee").unwrap())
        );

    let tab_bar = dyn_stack(
        move || {
            let app_state: Option<Arc<ApplicationState>> = use_context();

            app_state.unwrap().tabs.get().into_iter().enumerate()
        },
        move |(index, _tab_kind)| TabKey::new(*index),
        move |(index, tab_kind)| {
            println!("adding tab. tab_id: {:?}", index);

            match tab_kind {
                TabKind::Home(_home_tab) => {
                    button("Home")
                        .action(move || {
                            println!("Home tab pressed");
                            let app_state: Option<Arc<ApplicationState>> = use_context();
                            app_state.unwrap().active_tab.set(Some(TabKey::new(index)))
                        })
                        .into_any()
                }
                TabKind::Document(_document_tab) => {
                    button("Document")
                        .action(move || {
                            println!("Document tab pressed");
                            let app_state: Option<Arc<ApplicationState>> = use_context();
                            app_state.unwrap().active_tab.set(Some(TabKey::new(index)))
                        })
                        .into_any()
                }
            }
        }
    )
        .style(|s| s
            .width_full()
            .background(Color::parse("#dddddd").unwrap())
        );

    let document_container = tab(
        move || {
            println!("tab::active_fn");
            let app_state: Arc<ApplicationState> = use_context().unwrap();
            app_state.active_tab.get().map(|active|*active)
        },
        move || {
            println!("tab::each_fn");
            let app_state: Arc<ApplicationState> = use_context().unwrap();
            app_state.tabs.get().into_iter().enumerate()
        },
        // TODO investigate why we need this closure at all, it's not clear from the examples and there is no documentation.
        move |(index, _tab_kind)| {
            println!("tab::key_fn");
            TabKey::new(*index)
        },
        move |(index, active_tab)| {
            println!("tab::view_fn");

            let tab_key = TabKey::new(index);
            println!("displaying tab. tab_id: {:?}", &tab_key);

            let app_state: Arc<ApplicationState> = use_context().unwrap();

            // We need a `dyn_view` here to make the content update when `app_state.documents` is changed
            // this happens when a new document form is replaced with an actual document, but without
            // a new tab being created.
            dyn_view(move ||{
                match &active_tab {
                    TabKind::Home(_home_tab) => {
                        HomeContainer::build_view(tab_key).into_any()
                    }
                    TabKind::Document(document_tab) => {
                        app_state.documents.with(|documents|{
                            println!("building view");
                            let document = documents.get(document_tab.document_key).unwrap();
                            DocumentContainer::build_view(document).into_any()
                        })
                    }
                }
            })
        }
    )
        .style(|s| s
            .width_full()
            .height_full()
            .background(Color::DIM_GRAY)
        );

    (
        toolbar,
        tab_bar,
        document_container,
    )
        .v_stack()
        .style(|s| s
            .width_full()
            .height_full()
            .background(Color::LIGHT_GRAY)
        )
}

fn add_home_pressed() {
    println!("Add home pressed");

    let app_state: Arc<ApplicationState> = use_context().unwrap();

    app_state.tabs.update(|tabs|{
        tabs.push(
            TabKind::Home(HomeTab {})
        );
    });
}

fn close_all_pressed() {
    println!("Close all pressed");

    let app_state: Arc<ApplicationState> = use_context().unwrap();

    app_state.active_tab.set(None);
    app_state.tabs.update(|tabs|tabs.clear())
}

fn new_pressed() {
    println!("New pressed");


    let event_signal = NewDocumentForm::create_event_signal();

    create_effect(move |_|{

        event_signal.with(|event| match event {
            Some((event, document_key))      => {
                println!("event: {:?}", &event);

                let app_state: Arc<ApplicationState> = use_context().unwrap();
                app_state.documents.update(|documents|{
                    let document = documents.get_mut(document_key.clone()).unwrap();
                    if let DocumentKind::NewDocumentForm(form) = document {
                        println!("kind: {:?}", form.kind.get());
                        println!("name: {:?}", form.name.get());
                        println!("directory_path: {:?}", form.directory_path.get());

                        let mut path = form.directory_path.get().clone();
                        path.push(form.name.get());

                        let new_document_kind = form.kind.get();

                        let new_document = match new_document_kind {
                            NewDocumentKind::Text => {
                                {
                                    let mut file = File::create_new(path.clone()).unwrap();
                                    file.write("New file content".as_bytes()).expect("bytes should be written");
                                }

                                DocumentKind::TextDocument(TextDocument::new(path))
                            },
                            NewDocumentKind::Bitmap => {
                                {
                                    let imgbuf = image::ImageBuffer::<Rgba<u8>, Vec<u8>>::new(256, 256);

                                    let mut file = File::create_new(path.clone()).unwrap();
                                    imgbuf.write_to(&mut file, ImageFormat::Bmp).expect("should write to file");
                                }

                                DocumentKind::ImageDocument(ImageDocument::new(path))
                            }
                        };

                        // Replace the document, currently the form, with a text document
                        *document = new_document;
                    }

                    println!("documents: {:?}", documents)
                })

            }
            _ => ()
        });
    });


    let app_state: Arc<ApplicationState> = use_context().unwrap();

    let document_key = app_state.documents.try_update(|documents| {

        let new_document_form = NewDocumentForm::new(event_signal);
        let document = DocumentKind::NewDocumentForm(new_document_form);

        let document_key = documents.insert(document);

        let document = documents.get_mut(document_key).unwrap();
        if let DocumentKind::NewDocumentForm(new_document_form) = document {
            new_document_form.set_document_key(document_key);
        }

        document_key
    }).unwrap();

    let tab_key = app_state.tabs.try_update(|tabs| {
        tabs.push(
            TabKind::Document(DocumentTab { document_key })
        );

        TabKey::new(tabs.len() - 1)
    });

    app_state.active_tab.set(tab_key);
}

fn open_pressed() {
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

        let tab_key = app_state.tabs.try_update(|tabs| {
            tabs.push(
                TabKind::Document(DocumentTab { document_key })
            );

            TabKey::new(tabs.len() - 1)
        });

        app_state.active_tab.set(tab_key);
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
    let tab_key = app_state.tabs.try_update(|tabs| {
        tabs.push(
            TabKind::Home(HomeTab {})
        );

        TabKey::new(tabs.len() - 1)
    });

    app_state.active_tab.set(tab_key);
}
