use crate::binding::Data;

pub(crate) mod animatable_set;
pub(crate) mod style_set;

#[derive(Clone)]
pub enum PropValue<T: Default + Clone> {
    Inline(T),
    Shared(T),
    Animating(T),
    Default(T),
}

impl<T: Default + Clone> PropValue<T> {
    pub fn value(&self) -> &T {
        match self {
            PropValue::Inline(t) => t,
            PropValue::Shared(t) => t,
            PropValue::Animating(t) => t,
            PropValue::Default(t) => t,
        }
    }
}

impl<T: Default + Clone + std::fmt::Display> PropValue<T> {
    pub fn prop_str(&self) -> PropValue<String> {
        match self {
            PropValue::Inline(t) => PropValue::Inline(t.to_string()),
            PropValue::Shared(t) => PropValue::Shared(t.to_string()),
            PropValue::Animating(t) => PropValue::Animating(t.to_string()),
            PropValue::Default(t) => PropValue::Default(t.to_string()),
        }
    }
}

impl<T: Default + Clone> Default for PropValue<T> {
    fn default() -> Self {
        PropValue::Default(T::default())
    }
}

impl<T: Default + Clone + Data> Data for PropValue<T> {
    fn same(&self, other: &Self) -> bool {
        self.value().same(other.value())
    }
}

impl<T: Default + Clone + std::fmt::Display> std::fmt::Display for PropValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value().fmt(f)
    }
}
