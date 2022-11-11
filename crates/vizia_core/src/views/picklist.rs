use crate::fonts::icons_names::DOWN;
use crate::prelude::*;

pub struct PickList {
    on_select: Option<Box<dyn Fn(&mut EventContext, usize)>>,
}

pub enum PickListEvent {
    SetOption(usize),
}

impl PickList {
    pub fn new<L1, L2, T>(cx: &mut Context, list_lens: L1, selected: L2) -> Handle<Self>
    where
        L1: Lens<Target = Vec<T>>,
        <L1 as Lens>::Source: Model,
        T: 'static + Data + ToString,
        L2: Lens<Target = usize>,
    {
        Self { on_select: None }
            .build(cx, |cx| {
                let lens2 = list_lens.clone();
                let select1 = selected.clone();
                let select2 = selected.clone();
                // Dropdown List
                Dropdown::new(
                    cx,
                    move |cx| {
                        // A Label and an Icon
                        let lens1 = list_lens.clone();
                        let select1 = select1.clone();
                        HStack::new(cx, move |cx| {
                            // Label::new(cx, lens.clone().index(selected));
                            Label::new(cx, "").bind(lens1.clone(), move |handle, list| {
                                handle.bind(select1.clone(), move |handle, selected| {
                                    let selected_index = selected.get(handle.cx);

                                    handle.text(list.clone().index(selected_index));
                                });
                            });
                            Label::new(cx, DOWN).font("icons");
                        })
                        .child_left(Pixels(5.0))
                        .child_right(Pixels(5.0))
                        .col_between(Stretch(1.0))
                    },
                    move |cx| {
                        //let select = selected.clone();
                        //let select = select.clone();
                        let select2 = select2.clone();
                        List::new(cx, lens2.clone(), move |cx, index, item| {
                            Label::new(cx, item)
                                .width(Pixels(100.0))
                                .child_top(Stretch(1.0))
                                .child_bottom(Stretch(1.0))
                                .border_radius(Units::Pixels(4.0))
                                .checked(select2.clone().map(move |selected| *selected == index))
                                .on_press(move |cx| {
                                    cx.emit(PickListEvent::SetOption(index));
                                    cx.emit(PopupEvent::Close);
                                });
                        })
                        .width(Pixels(100.0));
                    },
                )
                .width(Pixels(100.0));
            })
            .size(Stretch(1.0))
    }
}

impl View for PickList {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|picklist_event, _| match picklist_event {
            PickListEvent::SetOption(index) => {
                if let Some(callback) = &self.on_select {
                    (callback)(cx, *index);
                }
            }
        });
    }
}

impl<'v> Handle<'v, PickList> {
    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize),
    {
        self.modify(|picklist: &mut PickList| picklist.on_select = Some(Box::new(callback)))
    }
}
