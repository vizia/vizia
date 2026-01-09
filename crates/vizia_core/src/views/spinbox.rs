use crate::icons::{
    ICON_CHEVRON_DOWN, ICON_CHEVRON_LEFT, ICON_CHEVRON_RIGHT, ICON_CHEVRON_UP, ICON_MINUS,
    ICON_PLUS,
};
use crate::prelude::*;

pub(crate) enum SpinboxEvent {
    Increment,
    Decrement,
}

/// A view which represents a value which can be incremented or decremented.
pub struct Spinbox {
    orientation: Signal<Orientation>,
    icons: Signal<SpinboxIcons>,

    on_decrement: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
    on_increment: Option<Box<dyn Fn(&mut EventContext) + Send + Sync>>,
}

/// And enum which represents the icons that can be used for the increment and decrement buttons of the [Spinbox].
#[derive(Clone, Copy, Debug, PartialEq, Data)]
pub enum SpinboxIcons {
    /// A plus icon for the increment button and a minus icon for the decrement button.
    PlusMinus,
    /// A right chevron for the increment button and a left chevron for the decrement button.
    Chevrons,
}

crate::impl_res_simple!(SpinboxIcons);

impl Spinbox {
    /// Creates a new [Spinbox] view.
    ///
    /// Accepts either a plain value or a `Signal<T>` for reactive state.
    pub fn new<T>(cx: &mut Context, value: impl Res<T> + Clone + 'static) -> Handle<Spinbox>
    where
        T: ToStringLocalized + Clone + 'static,
    {
        Self::custom(cx, move |cx| Label::new(cx, value.clone()))
    }

    /// Creates a custom [Spinbox] view with the given content to represent the value.
    pub fn custom<F, V>(cx: &mut Context, content: F) -> Handle<Spinbox>
    where
        F: Fn(&mut Context) -> Handle<V>,
        V: 'static + View,
    {
        let orientation = cx.state(Orientation::Horizontal);
        let icons = cx.state(SpinboxIcons::Chevrons);

        let is_horizontal = cx.derived({
            let orientation = orientation;
            move |store| *orientation.get(store) == Orientation::Horizontal
        });
        let is_vertical = cx.derived({
            let orientation = orientation;
            move |store| *orientation.get(store) == Orientation::Vertical
        });
        let navigable = cx.state(true);

        Self { orientation, icons, on_decrement: None, on_increment: None }
            .build(cx, move |cx| {
                // First button (decrement for horizontal, increment for vertical)
                Button::new(cx, move |cx| {
                    // Create derived signal for the icon based on both orientation and icons
                    let icon_signal = cx.derived(move |s| {
                        let orient = *orientation.get(s);
                        let ico = *icons.get(s);
                        match orient {
                            Orientation::Horizontal => match ico {
                                SpinboxIcons::PlusMinus => ICON_MINUS,
                                SpinboxIcons::Chevrons => ICON_CHEVRON_LEFT,
                            },
                            Orientation::Vertical => match ico {
                                SpinboxIcons::PlusMinus => ICON_PLUS,
                                SpinboxIcons::Chevrons => ICON_CHEVRON_UP,
                            },
                        }
                    });
                    Svg::new(cx, icon_signal)
                })
                .on_press(move |ex| {
                    let orient = *orientation.get(ex);
                    match orient {
                        Orientation::Horizontal => ex.emit(SpinboxEvent::Decrement),
                        Orientation::Vertical => ex.emit(SpinboxEvent::Increment),
                    }
                })
                .navigable(navigable)
                .class("spinbox-button");

                (content)(cx).class("spinbox-value");

                // Second button (increment for horizontal, decrement for vertical)
                Button::new(cx, move |cx| {
                    let icon_signal = cx.derived(move |s| {
                        let orient = *orientation.get(s);
                        let ico = *icons.get(s);
                        match orient {
                            Orientation::Horizontal => match ico {
                                SpinboxIcons::PlusMinus => ICON_PLUS,
                                SpinboxIcons::Chevrons => ICON_CHEVRON_RIGHT,
                            },
                            Orientation::Vertical => match ico {
                                SpinboxIcons::PlusMinus => ICON_MINUS,
                                SpinboxIcons::Chevrons => ICON_CHEVRON_DOWN,
                            },
                        }
                    });
                    Svg::new(cx, icon_signal)
                })
                .on_press(move |ex| {
                    let orient = *orientation.get(ex);
                    match orient {
                        Orientation::Horizontal => ex.emit(SpinboxEvent::Increment),
                        Orientation::Vertical => ex.emit(SpinboxEvent::Decrement),
                    }
                })
                .navigable(navigable)
                .class("spinbox-button");
            })
            .toggle_class("horizontal", is_horizontal)
            .toggle_class("vertical", is_vertical)
            .navigable(navigable)
    }
}

impl Handle<'_, Spinbox> {
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
    pub fn orientation(self, orientation: Signal<Orientation>) -> Self {
        self.bind(orientation, move |handle, orientation| {
            let orientation = *orientation.get(&handle);
            handle.modify2(move |spinbox, cx| spinbox.orientation.set(cx, orientation));
        })
    }

    /// Set the icons which should be used for the increment and decrement buttons of the [Spinbox]
    pub fn icons(mut self, icons: impl Res<SpinboxIcons> + 'static) -> Self {
        let icons = icons.into_signal(self.context());
        self.bind(icons, move |handle, icons| {
            let icons = *icons.get(&handle);
            handle.modify2(move |spinbox, cx| spinbox.icons.set(cx, icons));
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
                if let Some(callback) = &self.on_increment {
                    (callback)(cx)
                }
            }

            SpinboxEvent::Decrement => {
                if let Some(callback) = &self.on_decrement {
                    (callback)(cx)
                }
            }
        });
    }
}
