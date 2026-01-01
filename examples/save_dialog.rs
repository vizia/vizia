use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

struct SaveDialog {
    is_saved: Signal<bool>,
    show_dialog: Signal<bool>,
}

impl View for SaveDialog {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| {
            // Intercept WindowClose event to show a dialog if not 'saved'.
            if let WindowEvent::WindowClose = window_event {
                if !*self.is_saved.get(cx) {
                    self.show_dialog.set(cx, true);
                    meta.consume();
                }
            }
        });
    }
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let main_window = cx.parent_window();
        let is_saved = cx.state(false);
        let show_dialog = cx.state(false);
        let gap_10 = cx.state(Pixels(10.0));
        let padding_50 = cx.state(Pixels(50.0));
        let align_top_center = cx.state(Alignment::TopCenter);
        let stretch_one = cx.state(Stretch(1.0));
        let align_center = cx.state(Alignment::Center);
        let button_width = cx.state(Pixels(120.0));
        let auto = cx.state(Auto);
        let gap_20 = cx.state(Pixels(20.0));
        let position_absolute = cx.state(PositionType::Absolute);
        let backdrop_blur = cx.state(Filter::Blur(Pixels(2.0).into()));

        SaveDialog { is_saved, show_dialog }.build(cx, |cx| {
            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::static_text(cx, "Close"))
                    .on_press(|cx| cx.emit(WindowEvent::WindowClose));
                Button::new(cx, |cx| Label::static_text(cx, "Save"))
                    .on_press(move |cx| is_saved.set(cx, true));
            })
            .gap(gap_10)
            .padding(padding_50)
            .alignment(align_top_center);

            Binding::new(cx, show_dialog, move |cx| {
                if *show_dialog.get(cx) {
                    let popup_title = cx.state("Save work?");
                    let popup_size = cx.state((400, 100));
                    let popup_anchor = cx.state(Anchor::Center);
                    Window::popup(cx, true, move |cx| {
                        VStack::new(cx, move |cx| {
                            Label::static_text(cx, "Save before close?")
                                .width(stretch_one)
                                .alignment(align_center);
                            HStack::new(cx, move |cx| {
                                Button::new(cx, |cx| Label::static_text(cx, "Save & Close"))
                                    .on_press(move |cx| {
                                        is_saved.set(cx, true);
                                        show_dialog.set(cx, false);
                                        cx.emit_to(main_window, WindowEvent::WindowClose);
                                    })
                                    .width(button_width)
                                    .class("accent");

                                Button::new(cx, |cx| Label::static_text(cx, "Cancel"))
                                    .on_press(move |cx| show_dialog.set(cx, false))
                                    .width(button_width);
                            })
                            .horizontal_gap(gap_10)
                            .size(auto);
                        })
                        .alignment(align_center)
                        .vertical_gap(gap_20);
                    })
                    .on_close(move |cx| show_dialog.set(cx, false))
                    .title(popup_title)
                    .inner_size(popup_size)
                    .anchor(popup_anchor);
                }
            });

            Element::new(cx)
                .size(stretch_one)
                .position_type(position_absolute)
                .backdrop_filter(backdrop_blur)
                .display(show_dialog);
        });
    })
    .run()
}
