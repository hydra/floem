use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use strum::IntoEnumIterator;
use floem::reactive::{create_effect, create_rw_signal, RwSignal, SignalGet, SignalUpdate};
use floem::{IntoView, View};
use floem::action::open_file;
use floem::file::{FileDialogOptions, FileInfo};
use floem::peniko::Color;
use std::default::Default;
use floem::views::{button, container, Decorators, empty, label, stack, svg, text_input};
use floem::views::dropdown::Dropdown;
use floem::views::TupleStackExt;
use crate::documents::DocumentKey;

#[derive(strum::EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum NewDocumentKind {
    Text,
    Bitmap,
}

impl Display for NewDocumentKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

pub type NewDocumentEventSignal = RwSignal<Option<(NewDocumentEvent, DocumentKey)>>;

pub struct NewDocumentForm {
    pub(crate) kind: RwSignal<NewDocumentKind>,
    pub(crate) name: RwSignal<String>,
    pub(crate) directory_path: RwSignal<PathBuf>,
    event_signal: NewDocumentEventSignal,

    document_key: Option<DocumentKey>,
    pub(crate) file_path_signal: RwSignal<String>
}

#[derive(Debug)]
pub enum NewDocumentEvent {
    Ok
}

impl NewDocumentForm {

    pub fn new(event_signal: NewDocumentEventSignal, file_path_signal: RwSignal<String>) -> Self {
        Self {
            kind: create_rw_signal(NewDocumentKind::Text),
            name: create_rw_signal(Default::default()),
            directory_path: create_rw_signal(Default::default()),
            document_key: None,
            file_path_signal,
            event_signal
        }
    }

    pub fn create_event_signal() -> NewDocumentEventSignal {
        create_rw_signal(None)
    }

    pub fn set_document_key(&mut self, document_key: DocumentKey) {
        self.document_key.replace(document_key);
    }

    pub fn set_file_path_signal(&mut self, signal: RwSignal<String>) {
        self.file_path_signal = signal;
    }

    fn on_directory_pressed(file_info_signal: RwSignal<Option<FileInfo>>) {
        open_file(
            FileDialogOptions::new()
                .select_directories(),

            move|file_info| {
                if file_info.is_some() {
                    file_info_signal.set(file_info);
                }
            }
        )
    }

    pub fn build_view(&self) -> impl View {

        let file_info_signal: RwSignal<Option<FileInfo>> = create_rw_signal(None);
        let directory_path_signal = self.directory_path.clone();
        let event_signal = self.event_signal.clone();
        let document_key = self.document_key.clone();

        create_effect(move |_| {
            file_info_signal.get().inspect(|file_info|{
                let binding = file_info.clone();
                let path = binding.path.first().unwrap();
                directory_path_signal.set(path.clone());
            });
        });

        create_effect(move |_| {
            println!("directory: {:?}", directory_path_signal.get())
        });

        const CHEVRON_DOWN: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" xml:space="preserve" viewBox="0 0 185.344 185.344">
          <path fill="#010002" d="M92.672 144.373a10.707 10.707 0 0 1-7.593-3.138L3.145 59.301c-4.194-4.199-4.194-10.992 0-15.18a10.72 10.72 0 0 1 15.18 0l74.347 74.341 74.347-74.341a10.72 10.72 0 0 1 15.18 0c4.194 4.194 4.194 10.981 0 15.18l-81.939 81.934a10.694 10.694 0 0 1-7.588 3.138z"/>
        </svg>"##;

        let dropdown_view = move |item| {
            stack((
                label(move || item),
                container(svg(CHEVRON_DOWN).style(|s| s.size(12, 12).color(Color::BLACK)))
            ))
                .into_any()
        };

        let document_kind_row = (
            "File type",
            Dropdown::new_get_set(
                self.kind,
                dropdown_view,
                NewDocumentKind::iter(),
                |list_item| label(move || list_item).into_any()
            )
        ).h_stack();

        let name_row = (
            "Name",
            text_input(self.name)
        ).h_stack();

        let directory_row = (
            "Directory",
            label(move || directory_path_signal.get()
                .to_str()
                .unwrap()
                .to_string()
            ),
            button("...")
               .action(move ||Self::on_directory_pressed(file_info_signal))
        ).h_stack();

        let button_row = (
            empty()
                .style(|s|s
                    .width_pct(75.0)
                ),
            button("Ok")
                .action(move ||{
                    event_signal.set(Some(
                        (
                            NewDocumentEvent::Ok,
                            document_key.clone().unwrap()
                        )
                    ));
                })
                .style(|s|s
                    .width_pct(25.0)
                ),
        ).h_stack();

        (
            document_kind_row,
            name_row,
            directory_row,
            button_row,
        ).v_stack()
    }
}