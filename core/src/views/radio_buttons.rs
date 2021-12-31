use std::collections::HashMap;
use crate::{Context, Entity, Event, Handle, View, WindowEvent, MouseButton, PseudoClass, Label, LayoutType, HStack, VStack, TreeExt};
use crate::style::PropSet;
use crate::Units::*;

pub struct RadioButtons {
    pub idx: usize,
    on_changed: Option<Box<dyn Fn(&mut Context, usize)>>,
    entity_map: HashMap<Entity, usize>,
    check_boxes: Vec<Entity>,
    icon_checked: String,
    icon_unchecked: String,
}

const ICON_CHECK: &str = "\u{2713}";

impl RadioButtons {
    pub fn with_icons<F>(
        cx: &mut Context,
        layout: LayoutType,
        idx: usize,
        idx_count: usize,
        builder: F,
        icon_checked: String,
        icon_unchecked: String,
    ) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context, usize) + Clone
    {
        let result = Self {
            idx,
            on_changed: None,
            icon_checked, icon_unchecked,
            entity_map: HashMap::new(),
            check_boxes: vec![],
        }
            .build2(cx, move |cx| {
                let top_entity = cx.current;
                let stack_builder = move |cx: &mut Context| {
                    for available_idx in 0..idx_count {
                        let one_builder = builder.clone();
                        HStack::new(cx, move |cx| {
                            let hstack_id = cx.current;
                            let view = downcast_view_mut(cx, top_entity).unwrap();
                            let lbl_txt = if available_idx == idx { &view.icon_checked } else { &view.icon_unchecked }.clone();
                            let checkbox = Label::new(cx, &lbl_txt)
                                .class("radioknob");
                            downcast_view_mut(cx, top_entity).unwrap().check_boxes.push(checkbox.entity);

                            (one_builder)(cx, available_idx);

                            let children = hstack_id.branch_iter(&cx.tree).collect::<Vec<_>>();
                            let view = downcast_view_mut(cx, top_entity).unwrap();
                            for child in children {
                                view.entity_map.insert(child, available_idx);
                            }
                        });
                    }
                };
                match layout {
                    LayoutType::Row => { HStack::new(cx, stack_builder); }
                    LayoutType::Column => { VStack::new(cx, stack_builder); }
                    LayoutType::Grid => panic!("Why do you want a grid RadioButtons? go away"),
                };
            })
            .height(Pixels(20.0));
        result
    }

    pub fn new<F>(
        cx: &mut Context,
        layout: LayoutType,
        idx: usize,
        idx_count: usize,
        builder: F
    ) -> Handle<Self>
        where
            F: 'static + Fn(&mut Context, usize) + Clone
    {
        Self::with_icons(cx, layout, idx, idx_count, builder, ICON_CHECK.to_owned(), "".to_owned())
    }
}

impl Handle<RadioButtons> {
    pub fn on_changed<F>(self, cx: &mut Context, callback: F) -> Self
        where
            F: 'static + Fn(&mut Context, usize),
    {
        downcast_view_mut(cx, self.entity).unwrap().on_changed = Some(Box::new(callback));
        self
    }

}

pub fn downcast_view_mut(cx: &mut Context, entity: Entity) -> Option<&mut RadioButtons>
{
    if let Some(view) = cx.views.get_mut(&entity) {
        view.downcast_mut::<RadioButtons>()
    } else {
        None
    }
}

impl View for RadioButtons {
    fn element(&self) -> Option<String> {
        Some("radiobuttons".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if let Some(idx) = self.entity_map.get(&event.target) {
                        if *idx != self.idx {
                            let old_check = self.check_boxes[self.idx];
                            if let Some(pseudo_classes) = cx.style.borrow_mut().pseudo_classes.get_mut(old_check) {
                                pseudo_classes.set(PseudoClass::CHECKED, false);
                            }
                            old_check.set_text(cx, &self.icon_unchecked);

                            self.idx = *idx;
                            if let Some(func) = self.on_changed.take() {
                                func(cx, *idx);
                                self.on_changed = Some(func);
                            }

                            let new_check = self.check_boxes[self.idx];
                            if let Some(pseudo_classes) = cx.style.borrow_mut().pseudo_classes.get_mut(new_check) {
                                pseudo_classes.set(PseudoClass::CHECKED, true);
                            }
                            new_check.set_text(cx, &self.icon_checked);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}