use crate::prelude::*;

/// Enum representing semantic size presets for common controls.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ControlSize {
    /// Extra small control.
    ExtraSmall,
    /// Small control.
    Small,
    #[default]
    /// Medium control.
    Medium,
    /// Large control.
    Large,
}

impl_res_simple!(ControlSize);

/// Modifiers for common control-level behavior that can be shared across views.
pub trait ControlModifiers: Sized {
    /// Applies semantic sizing classes to a control.
    fn control_size<U: Into<ControlSize> + Clone + 'static>(
        self,
        size: impl Res<U> + 'static,
    ) -> Self;
}

pub(crate) fn bind_control_size<V: View, U: Into<ControlSize> + Clone + 'static>(
    handle: Handle<'_, V>,
    size: impl Res<U> + 'static,
) -> Handle<'_, V> {
    let size = size.to_signal(handle.cx).map(|value| value.clone().into());
    handle.bind(size, move |handle| {
        match size.get() {
            ControlSize::ExtraSmall => {
                handle
                    .toggle_class("xsmall", true)
                    .toggle_class("small", false)
                    .toggle_class("medium", false)
                    .toggle_class("large", false);
            }
            ControlSize::Small => {
                handle
                    .toggle_class("small", true)
                    .toggle_class("xsmall", false)
                    .toggle_class("medium", false)
                    .toggle_class("large", false);
            }
            ControlSize::Medium => {
                handle
                    .toggle_class("medium", true)
                    .toggle_class("xsmall", false)
                    .toggle_class("small", false)
                    .toggle_class("large", false);
            }
            ControlSize::Large => {
                handle
                    .toggle_class("large", true)
                    .toggle_class("xsmall", false)
                    .toggle_class("small", false)
                    .toggle_class("medium", false);
            }
        };
    })
}
