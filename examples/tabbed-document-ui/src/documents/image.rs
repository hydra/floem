use std::path::PathBuf;
use floem::peniko::Color;
use floem::style::TextOverflow;
use floem::View;
use floem::views::{Decorators, h_stack, img_from_path, label, static_label, v_stack};

pub struct ImageDocument {
    path: PathBuf,
    // TODO content: ImageContent(...)
    coordinate: Option<(usize, usize)>
}


impl ImageDocument {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            coordinate: None,
        }
    }

    pub fn build_view(&self) -> impl View {
        h_stack((
            //
            // info panel
            //
            v_stack((
                {
                    h_stack((
                        static_label("path"),
                        static_label(self.path.to_str().unwrap())
                            .style(|s|s
                                // FIXME doesn't make any difference, path appears truncated
                                .text_overflow(TextOverflow::Wrap)
                            )
                    ))
                },
                {
                    h_stack((
                        static_label("coordinate"),
                        {
                            // FIXME this needs to be reactive
                            let coordinate_label = format!("{:?}", self.coordinate);
                            label(move || coordinate_label.clone())
                        }
                    ))
                }
            ))
                .style(|s|s
                    .height_full()
                    .width_pct(20.0)
                ),

            //
            // content
            //
            {
                // TODO show the image
                let path = self.path.clone();
                img_from_path(move || path.clone())
                    .style(|s|s
                        .min_height(256)
                        .min_width(256)
                    )
            }
                .style(|s|s
                    .height_full()
                    // FIXME if this is 80% or 'full' it still doesn't take up the remaining space.
                    .width_full()
                    .background(Color::PURPLE)
                ),
        ))
            .style(|s|s
                .width_full()
                .height_full()
            )
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

}
