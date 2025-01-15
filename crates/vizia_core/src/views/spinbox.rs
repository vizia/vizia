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
#[derive(Lens)]
pub struct Spinbox {
    orientation: Orientation,
    icons: SpinboxIcons,

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

impl_res_simple!(SpinboxIcons);

impl Spinbox {
    /// Creates a new [Spinbox] view.
    pub fn new<L>(cx: &mut Context, lens: L) -> Handle<Spinbox>
    where
        L: Lens<Target: Data + ToStringLocalized>,
    {
        Self::custom(cx, move |cx| Label::new(cx, lens))
    }

    /// Creates a custom [Spinbox] view with the given content to represent the value.
    pub fn custom<F, V>(cx: &mut Context, content: F) -> Handle<Spinbox>
    where
        F: Fn(&mut Context) -> Handle<V>,
        V: 'static + View,
    {
        Self {
            orientation: Orientation::Horizontal,
            icons: SpinboxIcons::Chevrons,
            on_decrement: None,
            on_increment: None,
        }
        .build(cx, move |cx| {
            Binding::new(cx, Spinbox::orientation, move |cx, spinbox_kind| {
                match spinbox_kind.get(cx) {
                    Orientation::Horizontal => {
                        Button::new(cx, |cx| {
                            Svg::new(
                                cx,
                                Spinbox::icons.map(|icons| match icons {
                                    SpinboxIcons::PlusMinus => ICON_MINUS,
                                    SpinboxIcons::Chevrons => ICON_CHEVRON_LEFT,
                                }),
                            )
                        })
                        .on_press(|ex| ex.emit(SpinboxEvent::Decrement))
                        .navigable(true)
                        .class("spinbox-button");
                    }

                    Orientation::Vertical => {
                        Button::new(cx, |cx| {
                            Svg::new(
                                cx,
                                Spinbox::icons.map(|icons| match icons {
                                    SpinboxIcons::PlusMinus => ICON_PLUS,
                                    SpinboxIcons::Chevrons => ICON_CHEVRON_UP,
                                }),
                            )
                        })
                        .on_press(|ex| ex.emit(SpinboxEvent::Increment))
                        .navigable(true)
                        .class("spinbox-button");
                    }
                }
            });
            (content)(cx).class("spinbox-value");
            Binding::new(cx, Spinbox::orientation, move |cx, spinbox_kind| {
                match spinbox_kind.get(cx) {
                    Orientation::Horizontal => {
                        Button::new(cx, |cx| {
                            Svg::new(
                                cx,
                                Spinbox::icons.map(|icons| match icons {
                                    SpinboxIcons::PlusMinus => ICON_PLUS,
                                    SpinboxIcons::Chevrons => ICON_CHEVRON_RIGHT,
                                }),
                            )
                        })
                        .on_press(|ex| ex.emit(SpinboxEvent::Increment))
                        .navigable(true)
                        .class("spinbox-button");
                    }

                    Orientation::Vertical => {
                        Button::new(cx, |cx| {
                            Svg::new(
                                cx,
                                Spinbox::icons.map(|icons| match icons {
                                    SpinboxIcons::PlusMinus => ICON_MINUS,
                                    SpinboxIcons::Chevrons => ICON_CHEVRON_DOWN,
                                }),
                            )
                        })
                        .on_press(|ex| ex.emit(SpinboxEvent::Decrement))
                        .navigable(true)
                        .class("spinbox-button");
                    }
                }
            });
        })
        .toggle_class("horizontal", Spinbox::orientation.map(|o| o == &Orientation::Horizontal))
        .toggle_class("vertical", Spinbox::orientation.map(|o| o == &Orientation::Vertical))
        .navigable(true)
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
    pub fn orientation(self, orientation: impl Res<Orientation>) -> Self {
        self.bind(orientation, move |handle, orientation| {
            let orientation = orientation.get(&handle);
            handle.modify(move |spinbox| spinbox.orientation = orientation);
        })
    }

    /// Set the icons which should be used for the increment and decrement buttons of the [Spinbox]
    pub fn icons(self, icons: impl Res<SpinboxIcons>) -> Self {
        self.bind(icons, move |handle, icons| {
            let icons = icons.get(&handle);
            handle.modify(move |spinbox| spinbox.icons = icons);
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
