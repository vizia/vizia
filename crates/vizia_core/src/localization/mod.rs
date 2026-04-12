//! Provides types for adapting an application to a particular language or regional peculiarities.
//!
//! # Language Translation
//!
//! Vizia provides the ability to dynamically translate text using [fluent](https://projectfluent.org/). Fluent provides a syntax for describing how text should be translated into different languages.
//! The [fluent syntax guide](https://projectfluent.org/fluent/guide/) contains more information on the fluent syntax.
//!
//! ## Adding Fluent Files
//! Before text can be translated, one or more fluent files must be added to an application with the corresponding locale:
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! // Adds a fluent file to the application resource manager.
//! // This file is then used for translations to the corresponding locale.
//! cx.add_translation(
//!     "en-US".parse().unwrap(),
//!     include_str!("resources/en-US/translation.ftl").to_owned(),
//! );
//!
//! ```
//!
//! ## Setting the Locale
//! The application will use the system locale by default, however an environment event can be used to set a custom locale.
//! If no fluent file can be found for the specified locale, then a fallback fluent file is used from the list of available files.
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! // Sets the current locale to en-US, regardless of the system locale
//! cx.emit(EnvironmentEvent::SetLocale("en-US".parse().unwrap()));
//! ```
//!
//! ## Basic Translation
//! Use the [`Localized`] type to specify a translation key to be used with fluent files. The key is then used to look up the corresponding translation.
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! Label::new(cx, Localized::new("hello-world"));
//! ```
//! The markup in the loaded fluent (.ftl) files defines the translations for a particular key. The translation used depends on the application locale, which can be queried from [`Environment`].
//! ```ftl
//! // en-US/hello.ftl
//! hello-world = Hello, world!
//! ```
//! ```ftl
//! // fr/hello.ftl
//! hello-world = Bonjour, monde!
//! ```
//!
//! ## Variables
//! Data from the application can be inserted into translated text using a [placeable](https://projectfluent.org/fluent/guide/variables.html).
//! The variable is enclosed in curly braces and prefixed with a `$` symbol.
//! ```ftl
//! welcome = Welcome, { $user }!
//! ```
//! The [`Localized`] type provides the `arg(...)` method for referencing a variable. It accepts either a plain value
//! or a reactive resource, binding the fluent variable to application data and updating when that data changes.
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! Label::new(cx, Localized::new("welcome").arg("user", "Jane"));
//! ```
//! The same method also accepts any keyed signal or resource.
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! #
//! # pub struct AppData {
//! #   user: String,
//! # }
//! Label::new(cx, Localized::new("welcome").arg("user", AppData::user));
//! ```
//!
//! ## Attributes
//! Messages can have named attributes that provide alternative translations. These are useful for UI elements that need multiple text values.
//! Attributes must be preceded by a main message value:
//! ```ftl
//! file-dialog = File Dialog
//!     .title = Save File
//!     .save-button = Save
//! ```
//! To reference an attribute, use the `.attribute()` method:
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! Label::new(cx, Localized::new("file-dialog").attribute("title"));
//! ```
//!
//! ## Terms
//! Terms are special variables prefixed with a hyphen that can be referenced in other messages. They're automatically
//! available in all translations and are commonly used for product names or branding.
//! ```ftl
//! -brand = Vizia
//! welcome = Welcome to { -brand }!
//! ```
//! Terms are automatically resolved when formatting, so no special configuration is needed when using them in messages.
//!
//! ## Message References
//! Messages can reference other messages to maintain consistency and reduce duplication:
//! ```ftl
//! menu-save = Save
//! help-menu-save = Click { menu-save } to save the file.
//! ```
//! Message references are automatically resolved by the localization system and work seamlessly with the [`Localized`] type.
//! This is useful for keeping certain translations consistent across the interface and making maintenance easier.
//!
//! ## Number Formatting
//! Numbers can be formatted with locale-specific rules using the built-in `NUMBER` function in FTL:
//! ```ftl
//! price = Your total is { NUMBER($amount, style: "currency", currency: "USD") }
//! ```
//! In Rust, pass numbers directly as arguments and they will be formatted according to the locale:
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! Label::new(cx, Localized::new("price").arg("amount", 99.99));
//! ```
//!
//! For more control, use the helper functions to specify decimal places:
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! Label::new(cx, Localized::new("price").arg("amount", number_with_fraction(99.99, 2)));
//! ```
//! Or for percentages:
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! Label::new(cx, Localized::new("completion").arg("percent", percentage(0.75, 1)));
//! ```
//!
//! ## Date Formatting
//! Dates can be formatted with locale-specific rules using the built-in `DATETIME` function in FTL:
//! ```ftl
//! today-is = Today is { DATETIME($date, month: "long", day: "numeric") }
//! ```
//! In Rust, pass a timestamp (milliseconds since Unix epoch) as a number, and Fluent will format it:
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! let now_seconds = 1712973600i64; // seconds since Unix epoch
//! Label::new(cx, Localized::new("today-is").arg("date", now_seconds * 1000)); // Fluent expects milliseconds
//! ```
//! Or use pre-formatted date strings if you need custom formatting:
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! let formatted_date = "April 13, 2026".to_string();
//! Label::new(cx, Localized::new("event-date").arg("date", formatted_date));
//! ```
//!
//! ### With Chrono
//! For convenient integration with [chrono](https://docs.rs/chrono/), wrap your datetime values with
//! [`FluentDateTime`] or [`FluentNaiveDateTime`] - the conversion to milliseconds is handled automatically:
//! ```ignore
//! # use vizia_core::prelude::*;
//! # use vizia_core::localization::FluentDateTime;
//! # use chrono::Utc;
//! # let mut cx = &mut Context::default();
//! let now = Utc::now();
//! // Automatic conversion to milliseconds happens internally
//! Label::new(cx, Localized::new("event-date").arg("date", FluentDateTime(now)));
//! ```
//! 
//! The wrappers support:
//! - `FluentDateTime<Tz>` - for timezone-aware datetimes like `DateTime<Utc>` or `DateTime<Local>`
//! - `FluentNaiveDateTime` - for naive datetimes (assumes UTC)
use crate::context::LocalizationContext;
use crate::prelude::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use fluent_bundle::FluentArgs;
use fluent_bundle::FluentValue;
use fluent_bundle::types::{FluentNumber, FluentNumberOptions};
use hashbrown::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

/// Helper function for formatting a number with decimal places for localized display.
///
/// # Example
/// ```ignore
/// # use vizia_core::prelude::*;
/// # let mut cx = &mut Context::default();
/// Label::new(cx, Localized::new("price").arg("amount", number_with_fraction(99.99, 2)));
/// ```
pub fn number_with_fraction(value: f64, fraction_digits: usize) -> FluentNumber {
    FluentNumber::new(
        value,
        FluentNumberOptions {
            minimum_fraction_digits: Some(fraction_digits),
            maximum_fraction_digits: Some(fraction_digits),
            ..Default::default()
        },
    )
}

impl Res<FluentNumber> for FluentNumber {
    fn get_value(&self, _: &impl DataContext) -> Self {
        self.clone()
    }
}

/// Helper function for formatting a number as a percentage for localized display.
///
/// # Example
/// ```ignore
/// # use vizia_core::prelude::*;
/// # let mut cx = &mut Context::default();
/// Label::new(cx, Localized::new("completion").arg("percent", percentage(0.75, 1)));
/// ```
pub fn percentage(value: f64, fraction_digits: usize) -> FluentNumber {
    FluentNumber::new(
        value * 100.0,
        FluentNumberOptions {
            minimum_fraction_digits: Some(fraction_digits),
            maximum_fraction_digits: Some(fraction_digits),
            ..Default::default()
        },
    )
}

/// Helper function to convert a chrono DateTime to milliseconds since Unix epoch for Fluent date formatting.
///
/// Fluent's DATETIME function expects milliseconds, not seconds.
///
/// # Example
/// ```ignore
/// # use vizia_core::prelude::*;
/// # use chrono::Utc;
/// # let mut cx = &mut Context::default();
/// let now = Utc::now();
/// Label::new(cx, Localized::new("event-date").arg("date", datetime_to_millis(now)));
/// ```
pub fn datetime_to_millis<Tz: chrono::TimeZone>(dt: DateTime<Tz>) -> i64 {
    dt.with_timezone(&Utc).timestamp_millis()
}

/// Helper function to convert a chrono NaiveDateTime to milliseconds since Unix epoch for Fluent date formatting.
///
/// Note: NaiveDateTime is timezone-unaware. It's recommended to use `datetime_to_millis` with timezone-aware DateTime.
///
/// # Example
/// ```ignore
/// # use vizia_core::prelude::*;
/// # use chrono::Utc;
/// # let mut cx = &mut Context::default();
/// let now = Utc::now().naive_utc();
/// Label::new(cx, Localized::new("event-date").arg("date", naive_datetime_to_millis(now)));
/// ```
pub fn naive_datetime_to_millis(dt: NaiveDateTime) -> i64 {
    dt.and_utc().timestamp_millis()
}

/// Wrapper for chrono DateTime that automatically converts to Fluent's expected millisecond format.
///
/// This allows passing chrono types directly to `arg()` without manual millisecond conversion.
///
/// # Example
/// ```ignore
/// # use vizia_core::prelude::*;
/// # use vizia_core::localization::FluentDateTime;
/// # use chrono::Utc;
/// # let mut cx = &mut Context::default();
/// let now = Utc::now();
/// Label::new(cx, Localized::new("event-date").arg("date", FluentDateTime(now)));
/// ```
#[derive(Clone)]
pub struct FluentDateTime<Tz: chrono::TimeZone + Clone>(pub DateTime<Tz>);

impl<Tz: chrono::TimeZone + Clone> Into<FluentValue<'static>> for FluentDateTime<Tz> {
    fn into(self) -> FluentValue<'static> {
        self.0.with_timezone(&Utc).timestamp_millis().into()
    }
}

impl<Tz: chrono::TimeZone + Clone + 'static> Res<FluentDateTime<Tz>> for FluentDateTime<Tz> {
    fn get_value(&self, _: &impl DataContext) -> Self {
        self.clone()
    }
}

/// Wrapper for chrono NaiveDateTime that automatically converts to Fluent's expected millisecond format.
///
/// This allows passing naive chrono types directly to `arg()` without manual millisecond conversion.
/// Note: assumes UTC timezone for the conversion.
///
/// # Example
/// ```ignore
/// # use vizia_core::prelude::*;
/// # use vizia_core::localization::FluentNaiveDateTime;
/// # use chrono::Utc;
/// # let mut cx = &mut Context::default();
/// let now = Utc::now().naive_utc();
/// Label::new(cx, Localized::new("event-date").arg("date", FluentNaiveDateTime(now)));
/// ```
#[derive(Clone)]
pub struct FluentNaiveDateTime(pub NaiveDateTime);

impl Into<FluentValue<'static>> for FluentNaiveDateTime {
    fn into(self) -> FluentValue<'static> {
        self.0.and_utc().timestamp_millis().into()
    }
}

impl Res<FluentNaiveDateTime> for FluentNaiveDateTime {
    fn get_value(&self, _: &impl DataContext) -> Self {
        self.clone()
    }
}

pub(crate) trait FluentStore {
    fn get_val(&self, cx: &LocalizationContext) -> FluentValue<'static>;
    fn make_clone(&self) -> Box<dyn FluentStore>;
    fn bind(&self, cx: &mut Context, closure: Box<dyn Fn(&mut Context)>);
}

#[derive(Clone)]
pub(crate) struct ResState<R, T> {
    res: R,
    _marker: PhantomData<T>,
}

impl<R, T> FluentStore for ResState<R, T>
where
    R: 'static + Clone + Res<T>,
    T: 'static + Clone + Into<FluentValue<'static>>,
{
    fn get_val(&self, cx: &LocalizationContext) -> FluentValue<'static> {
        self.res.get_value(cx).into()
    }

    fn make_clone(&self) -> Box<dyn FluentStore> {
        Box::new(self.clone())
    }

    fn bind(&self, cx: &mut Context, closure: Box<dyn Fn(&mut Context)>) {
        self.res.clone().set_or_bind(cx, move |cx, _| closure(cx));
    }
}

/// A type which formats a localized message with any number of named arguments.
pub struct Localized {
    key: String,
    attribute: Option<String>,
    args: HashMap<String, Box<dyn FluentStore>>,
    map: Rc<dyn Fn(&str) -> String + 'static>,
}

impl PartialEq for Localized {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.attribute == other.attribute
    }
}

impl Clone for Localized {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            attribute: self.attribute.clone(),
            args: self.args.iter().map(|(k, v)| (k.clone(), v.make_clone())).collect(),
            map: self.map.clone(),
        }
    }
}

impl Localized {
    fn resolve_text(&self, cx: &LocalizationContext) -> String {
        let locale = &cx.environment().locale.get();
        let bundle = cx.resource_manager.current_translation(locale);
        let message = if let Some(msg) = bundle.get_message(&self.key) {
            msg
        } else {
            return (self.map)(&self.key);
        };

        let value = if let Some(attr_name) = &self.attribute {
            // Resolve attribute instead of message value
            if let Some(attr) = message.get_attribute(attr_name) {
                attr.value()
            } else {
                return (self.map)(&format!("{}.{}", &self.key, attr_name));
            }
        } else {
            // Resolve message value
            if let Some(value) = message.value() {
                value
            } else {
                return (self.map)(&self.key);
            }
        };

        let mut err = vec![];
        let args = self.get_args(cx);
        let res = bundle.format_pattern(value, Some(&args), &mut err);

        if err.is_empty() { (self.map)(&res) } else { format!("{} {{ERROR: {:?}}}", res, err) }
    }

    fn get_args(&self, cx: &LocalizationContext) -> FluentArgs {
        let mut res = FluentArgs::new();
        for (name, arg) in &self.args {
            res.set(name.to_owned(), arg.get_val(cx));
        }
        res
    }

    /// Creates a new Localized type with a given key.
    ///
    /// The given key is used to retrieve a translation from a fluent bundle resource.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    ///
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     Label::new(cx, Localized::new("key"));
    /// })
    /// .run();
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_owned(),
            attribute: None,
            args: HashMap::new(),
            map: Rc::new(|s| s.to_string()),
        }
    }

    /// Sets a mapping function to apply to the translated text.
    pub fn map(mut self, mapping: impl Fn(&str) -> String + 'static) -> Self {
        self.map = Rc::new(mapping);

        self
    }

    /// Selects a message attribute to translate instead of the message value.
    ///
    /// Messages can contain multiple attributes that define alternative translations.
    /// This method allows you to select a specific attribute by name.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Resolves the "title" attribute of the "dialog" message
    ///     Label::new(cx, Localized::new("dialog").attribute("title"));
    /// })
    /// .run();
    pub fn attribute(mut self, attr_name: &str) -> Self {
        self.attribute = Some(attr_name.to_owned());
        self
    }
    ///
    /// Takes a key name and a resource for the argument value.
    ///
    /// This accepts both plain values and reactive resources.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    ///
    /// # use vizia_winit::application::Application;
    /// #
    /// # struct AppData {
    /// #   value: i32,
    /// # }
    /// # impl Model for AppData {}
    /// Application::new(|cx|{
    ///     
    ///     AppData {
    ///         value: 5,
    ///     }.build(cx);
    ///
    ///     Label::new(cx, Localized::new("key").arg("value", AppData::value));
    /// })
    /// .run();
    pub fn arg<R, T>(mut self, key: &str, res: R) -> Self
    where
        R: 'static + Clone + Res<T>,
        T: 'static + Clone + Into<FluentValue<'static>>,
    {
        self.args.insert(key.to_owned(), Box::new(ResState { res, _marker: PhantomData }));
        self
    }
}

impl Res<String> for Localized {
    fn get_value(&self, cx: &impl DataContext) -> String {
        let cx = cx.localization_context().expect("Failed to get context");
        self.resolve_text(&cx)
    }

    fn set_or_bind<F>(self, cx: &mut Context, closure: F)
    where
        F: 'static + Fn(&mut Context, Localized),
    {
        let current = cx.current();
        let self2 = self.clone();
        let closure = Arc::new(closure);
        cx.with_current(current, |cx| {
            let stores = self2.args.values().map(|x| x.make_clone()).collect::<Vec<_>>();
            let self3 = self2.clone();
            let closure = closure.clone();
            bind_recursive(cx, &stores, move |cx| {
                closure(cx, self3.clone());
            });
        });
    }
}

fn bind_recursive<F>(cx: &mut Context, stores: &[Box<dyn FluentStore>], closure: F)
where
    F: 'static + Clone + Fn(&mut Context),
{
    if let Some((store, rest)) = stores.split_last() {
        let rest = rest.iter().map(|x| x.make_clone()).collect::<Vec<_>>();
        store.bind(
            cx,
            Box::new(move |cx| {
                bind_recursive(cx, &rest, closure.clone());
            }),
        );
    } else {
        closure(cx);
    }
}

impl<T: ToString> ToStringLocalized for T {
    fn to_string_local(&self, _cx: &impl DataContext) -> String {
        self.to_string()
    }
}

/// A trait for converting from [Localized] to a `String` via a translation using fluent.
pub trait ToStringLocalized {
    /// Method for converting the current type to a `String` via a translation using fluent.
    fn to_string_local(&self, cx: &impl DataContext) -> String;
}

impl ToStringLocalized for Localized {
    fn to_string_local(&self, cx: &impl DataContext) -> String {
        let cx = cx.localization_context().expect("Failed to get context");
        self.resolve_text(&cx)
    }
}
