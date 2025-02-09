use std::ops::Deref;

use crate::prelude::*;

enum VirtualTableEvent {
    Scroll(f32),
}

#[derive(Lens)]
pub struct VirtualTable {
    column_widths: Vec<Units>,
    scroll_x: f32,
}

impl VirtualTable {
    pub fn new<L1: Lens, T: 'static, L2: Lens, U: 'static>(
        cx: &mut Context,
        columns: L1,
        rows: L2,
        row_height: f32,
        column_content: impl 'static + Copy + Fn(&mut Context, usize, MapRef<L1, T>),
        row_content: impl 'static + Copy + Fn(&mut Context, usize, MapRef<L2, U>),
    ) -> Handle<Self>
    where
        L1::Target: Deref<Target = [T]> + Data,
        L2::Target: Deref<Target = [U]> + Data,
    {
        let num_columns = columns.get_ref(cx).unwrap().len();
        let num_rows = rows.get_ref(cx).unwrap().len();
        Self { column_widths: vec![Pixels(100.0); num_columns], scroll_x: 0.0 }.build(
            cx,
            move |cx| {
                List::new(cx, columns, move |cx, index, item| {
                    HStack::new(cx, move |cx| {
                        column_content(cx, index, item);
                        Divider::vertical(cx);
                    })
                    .gap(Stretch(1.0))
                    .size(Self::column_widths.map(move |widths| widths[index]))
                    .padding_left(Pixels(5.0))
                    .alignment(Alignment::Left);
                })
                .show_horizontal_scrollbar(false)
                .scroll_x(Self::scroll_x)
                .horizontal(true);
                Divider::horizontal(cx);
                VirtualList::new(cx, rows, row_height, move |cx, index, item| {
                    VStack::new(cx, |cx| {
                        Grid::new(
                            cx,
                            vec![Pixels(100.0); num_columns],
                            vec![Pixels(row_height)],
                            move |cx| {
                                for i in 0..num_columns {
                                    HStack::new(cx, |cx| {
                                        row_content(cx, i, item);
                                    })
                                    .alignment(Alignment::Left)
                                    .column_start(i)
                                    .row_start(0);
                                }
                            },
                        )
                        .width(Pixels(100.0 * num_columns as f32));
                        if index != num_rows - 1 {
                            Divider::horizontal(cx);
                        }
                    })
                    .size(Auto)
                })
                .scroll_x(Self::scroll_x)
                .on_scroll(|cx, x, _| cx.emit(VirtualTableEvent::Scroll(x)));
            },
        )
    }
}

impl View for VirtualTable {
    fn element(&self) -> Option<&'static str> {
        Some("virtual-table")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|virtual_list_event, _| match virtual_list_event {
            VirtualTableEvent::Scroll(x) => {
                self.scroll_x = *x;
            }
        });
    }
}
