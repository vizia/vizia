use crate::prelude::*;

/// A textbox specialized for numeric input with optional min/max validation.
///
/// NumberInput wraps a Textbox and provides convenient methods for numeric range
/// validation and two-way binding.
///
/// # Examples
///
/// ## Basic NumberInput with Two-Way Binding
/// ```
/// # use vizia_core::prelude::*;
/// # let mut cx = &mut Context::default();
/// let number = cx.state(5i32);
/// NumberInput::new(cx, number).two_way();
/// ```
///
/// ## NumberInput with Range Validation
/// ```
/// # use vizia_core::prelude::*;
/// # let mut cx = &mut Context::default();
/// let number = cx.state(50i32);
/// NumberInput::new(cx, number)
///     .max(100)
///     .two_way();
/// ```
pub struct NumberInput<T: 'static> {
    value: Signal<T>,
    min: Option<T>,
    max: Option<T>,
}

impl<T> NumberInput<T>
where
    T: 'static + Clone + ToStringLocalized + std::str::FromStr + PartialOrd + Send + Sync,
{
    /// Creates a new number input bound to the given signal.
    pub fn new(cx: &mut Context, value: Signal<T>) -> Handle<Self> {
        Self { value, min: None, max: None }
            .build(cx, move |cx| {
                Textbox::new(cx, value);
            })
            .class("number-input")
    }
}

impl<T> View for NumberInput<T>
where
    T: 'static + Clone + ToStringLocalized + std::str::FromStr + PartialOrd + Send + Sync,
{
    fn element(&self) -> Option<&'static str> {
        Some("number-input")
    }
}

impl<T> Handle<'_, NumberInput<T>>
where
    T: 'static + Clone + ToStringLocalized + std::str::FromStr + PartialOrd + Send + Sync,
{
    /// Sets the minimum allowed value.
    ///
    /// Values below this will cause the textbox to show as invalid.
    pub fn min(self, min: T) -> Self {
        self.modify(|input| input.min = Some(min)).rebuild_validation()
    }

    /// Sets the maximum allowed value.
    ///
    /// Values above this will cause the textbox to show as invalid.
    pub fn max(self, max: T) -> Self {
        self.modify(|input| input.max = Some(max)).rebuild_validation()
    }

    /// Sets both minimum and maximum allowed values.
    pub fn range(self, min: T, max: T) -> Self {
        self.modify(|input| {
            input.min = Some(min);
            input.max = Some(max);
        })
        .rebuild_validation()
    }

    /// Enables two-way binding: submitted values automatically update the bound signal.
    pub fn two_way(self) -> Self {
        let entity = self.entity();
        self.modify2(|input, cx| {
            let signal = input.value;
            if let Some(textbox_entity) = cx.tree.get_first_child(entity) {
                if let Some(view) = cx.views.get_mut(&textbox_entity) {
                    if let Some(textbox) = view.downcast_mut::<Textbox<T>>() {
                        textbox.on_submit = Some(Box::new(move |cx, val, _| {
                            signal.set(cx, val);
                        }));
                    }
                }
            }
        })
    }

    /// Sets a custom validation function (in addition to min/max).
    pub fn validate<F: 'static + Fn(&T) -> bool + Clone + Send + Sync>(self, validate: F) -> Self {
        let entity = self.entity();
        self.modify2(move |input, cx| {
            let min = input.min.clone();
            let max = input.max.clone();
            let validate = validate.clone();

            if let Some(textbox_entity) = cx.tree.get_first_child(entity) {
                if let Some(view) = cx.views.get_mut(&textbox_entity) {
                    if let Some(textbox) = view.downcast_mut::<Textbox<T>>() {
                        textbox.validate = Some(Box::new(move |val| {
                            let min_ok = min.as_ref().is_none_or(|m| val >= m);
                            let max_ok = max.as_ref().is_none_or(|m| val <= m);
                            min_ok && max_ok && validate(val)
                        }));
                    }
                }
            }
        })
    }

    // Internal: rebuild the validation closure with current min/max
    fn rebuild_validation(self) -> Self {
        let entity = self.entity();
        self.modify2(|input, cx| {
            let min = input.min.clone();
            let max = input.max.clone();

            if let Some(textbox_entity) = cx.tree.get_first_child(entity) {
                if let Some(view) = cx.views.get_mut(&textbox_entity) {
                    if let Some(textbox) = view.downcast_mut::<Textbox<T>>() {
                        textbox.validate = Some(Box::new(move |val| {
                            let min_ok = min.as_ref().is_none_or(|m| val >= m);
                            let max_ok = max.as_ref().is_none_or(|m| val <= m);
                            min_ok && max_ok
                        }));
                    }
                }
            }
        })
    }
}
