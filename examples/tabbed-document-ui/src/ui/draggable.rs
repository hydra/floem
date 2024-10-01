use floem::event::{Event, EventListener, EventPropagation};
use floem::peniko::Color;
use floem::reactive::{create_rw_signal, SignalGet, SignalUpdate};
use floem::style::{CursorStyle, Position};
use floem::unit::Px;
use floem::View;
use floem::views::{Decorators, empty, h_stack};

pub fn h_draggable_container<V1: View + 'static, V2: View + 'static>(v1: V1, v2: V2, initial_position: Px, handle_width: Px, width: Px) -> impl View {

    let sidebar_width = create_rw_signal(initial_position);
    let is_sidebar_dragging = create_rw_signal(false);

    let dragger = empty()
        .style(move |s| {
            s.position(Position::Absolute)
                .z_index(10)
                .inset_top(0)
                .inset_bottom(0)
                .inset_left(sidebar_width.get())
                .width(handle_width)
                .border_left(width)
                .border_color(Color::rgb8(205, 205, 205))
                .hover(|s| s
                    .border_color(Color::rgb8(41, 98, 218))
                    .cursor(CursorStyle::ColResize)
                )
                .apply_if(is_sidebar_dragging.get(), |s| s
                    .border_color(Color::rgb8(41, 98, 218))
                )
        })
        .draggable()
        .dragging_style(|s| s
            .border_color(Color::TRANSPARENT)
            .cursor(CursorStyle::ColResize)
        )
        .on_event(EventListener::DragStart, move |_| {
            is_sidebar_dragging.set(true);
            EventPropagation::Continue
        })
        .on_event(EventListener::DragEnd, move |_| {
            is_sidebar_dragging.set(false);
            EventPropagation::Continue
        })
        .on_event(EventListener::DoubleClick, move |_| {
            sidebar_width.set(initial_position);
            EventPropagation::Continue
        });

    let v1 = v1
        .style(move |s| {
            s.width(sidebar_width.get())
        });

    let v2 = v2
        .style(|s| {
            s.flex_col()
                .flex_basis(0)
                .min_width(0)
                .flex_grow(1.0)
        });


    let view = h_stack((v1, dragger, v2))
        .on_event(EventListener::PointerMove, move |event| {
            let pos = match event {
                Event::PointerMove(p) => p.pos,
                _ => (0.0, 0.0).into(),
            };

            if is_sidebar_dragging.get() {
                sidebar_width.set(pos.x.into());
            }
            EventPropagation::Continue
        })
        .style(|s| s
            .width_full()
            .height_full()
        );

    view
}