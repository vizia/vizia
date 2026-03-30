use crate::icons::{
    ICON_CHEVRON_DOWN, ICON_CHEVRON_LEFT, ICON_CHEVRON_RIGHT, ICON_CHEVRON_UP, ICON_MINUS,
    ICON_PLUS,
};
use crate::prelude::*;

pub(crate) enum SpinboxEvent {
    Increment,
    Decrement,
    SetMin,
    SetMax,
}

/// A view which represents a value which can be incremented or decremented.
pub struct Spinbox {
    value: Signal<f64>,
    orientation: Signal<Orientation>,
    icons: Signal<SpinboxIcons>,
    min: Signal<Option<f64>>,
    max: Signal<Option<f64>>,

    on_change: Option<Box<dyn Fn(&mut EventContext, f64)>>,
    on_decrement: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    on_increment: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
}

/// And enum which represents the icons that can be used for the increment and decrement buttons of the [Spinbox].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpinboxIcons {
    /// A plus icon for the increment button and a minus icon for the decrement button.
    PlusMinus,
    /// A right chevron for the increment button and a left chevron for the decrement button.
    Chevrons,
}

impl_res_simple!(SpinboxIcons);

impl Spinbox {
    /// Creates a new [Spinbox] view.
    pub fn new<S, T>(cx: &mut Context, value: S) -> Handle<Spinbox>
    where
        S: Copy + SignalGet<T> + SignalMap<T> + Res<T> + 'static,
        T: Clone + Into<f64> + 'static,
    {
        let numeric_value = value.map(|v| v.clone().into()).to_signal(cx);

        let orientation = Signal::new(Orientation::Horizontal);
        let icons = Signal::new(SpinboxIcons::Chevrons);
        let min = Signal::new(None::<f64>);
        let max = Signal::new(None::<f64>);

        Self {
            value: numeric_value,
            orientation,
            icons,
            min,
            max,
            on_change: None,
            on_decrement: None,
            on_increment: None,
        }
            .build(cx, move |cx| {
                Keymap::from(vec![
                    (
                        KeyChord::new(Modifiers::empty(), Code::ArrowUp),
                        KeymapEntry::new("Increment", |cx| cx.emit(SpinboxEvent::Increment)),
                    ),
                    (
                        KeyChord::new(Modifiers::empty(), Code::ArrowRight),
                        KeymapEntry::new("Increment", |cx| cx.emit(SpinboxEvent::Increment)),
                    ),
                    (
                        KeyChord::new(Modifiers::empty(), Code::ArrowDown),
                        KeymapEntry::new("Decrement", |cx| cx.emit(SpinboxEvent::Decrement)),
                    ),
                    (
                        KeyChord::new(Modifiers::empty(), Code::ArrowLeft),
                        KeymapEntry::new("Decrement", |cx| cx.emit(SpinboxEvent::Decrement)),
                    ),
                    (
                        KeyChord::new(Modifiers::empty(), Code::Home),
                        KeymapEntry::new("Set Min", |cx| cx.emit(SpinboxEvent::SetMin)),
                    ),
                    (
                        KeyChord::new(Modifiers::empty(), Code::End),
                        KeymapEntry::new("Set Max", |cx| cx.emit(SpinboxEvent::SetMax)),
                    ),
                ])
                .build(cx);

                let at_min = Memo::new(move |_| {
                    matches!((min.get(), numeric_value.get()), (Some(min), value) if value <= min)
                });
                let at_max = Memo::new(move |_| {
                    matches!((max.get(), numeric_value.get()), (Some(max), value) if value >= max)
                });

                Binding::new(cx, orientation, move |cx| match orientation.get() {
                    Orientation::Horizontal => {
                        Button::new(cx, |cx| {
                            Svg::new(
                                cx,
                                icons.map(|icons| match icons {
                                    SpinboxIcons::PlusMinus => ICON_MINUS,
                                    SpinboxIcons::Chevrons => ICON_CHEVRON_LEFT,
                                }),
                            )
                        })
                        .on_press(|ex| ex.emit(SpinboxEvent::Decrement))
                        .disabled(at_min)
                        .navigable(false)
                        .class("spinbox-button");
                    }

                    Orientation::Vertical => {
                        Button::new(cx, |cx| {
                            Svg::new(
                                cx,
                                icons.map(|icons| match icons {
                                    SpinboxIcons::PlusMinus => ICON_PLUS,
                                    SpinboxIcons::Chevrons => ICON_CHEVRON_UP,
                                }),
                            )
                        })
                        .on_press(|ex| ex.emit(SpinboxEvent::Increment))
                        .disabled(at_max)
                        .navigable(false)
                        .class("spinbox-button");
                    }
                });
                Textbox::new(cx, numeric_value).class("spinbox-value").role(Role::SpinButton);
                Binding::new(cx, orientation, move |cx| match orientation.get() {
                    Orientation::Horizontal => {
                        Button::new(cx, |cx| {
                            Svg::new(
                                cx,
                                icons.map(|icons| match icons {
                                    SpinboxIcons::PlusMinus => ICON_PLUS,
                                    SpinboxIcons::Chevrons => ICON_CHEVRON_RIGHT,
                                }),
                            )
                        })
                        .on_press(|ex| ex.emit(SpinboxEvent::Increment))
                        .disabled(at_max)
                        .navigable(false)
                        .class("spinbox-button");
                    }

                    Orientation::Vertical => {
                        Button::new(cx, |cx| {
                            Svg::new(
                                cx,
                                icons.map(|icons| match icons {
                                    SpinboxIcons::PlusMinus => ICON_MINUS,
                                    SpinboxIcons::Chevrons => ICON_CHEVRON_DOWN,
                                }),
                            )
                        })
                        .on_press(|ex| ex.emit(SpinboxEvent::Decrement))
                        .disabled(at_min)
                        .navigable(false)
                        .class("spinbox-button");
                    }
                });
            })
            .toggle_class("horizontal", orientation.map(|o| o == &Orientation::Horizontal))
            .toggle_class("vertical", orientation.map(|o| o == &Orientation::Vertical))
            .navigable(false)
    }

    fn clamp_value(&self, value: f64) -> f64 {
        let value = if let Some(min) = self.min.get() { value.max(min) } else { value };
        if let Some(max) = self.max.get() { value.min(max) } else { value }
    }

    fn emit_change(&self, cx: &mut EventContext, value: f64) {
        if let Some(callback) = &self.on_change {
            (callback)(cx, self.clamp_value(value));
        }
    }
}

impl Handle<'_, Spinbox> {
    /// Sets the callback triggered when the spinbox value is changed.
    pub fn on_change<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, f64),
    {
        self.modify(|spinbox| spinbox.on_change = Some(Box::new(callback)))
    }

    /// Sets the callback which is triggered when the [Spinbox] value is incremented.
    pub fn on_increment<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        self.modify(|spinbox: &mut Spinbox| spinbox.on_increment = Some(Box::new(callback)))
    }

    /// Sets the callback which is triggered when the [Spinbox] value is decremented.
    pub fn on_decrement<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext) + Send + Sync,
    {
        self.modify(|spinbox: &mut Spinbox| spinbox.on_decrement = Some(Box::new(callback)))
    }

    /// Sets the orientation of the [Spinbox].
    pub fn orientation(self, orientation: impl Res<Orientation> + 'static) -> Self {
        let orientation = orientation.to_signal(self.cx);
        self.bind(orientation, move |handle| {
            let orientation = orientation.get();
            handle.modify(move |spinbox| spinbox.orientation.set(orientation));
        })
    }

    /// Set the icons which should be used for the increment and decrement buttons of the [Spinbox]
    pub fn icons(self, icons: impl Res<SpinboxIcons> + 'static) -> Self {
        let icons = icons.to_signal(self.cx);
        self.bind(icons, move |handle| {
            let icons = icons.get();
            handle.modify(move |spinbox| spinbox.icons.set(icons));
        })
    }

    /// Sets the minimum value of the [Spinbox], disabling the decrement button when reached.
    pub fn min<U: Into<f64> + Clone + 'static>(self, min: impl Res<U> + 'static) -> Self {
        let min_signal = min.to_signal(self.cx);
        self.bind(min_signal, move |handle| {
            let val: f64 = min_signal.get().into();
            handle.modify(move |spinbox| spinbox.min.set(Some(val)));
        })
    }

    /// Sets the maximum value of the [Spinbox], disabling the increment button when reached.
    pub fn max<U: Into<f64> + Clone + 'static>(self, max: impl Res<U> + 'static) -> Self {
        let max_signal = max.to_signal(self.cx);
        self.bind(max_signal, move |handle| {
            let val: f64 = max_signal.get().into();
            handle.modify(move |spinbox| spinbox.max.set(Some(val)));
        })
    }
}

impl View for Spinbox {
    fn element(&self) -> Option<&'static str> {
        Some("spinbox")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|spinbox_event, _| match spinbox_event {
            SpinboxEvent::Increment => {
                if self.on_change.is_some() {
                    self.emit_change(cx, self.value.get() + 1.0);
                }

                if let Some(callback) = &self.on_increment {
                    (callback)(cx)
                }
            }

            SpinboxEvent::Decrement => {
                if self.on_change.is_some() {
                    self.emit_change(cx, self.value.get() - 1.0);
                }

                if let Some(callback) = &self.on_decrement {
                    (callback)(cx)
                }
            }

            SpinboxEvent::SetMin => {
                if let Some(min) = self.min.get() {
                    self.emit_change(cx, min);
                }
            }

            SpinboxEvent::SetMax => {
                if let Some(max) = self.max.get() {
                    self.emit_change(cx, max);
                }
            }
        });
    }
}
