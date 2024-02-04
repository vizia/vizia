use vizia_storage::LayoutTreeIterator;

use crate::prelude::*;

pub enum KeyBindEvent {
    Create,
}

#[derive(Lens)]
pub struct KeyBindView {
    entity: Entity,
    bindings: Vec<(String, Vec<(String, KeyChord)>)>,
}

impl KeyBindView {
    pub fn new(cx: &mut Context, entity: Entity) -> Handle<Self> {
        Self { entity, bindings: Vec::new() }
            .build(cx, |cx| {
                List::new(cx, KeyBindView::bindings, |cx, idx, bindings| {
                    List::new(cx, bindings.map_ref(|b| &b.1), |cx, index, item| {
                        let item = item.get(cx);

                        Label::new(cx, item.0).right(Stretch(1.0));
                        if item.1.modifiers.contains(Modifiers::LOGO) {
                            Label::new(cx, "Logo");
                            Label::new(cx, "+");
                        }
                        if item.1.modifiers.contains(Modifiers::CTRL) {
                            Label::new(cx, "Ctrl");
                            Label::new(cx, "+");
                        }
                        if item.1.modifiers.contains(Modifiers::SHIFT) {
                            Label::new(cx, "Shift");
                            Label::new(cx, "+");
                        }
                        if item.1.modifiers.contains(Modifiers::ALT) {
                            Label::new(cx, "Alt");
                            Label::new(cx, "+");
                        }

                        let s = match item.1.code {
                            Code::ArrowDown => "ArrowDown",
                            Code::ArrowUp => "ArrowUp",
                            Code::Escape => "Esc",
                            _ => "?",
                        };
                        Label::new(cx, s);
                    });
                });
            })
            .child_space(Pixels(20.0))
            .on_build(|cx| cx.emit(KeyBindEvent::Create))
    }
}

impl View for KeyBindView {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|keybind_event, _| match keybind_event {
            KeyBindEvent::Create => {
                let iter = LayoutTreeIterator::subtree(cx.tree, self.entity);
                for descendant in iter {
                    cx.with_current(descendant, |cx| {
                        if let Some(keymap) = cx.get_model::<Keymap<&'static str>>() {
                            self.bindings.push((
                                keymap.name().unwrap_or_default().to_owned(),
                                keymap
                                    .export()
                                    .into_iter()
                                    .map(|(chord, entry)| ((*entry.action()).to_owned(), *chord))
                                    .collect(),
                            ));
                        }
                    })
                }
            }
        })
    }
}
