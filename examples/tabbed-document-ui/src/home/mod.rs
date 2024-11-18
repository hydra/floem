use floem::peniko::Color;
use floem::prelude::ViewTuple;
use floem::unit::UnitExt;
use floem::View;
use floem::views::Decorators;

pub struct HomeContainer {}

impl HomeContainer {
    pub fn build_view() -> impl View {
        (
            "Tabbed UI example".style(|s| s.padding(20.px())),
            "\u{1F3E0}".style(|s| s
                .padding(20.px())
            )
        )
            .v_stack()
            .style(|s|s
                .font_size(32)
                .color(Color::WHITE)
                .width_full()
                .height_full()
                .items_center()
                .justify_center()
                .flex_col()
            )
    }
}
