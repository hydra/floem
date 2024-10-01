use floem::unit::UnitExt;
use floem::View;
use floem::view_tuple::ViewTuple;
use floem::views::{container, Decorators, stack};

pub fn info_panel_row<V: View + 'static>(title: &'static str, rhs: V) -> impl View {
    container(
        stack((
            container(title)
                .style(|s|s
                    .width(50.0)
                    .margin_right(10.0)
                )
            ,
            rhs,
        ))
            .style(|s|s
                .flex_row()
            )
    )
        .style(|s|s
            .flex_row()
            .width(100.pct())
        )
}

pub fn info_panel<VT: ViewTuple + 'static>(children: VT) -> impl View {
    container(
        stack(children)
            .style(|s| s
                .flex_col()
            )
    )
}
