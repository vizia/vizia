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
struct SaveDialogApp {
    is_saved: Signal<bool>,
    show_dialog: Signal<bool>,
    gap_10: Signal<Units>,
    padding_50: Signal<Units>,
    align_top_center: Signal<Alignment>,
    stretch_one: Signal<Units>,
    align_center: Signal<Alignment>,
    button_width: Signal<Units>,
    auto: Signal<Units>,
    gap_20: Signal<Units>,
    position_absolute: Signal<PositionType>,
    backdrop_blur: Signal<Filter>,
}

#[cfg(not(feature = "baseview"))]
impl App for SaveDialogApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            is_saved: cx.state(false),
            show_dialog: cx.state(false),
            gap_10: cx.state(Pixels(10.0)),
            padding_50: cx.state(Pixels(50.0)),
            align_top_center: cx.state(Alignment::TopCenter),
            stretch_one: cx.state(Stretch(1.0)),
            align_center: cx.state(Alignment::Center),
            button_width: cx.state(Pixels(120.0)),
            auto: cx.state(Auto),
            gap_20: cx.state(Pixels(20.0)),
            position_absolute: cx.state(PositionType::Absolute),
            backdrop_blur: cx.state(Filter::Blur(Pixels(2.0).into())),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let main_window = cx.parent_window();
        let is_saved = self.is_saved;
        let show_dialog = self.show_dialog;
        let gap_10 = self.gap_10;
        let padding_50 = self.padding_50;
        let align_top_center = self.align_top_center;
        let stretch_one = self.stretch_one;
        let align_center = self.align_center;
        let button_width = self.button_width;
        let auto = self.auto;
        let gap_20 = self.gap_20;
        let position_absolute = self.position_absolute;
        let backdrop_blur = self.backdrop_blur;

        SaveDialog { is_saved, show_dialog }.build(cx, |cx| {
            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "Close"))
                    .on_press(|cx| cx.emit(WindowEvent::WindowClose));
                Button::new(cx, |cx| Label::new(cx, "Save"))
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
                            Label::new(cx, "Save before close?")
                                .width(stretch_one)
                                .alignment(align_center);
                            HStack::new(cx, move |cx| {
                                Button::new(cx, |cx| Label::new(cx, "Save & Close"))
                                    .on_press(move |cx| {
                                        is_saved.set(cx, true);
                                        show_dialog.set(cx, false);
                                        cx.emit_to(main_window, WindowEvent::WindowClose);
                                    })
                                    .width(button_width)
                                    .class("accent");

                                Button::new(cx, |cx| Label::new(cx, "Cancel"))
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
        self
    }
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
    SaveDialogApp::run()
}
