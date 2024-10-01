use std::fs;
use std::path::PathBuf;
use floem::peniko::Color;
use floem::reactive::SignalGet;
use floem::style::TextOverflow;
use floem::View;
use floem::views::{Decorators, label, static_label, text_editor, TupleStackExt};

pub struct TextDocument {
    path: PathBuf,
    content: String,
}

impl TextDocument {

    pub fn new(path: PathBuf) -> Self {

        let content = fs::read_to_string(&path).unwrap();

        Self {
            path,
            content,
        }
    }

    pub fn build_view(&self) -> impl View {
        let editor = text_editor(self.content.clone());
        let cursor = editor.editor().cursor;

        let info_panel = (
            {
                (
                    static_label("path"),
                    static_label(self.path.to_str().unwrap())
                        .style(|s| s
                            // FIXME doesn't make any difference, path appears truncated
                            .text_overflow(TextOverflow::Wrap)
                            // FIXME this doesn't work either
                            //.text_clip()
                        )
                )
                    .h_stack()
            },
            {
                (
                    static_label("selection"),
                    {
                        label(move || {
                            let cursor = cursor.get();
                            let selection = cursor.get_selection();

                            selection
                                .map_or_else(
                                    || "None".to_string(),
                                    |(start, end)| {
                                        format!("offset: {}, length: {}", start, (end as i32 - start as i32).abs())
                                    }
                                )
                        })
                    }
                )
                    .h_stack()
            }
        )
            .v_stack()
            .style(|s| s
                .height_full()
                .width_pct(20.0)
            );

        let content = {
            editor
        }
            .style(|s| s
                .height_full()
                .width_full()
                .background(Color::DARK_GRAY)
            );


        (
            info_panel,
            content,
        )
            .h_stack()
            .style(|s|s
                .width_full()
                .height_full()
            )
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
