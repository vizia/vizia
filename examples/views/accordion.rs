use vizia::prelude::*;
mod helpers;
use helpers::*;

pub struct AppData {
    allow_multiple_open: Signal<bool>,
    open_indices: Signal<Vec<usize>>,
}

pub enum AppEvent {
    ToggleMultipleOpen,
    ToggleSection(usize),
    SectionToggled(usize, bool),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleMultipleOpen => {
                self.allow_multiple_open.update(|multi| *multi ^= true);
                if !self.allow_multiple_open.get() {
                    self.open_indices.update(|indices| {
                        if let Some(first_open) = indices.first().copied() {
                            indices.clear();
                            indices.push(first_open);
                        }
                    });
                }
            }
            AppEvent::ToggleSection(index) => {
                self.open_indices.update(|indices| {
                    if self.allow_multiple_open.get() {
                        if indices.contains(index) {
                            indices.retain(|&i| i != *index);
                        } else {
                            indices.push(*index);
                        }
                    } else {
                        if indices.contains(index) {
                            indices.clear();
                        } else {
                            indices.clear();
                            indices.push(*index);
                        }
                    }
                });
            }
            AppEvent::SectionToggled(index, is_open) => {
                self.open_indices.update(|indices| {
                    if *is_open {
                        if self.allow_multiple_open.get() {
                            if !indices.contains(index) {
                                indices.push(*index);
                            }
                        } else {
                            indices.clear();
                            indices.push(*index);
                        }
                    } else {
                        indices.retain(|&i| i != *index);
                    }
                });
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let allow_multiple_open = Signal::new(false);
        let open_indices = Signal::new(vec![0]);

        AppData { allow_multiple_open, open_indices }.build(cx);

        let items = Signal::new(vec![
            (0, "accordion-title-1", "accordion-content-1"),
            (1, "accordion-title-2", "accordion-content-2"),
            (2, "accordion-title-3", "accordion-content-3"),
        ]);

        ExamplePage::vertical(cx, |cx| {
            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Switch::new(cx, allow_multiple_open)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleMultipleOpen));
                    Label::new(cx, Localized::new("allow-multiple-open"))
                        .describing("multiple-toggle");
                })
                .alignment(Alignment::Left)
                .gap(Pixels(8.0))
                .size(Auto);

                Button::new(cx, |cx| Label::new(cx, Localized::new("toggle-section")))
                    .on_press(|cx| cx.emit(AppEvent::ToggleSection(1)));
            })
            .size(Auto)
            .gap(Pixels(16.0))
            .alignment(Alignment::Center);

            Accordion::new(cx, items, |_cx, _index, item| {
                let title_key = item.1;
                let content_key = item.2;

                AccordionPair::new(
                    move |cx| {
                        Label::new(cx, Localized::new(title_key))
                            .width(Stretch(1.0))
                            .text_wrap(true)
                            .hoverable(false);
                    },
                    move |cx| {
                        Label::new(cx, Localized::new(content_key))
                            .width(Stretch(1.0))
                            .text_wrap(true)
                            .hoverable(false);
                    },
                )
            })
            .open(open_indices)
            .on_toggle(|cx, index, is_open| {
                cx.emit(AppEvent::SectionToggled(index, is_open));
            })
            .width(Stretch(1.0));
        });
    })
    .title("Accordion")
    .run()
}
