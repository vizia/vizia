use std::{collections::BTreeSet, ops::Deref};

use crate::{icons::ICON_ARROWS_SORT, prelude::*};

pub enum ColumnSortDirection {
    Ascending,
    Descending,
}

pub enum VirtualTableEvent {
    Scroll(f32),
    ResizeColumn(usize, f32),
    SortColumn(usize),
}

#[derive(Lens)]
pub struct VirtualTable {
    num_rows: usize,
    column_widths: Vec<Units>,
    scroll_x: f32,
    sort_column: Option<usize>,
    sort_direction: bool,
    on_sort_column: Option<Box<dyn Fn(&mut EventContext, usize, bool)>>,
    // The column being dragged
    drag_column: Option<usize>,

    selected: BTreeSet<usize>,
    selectable: Selectable,
    focused: Option<usize>,
    focus_visible: bool,
    selection_follows_focus: bool,
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
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
        Self {
            num_rows: rows.get_ref(cx).unwrap().len(),
            column_widths: vec![Pixels(100.0); num_columns],
            scroll_x: 0.0,
            sort_column: None,
            sort_direction: false,
            on_sort_column: None,
            drag_column: None,

            selected: BTreeSet::default(),
            selectable: Selectable::None,
            focused: None,
            focus_visible: false,
            selection_follows_focus: false,
            on_select: None,
        }
        .build(cx, move |cx| {
            Keymap::from(vec![
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowDown),
                    KeymapEntry::new("Focus Next", |cx| cx.emit(ListEvent::FocusNext)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::ArrowUp),
                    KeymapEntry::new("Focus Previous", |cx| cx.emit(ListEvent::FocusPrev)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Space),
                    KeymapEntry::new("Select Focused", |cx| cx.emit(ListEvent::SelectFocused)),
                ),
                (
                    KeyChord::new(Modifiers::empty(), Code::Enter),
                    KeymapEntry::new("Select Focused", |cx| cx.emit(ListEvent::SelectFocused)),
                ),
            ])
            .build(cx);

            List::new(cx, columns, move |cx, index, item| {
                ResizableStack::new(
                    cx,
                    Self::column_widths.idx(index),
                    ResizeStackDirection::Right,
                    move |cx, new_size| {
                        cx.emit(VirtualTableEvent::ResizeColumn(index, new_size));
                    },
                    move |cx| {
                        if index != 0 {
                            Divider::vertical(cx);
                        }

                        column_content(cx, index, item);
                        Spacer::new(cx);
                        Button::new(cx, |cx| Svg::new(cx, ICON_ARROWS_SORT))
                            .size(Pixels(16.0))
                            .on_press(move |cx| cx.emit(VirtualTableEvent::SortColumn(index)));
                        if index == num_columns - 1 {
                            Divider::vertical(cx);
                        }
                    },
                )
                .min_width(Pixels(0.0))
                .overflow(Overflow::Hidden)
                .layout_type(LayoutType::Row)
                .gap(Pixels(4.0))
                //.padding_left(Pixels(5.0))
                .alignment(Alignment::Left);

                // HStack::new(cx, move |cx| {
                //     column_content(cx, index, item);
                //     Divider::vertical(cx);
                // })
                // .gap(Stretch(1.0))
                // .size(Self::column_widths.map(move |widths| widths[index]))
                // .padding_left(Pixels(5.0))
                // .alignment(Alignment::Left);
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
                                .overflow(Overflow::Hidden)
                                .alignment(Alignment::Left)
                                .column_start(i)
                                .row_start(0);
                            }
                        },
                    )
                    .hoverable(false)
                    .grid_columns(Self::column_widths)
                    .width(Self::column_widths.map(|widths| {
                        Pixels(widths.iter().map(|width| width.to_px(0.0, 0.0)).sum::<f32>())
                    }));
                    let num_rows = rows.get_ref(cx).unwrap().len();
                    if num_rows > 0 && index != num_rows - 1 {
                        Divider::horizontal(cx);
                    }
                })
                .class("table-row")
                .size(Auto)
                .checked(VirtualTable::selected.map(move |selected| selected.contains(&index)))
                //.toggle_class("focused", List::focused.map(move |focused| *focused == Some(index)))
                // .focused_with_visibility(
                //     VirtualTable::focused.map(move |f| *f == Some(index)),
                //     VirtualTable::focus_visible,
                // )
                .on_press(move |cx| cx.emit(ListEvent::Select(index)))
            })
            .scroll_x(Self::scroll_x)
            .on_scroll(|cx, x, _| cx.emit(VirtualTableEvent::Scroll(x)));

            // Binding::new(cx, Self::drag_column, |cx, drag_column| {
            //     if let Some(drag_column) = drag_column.get(cx) {
            //         HStack::new(cx, move |cx| {
            //             column_content(cx, drag_column, item);
            //             Divider::vertical(cx);
            //         })
            //         .gap(Stretch(1.0))
            //         .size(Self::column_widths.map(move |widths| widths[index]))
            //         .padding_left(Pixels(5.0))
            //         .alignment(Alignment::Left);
            //     }
            // });
        })
        .toggle_class("selectable", VirtualTable::selectable.map(|s| *s != Selectable::None))
        .bind(rows.map(|rows| rows.len()), |handle, num_rows| {
            let num_rows = num_rows.get(&handle);
            handle.modify(|list| list.num_rows = num_rows);
        })
    }
}

impl View for VirtualTable {
    fn element(&self) -> Option<&'static str> {
        Some("virtual-table")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|virtual_table_event, _| match virtual_table_event {
            VirtualTableEvent::Scroll(x) => {
                self.scroll_x = *x;
            }

            VirtualTableEvent::ResizeColumn(index, width) => {
                self.column_widths[*index] = Pixels(*width);
            }

            VirtualTableEvent::SortColumn(index) => {
                if self.sort_column == Some(*index) {
                    self.sort_direction = !self.sort_direction;
                } else {
                    self.sort_column = Some(*index);
                    self.sort_direction = false;
                }
                if let Some(callback) = &self.on_sort_column {
                    callback(cx, *index, self.sort_direction);
                }
            }
        });

        event.take(|list_event, _| match list_event {
            ListEvent::Select(index) => {
                println!("Select {}", index);
                cx.focus();
                match self.selectable {
                    Selectable::Single => {
                        if self.selected.contains(&index) {
                            self.selected.clear();
                            self.focused = None;
                        } else {
                            self.selected.clear();
                            self.selected.insert(index);
                            self.focused = Some(index);
                            self.focus_visible = false;
                            if let Some(on_select) = &self.on_select {
                                on_select(cx, index);
                            }
                        }
                    }

                    Selectable::Multi => {
                        if self.selected.contains(&index) {
                            self.selected.remove(&index);
                            self.focused = None;
                        } else {
                            self.selected.insert(index);
                            self.focused = Some(index);
                            self.focus_visible = false;
                            if let Some(on_select) = &self.on_select {
                                on_select(cx, index);
                            }
                        }
                    }

                    Selectable::None => {}
                }
            }

            ListEvent::SelectFocused => {
                if let Some(focused) = &self.focused {
                    cx.emit(ListEvent::Select(*focused))
                }
            }

            ListEvent::ClearSelection => {
                self.selected.clear();
            }

            ListEvent::FocusNext => {
                println!("Focus Next");
                if let Some(focused) = &mut self.focused {
                    *focused = focused.saturating_add(1);

                    if *focused >= self.num_rows {
                        *focused = 0;
                    }
                } else {
                    self.focused = Some(0);
                }

                self.focus_visible = true;

                if self.selection_follows_focus {
                    cx.emit(ListEvent::SelectFocused);
                }
            }

            ListEvent::FocusPrev => {
                if let Some(focused) = &mut self.focused {
                    if *focused == 0 {
                        *focused = self.num_rows;
                    }

                    *focused = focused.saturating_sub(1);
                } else {
                    self.focused = Some(self.num_rows.saturating_sub(1));
                }

                self.focus_visible = true;

                if self.selection_follows_focus {
                    cx.emit(ListEvent::SelectFocused);
                }
            }

            _ => {}
        });
    }
}

impl Handle<'_, VirtualTable> {
    pub fn on_sort_column(mut self, f: impl Fn(&mut EventContext, usize, bool) + 'static) -> Self {
        self.modify(|virtual_table| virtual_table.on_sort_column = Some(Box::new(f)))
    }

    /// Sets the  selected items of the list. Takes a lens to a list of indices.
    pub fn selected<S: Lens>(self, selected: S) -> Self
    where
        S::Target: Deref<Target = [usize]> + Data,
    {
        self.bind(selected, |handle, s| {
            let ss = s.get(&handle).deref().to_vec();
            handle.modify(|list| {
                list.selected.clear();
                for idx in ss {
                    list.selected.insert(idx);
                    list.focused = Some(idx);
                }
            });
        })
    }

    /// Sets the callback triggered when a [ListItem] is selected.
    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize),
    {
        self.modify(|list| list.on_select = Some(Box::new(callback)))
    }

    /// Set the selectable state of the [List].
    pub fn selectable<U: Into<Selectable>>(self, selectable: impl Res<U>) -> Self {
        self.bind(selectable, |handle, selectable| {
            let s = selectable.get(&handle).into();
            handle.modify(|list| list.selectable = s);
        })
    }

    /// Sets whether the selection should follow the focus.
    pub fn selection_follows_focus<U: Into<bool>>(self, flag: impl Res<U>) -> Self {
        self.bind(flag, |handle, selection_follows_focus| {
            let s = selection_follows_focus.get(&handle).into();
            handle.modify(|list| list.selection_follows_focus = s);
        })
    }
}

// pub enum ResizeHandleEvent {
//     StartDrag,
//     StopDrag,
//     Drag(f32),
// }

// pub struct ResizeHandle {
//     is_dragging: bool,
//     column_index: usize,
//     on_drag: Box<dyn Fn(&mut EventContext, f32)>,
// }

// impl ResizeHandle {}

// impl View for ResizeHandle {
//     fn element(&self) -> Option<&'static str> {
//         Some("resize-handle")
//     }

//     fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
//         event.map(|resizable_stack_event, event| match resizable_stack_event {
//             ResizeHandleEvent::StartDrag => {
//                 self.is_dragging = true;
//                 cx.capture();
//                 cx.lock_cursor_icon();

//                 // Disable pointer events for everything while dragging
//                 cx.with_current(Entity::root(), |cx| {
//                     cx.set_pointer_events(false);
//                 });

//                 // Prevent propagation
//                 event.consume();
//             }

//             ResizeHandleEvent::StopDrag => {
//                 self.is_dragging = false;
//                 cx.release();
//                 cx.unlock_cursor_icon();

//                 // Re-enable pointer events
//                 cx.with_current(Entity::root(), |cx| {
//                     cx.set_pointer_events(true);
//                 });

//                 event.consume()
//             }

//             ResizeHandleEvent::Drag(new_size) => {
//                 (self.on_drag)(cx, *new_size);
//             }
//         });

//         event.map(|window_event, _| match window_event {
//             WindowEvent::MouseMove(x, y) => {
//                 let dpi = cx.scale_factor();
//                 if self.is_dragging {
//                     let new_size = {
//                         let posx = cx.bounds().x;
//                         (*x - posx) / dpi
//                     };

//                     (self.on_drag)(cx, new_size);
//                 }
//             }

//             WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
//                 cx.emit(ResizeHandleEvent::StopDrag);
//             }

//             _ => {}
//         });
//     }
// }

#[derive(PartialEq, Clone, Copy)]
pub enum ResizeStackDirection {
    Right,
    Bottom,
}

// A view which can be resized by clicking and dragging from the right/bottom edge of the view.
#[derive(Lens)]
pub struct ResizableStack {
    // State which tracks whether the edge of the view is being dragged.
    is_dragging: bool,
    // Callback which is triggered when the view is being dragged.
    on_drag: Box<dyn Fn(&mut EventContext, f32)>,

    direction: ResizeStackDirection,
}

impl ResizableStack {
    pub fn new<F>(
        cx: &mut Context,
        size: impl Lens<Target = Units>,
        direction: ResizeStackDirection,
        on_drag: impl Fn(&mut EventContext, f32) + 'static,
        content: F,
    ) -> Handle<Self>
    where
        F: FnOnce(&mut Context),
    {
        let handle =
            Self { is_dragging: false, on_drag: Box::new(on_drag), direction }.build(cx, |cx| {
                if direction == ResizeStackDirection::Right {
                    Element::new(cx)
                        .width(Pixels(6.0))
                        .left(Stretch(1.0))
                        .right(Pixels(-4.0))
                        .position_type(PositionType::Absolute)
                        .z_index(10)
                        .class("resize_handle")
                        .toggle_class("drag_handle", ResizableStack::is_dragging)
                        .cursor(CursorIcon::EwResize)
                        .on_press_down(|cx| cx.emit(ResizableStackEvent::StartDrag));
                } else {
                    Element::new(cx)
                        .height(Pixels(6.0))
                        .top(Stretch(1.0))
                        .bottom(Pixels(-4.0))
                        .position_type(PositionType::Absolute)
                        .z_index(10)
                        .class("resize_handle")
                        .toggle_class("drag_handle", ResizableStack::is_dragging)
                        .cursor(CursorIcon::NsResize)
                        .on_press_down(|cx| cx.emit(ResizableStackEvent::StartDrag));
                }

                (content)(cx);
            });

        if direction == ResizeStackDirection::Right {
            handle.width(size)
        } else {
            handle.height(size)
        }
    }
}

pub enum ResizableStackEvent {
    StartDrag,
    StopDrag,
}

impl View for ResizableStack {
    fn element(&self) -> Option<&'static str> {
        Some("resizable-stack")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|resizable_stack_event, event| match resizable_stack_event {
            ResizableStackEvent::StartDrag => {
                self.is_dragging = true;
                cx.capture();
                cx.lock_cursor_icon();

                // Disable pointer events for everything while dragging
                cx.with_current(Entity::root(), |cx| {
                    cx.set_pointer_events(false);
                });

                // Prevent propagation in case the resizable stack is within another resizable stack
                event.consume();
            }

            ResizableStackEvent::StopDrag => {
                self.is_dragging = false;
                cx.release();
                cx.unlock_cursor_icon();

                // Re-enable pointer events
                cx.with_current(Entity::root(), |cx| {
                    cx.set_pointer_events(true);
                });

                event.consume()
            }
        });

        event.map(|window_event, _| match window_event {
            WindowEvent::MouseMove(x, y) => {
                let dpi = cx.scale_factor();
                if self.is_dragging {
                    let new_size = if self.direction == ResizeStackDirection::Right {
                        let posx = cx.bounds().x;
                        (*x - posx) / dpi
                    } else {
                        let posy = cx.bounds().y;
                        (*y - posy) / dpi
                    };

                    if new_size.is_finite() && new_size > 5.0 {
                        (self.on_drag)(cx, new_size);
                    }
                }
            }

            WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                cx.emit(ResizableStackEvent::StopDrag);
            }

            _ => {}
        });
    }
}
