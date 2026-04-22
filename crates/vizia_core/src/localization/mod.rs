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
//!     langid!("en-US"),
//!     include_str!("resources/en-US/translation.ftl").to_owned(),
//! );
//!
//! ```
//!
//! ## Setting the Locale
//! The application will use the system locale by default, however an environment event can be used to set a custom locale.
//! If no exact translation exists for the specified locale, vizia will negotiate the best available match and then fall back
//! per message to the default translation bundle when needed.
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! // Sets the current locale to en-US, regardless of the system locale
//! cx.emit(EnvironmentEvent::SetLocale(langid!("en-US")));
//! ```
//!
//! ## Diagnostics
//! Missing keys, missing attributes, and Fluent formatting issues are reported through the standard
//! [`log`](https://docs.rs/log) backend at `warn` level.
//! Configure your logger (for example with `env_logger`, `tracing-log`, or `fern`) to surface these messages.
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
//! or a signal, binding the fluent variable to application data and updating when that data changes.
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! Label::new(cx, Localized::new("welcome").arg("user", "Jane"));
//! ```
//!
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! #
//! let user = Signal::new("Jane".to_string());
//!
//! Label::new(cx, Localized::new("welcome").arg("user", user));
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
//! ## Selectors and Plurals
//! Fluent selectors let you choose translations based on a variable value:
//! ```ftl
//! role-label = { $role ->
//!     [admin] You are signed in as an administrator.
//!    *[user] You are signed in as a user.
//! }
//! cart-summary = { $count ->
//!     [one] You have one item in your cart.
//!    *[other] You have { $count } items in your cart.
//! }
//! ```
//! In Rust, pass the selector values with `arg(...)`:
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! Label::new(cx, Localized::new("role-label").arg("role", "admin"));
//! Label::new(cx, Localized::new("cart-summary").arg("count", 3));
//! ```
//!
//! ## Number Formatting
//! Numbers can be formatted in FTL with the built-in `NUMBER` function:
//! ```ftl
//! price = Your total is { NUMBER($amount) }
//! ```
//! In Rust, pass numbers directly as arguments:
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
//! Currency symbols and symbol placement are best handled in translations today.
//! Pre-format the numeric portion in Rust or upstream and let each locale decide where the symbol belongs:
//! ```ftl
//! # en-US
//! price-currency = Price: ${ $amount }
//!
//! # fr
//! price-currency = Prix : { $amount } €
//! ```
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! Label::new(cx, Localized::new("price-currency").arg("amount", "99.99"));
//! ```
//!
//! ## Date Formatting
//! Dates can be formatted with locale-specific rules using the built-in `DATETIME` function in FTL.
//! Pass chrono datetime values directly to `arg()` - the conversion to milliseconds is handled automatically:
//! ```ignore
//! # use vizia_core::prelude::*;
//! # use chrono::Utc;
//! # let mut cx = &mut Context::default();
//! let now = Utc::now();
//! Label::new(cx, Localized::new("event-date").arg("date", now));
//! ```
//!
//! Both timezone-aware and naive datetimes are supported:
//! - Timezone-aware datetimes like `DateTime<Utc>` or `DateTime<Local>` work directly
//! - Naive datetimes are automatically assumed to be in UTC
//!
//! For custom formatting, you can use pre-formatted date strings:
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! let formatted_date = "April 13, 2026".to_string();
//! Label::new(cx, Localized::new("event-date").arg("date", formatted_date));
//! ```
use crate::context::LocalizationContext;
use crate::prelude::*;
use crate::resource::LocalizationIssue;
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

/// Wrapper for chrono DateTime that automatically converts to Fluent's expected millisecond format.
///
/// While this wrapper can be used, chrono DateTime types implement `Res` directly,
/// so you can pass them to `arg()` without wrapping:
///
/// # Example
/// ```ignore
/// # use vizia_core::prelude::*;
/// # use chrono::Utc;
/// # let mut cx = &mut Context::default();
/// let now = Utc::now();
/// // Chrono datetimes work directly with arg()
/// Label::new(cx, Localized::new("event-date").arg("date", now));
/// ```
#[derive(Clone)]
pub struct FluentDateTime<Tz: chrono::TimeZone + Clone>(pub DateTime<Tz>);

impl<Tz: chrono::TimeZone + Clone> From<FluentDateTime<Tz>> for FluentValue<'static> {
    fn from(val: FluentDateTime<Tz>) -> Self {
        let FluentDateTime(datetime) = val;
        datetime.with_timezone(&Utc).timestamp_millis().into()
    }
}

impl<Tz: chrono::TimeZone + Clone + 'static> Res<FluentDateTime<Tz>> for FluentDateTime<Tz> {
    fn get_value(&self, _: &impl DataContext) -> Self {
        self.clone()
    }
}

impl<Tz: chrono::TimeZone + Clone + 'static> Res<FluentDateTime<Tz>> for DateTime<Tz> {
    fn get_value(&self, _: &impl DataContext) -> FluentDateTime<Tz> {
        FluentDateTime(self.clone())
    }
}

/// Wrapper for chrono NaiveDateTime that automatically converts to Fluent's expected millisecond format.
///
/// While this wrapper can be used, chrono NaiveDateTime types implement `Res` directly,
/// so you can pass them to `arg()` without wrapping. Note: assumes UTC timezone for the conversion.
///
/// # Example
/// ```ignore
/// # use vizia_core::prelude::*;
/// # use chrono::Utc;
/// # let mut cx = &mut Context::default();
/// let now = Utc::now().naive_utc();
/// // Naive datetimes work directly with arg() (assumes UTC)
/// Label::new(cx, Localized::new("event-date").arg("date", now));
/// ```
#[derive(Clone)]
pub struct FluentNaiveDateTime(pub NaiveDateTime);

impl From<FluentNaiveDateTime> for FluentValue<'static> {
    fn from(val: FluentNaiveDateTime) -> Self {
        val.0.and_utc().timestamp_millis().into()
    }
}

impl Res<FluentNaiveDateTime> for FluentNaiveDateTime {
    fn get_value(&self, _: &impl DataContext) -> Self {
        self.clone()
    }
}

impl Res<FluentNaiveDateTime> for NaiveDateTime {
    fn get_value(&self, _: &impl DataContext) -> FluentNaiveDateTime {
        FluentNaiveDateTime(*self)
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
        let requested_locale = cx.environment().locale.get();
        let args = self.get_args(cx);
        let mut saw_message = false;

        for locale in cx.resource_manager.translation_locales(&requested_locale) {
            let bundle = cx.resource_manager.current_translation(&locale);
            let Some(message) = bundle.get_message(&self.key) else {
                continue;
            };
            saw_message = true;

            let value = if let Some(attr_name) = &self.attribute {
                if let Some(attr) = message.get_attribute(attr_name) {
                    attr.value()
                } else {
                    continue;
                }
            } else if let Some(value) = message.value() {
                value
            } else {
                continue;
            };

            let mut err = vec![];
            let res = bundle.format_pattern(value, Some(&args), &mut err);

            if !err.is_empty() {
                cx.resource_manager.report_localization_issue(LocalizationIssue::FormatError {
                    key: self.key.clone(),
                    locale: locale.to_string(),
                    details: format!("{:?}", err),
                });
            }

            return (self.map)(&res);
        }

        if let Some(attr_name) = &self.attribute {
            if saw_message {
                cx.resource_manager.report_localization_issue(
                    LocalizationIssue::MissingAttribute {
                        key: self.key.clone(),
                        attribute: attr_name.clone(),
                        requested_locale: requested_locale.to_string(),
                    },
                );
            } else {
                cx.resource_manager.report_localization_issue(LocalizationIssue::MissingMessage {
                    key: self.key.clone(),
                    requested_locale: requested_locale.to_string(),
                });
            }
            (self.map)(&format!("{}.{}", &self.key, attr_name))
        } else {
            cx.resource_manager.report_localization_issue(LocalizationIssue::MissingMessage {
                key: self.key.clone(),
                requested_locale: requested_locale.to_string(),
            });
            (self.map)(&self.key)
        }
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
        bind_localized_updates(cx, self, Arc::new(closure));
    }

    fn to_signal(self, cx: &mut Context) -> Signal<String> {
        let signal = Signal::new(self.get_value(cx));

        bind_localized_updates(
            cx,
            self,
            Arc::new(move |cx, localized| {
                signal.set(localized.get_value(cx));
            }),
        );

        signal
    }
}

fn bind_localized_updates(
    cx: &mut Context,
    localized: Localized,
    closure: Arc<dyn Fn(&mut Context, Localized)>,
) {
    let current = cx.current();
    let localized_for_scope = localized.clone();
    cx.with_current(current, move |cx| {
        let stores = localized_for_scope.args.values().map(|x| x.make_clone()).collect::<Vec<_>>();
        let localized_for_bind = localized_for_scope.clone();
        let closure_for_bind = closure.clone();
        bind_recursive(cx, &stores, move |cx| {
            let localized_for_locale = localized_for_bind.clone();
            let closure_for_locale = closure_for_bind.clone();
            cx.environment().locale.set_or_bind(cx, move |cx, _| {
                closure_for_locale(cx, localized_for_locale.clone());
            });
        });
    });
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_message_falls_back_to_key() {
        let cx = Context::default();
        cx.data::<Environment>().locale.set("en-US".parse().unwrap());

        let text = Localized::new("missing-key").to_string_local(&cx);

        assert_eq!(text, "missing-key");
    }

    #[test]
    fn missing_attribute_falls_back_to_key_attribute() {
        let mut cx = Context::default();
        cx.data::<Environment>().locale.set("en-US".parse().unwrap());
        cx.add_translation("en-US".parse().unwrap(), "dialog = File Dialog".to_string()).unwrap();

        let text = Localized::new("dialog").attribute("title").to_string_local(&cx);

        assert_eq!(text, "dialog.title");
    }

    #[test]
    fn format_error_returns_partial_resolved_text() {
        let mut cx = Context::default();
        cx.data::<Environment>().locale.set("en-US".parse().unwrap());
        cx.add_translation("en-US".parse().unwrap(), "welcome = Welcome, { $name }!".to_string())
            .unwrap();

        let text = Localized::new("welcome").to_string_local(&cx);
        assert!(text.contains("Welcome"));
        assert!(text.contains("$name"));
    }

    #[test]
    fn falls_back_to_default_bundle_per_key() {
        let mut cx = Context::default();
        cx.data::<Environment>().locale.set("fr".parse().unwrap());

        // Ensure the requested locale exists but does not contain the requested key.
        cx.add_translation("fr".parse().unwrap(), "bonjour = Bonjour".to_string()).unwrap();

        // Provide the requested key only in the default bundle.
        cx.add_translation(
            LanguageIdentifier::default(),
            "greeting = Hello from default".to_string(),
        )
        .unwrap();

        let text = Localized::new("greeting").to_string_local(&cx);

        assert_eq!(text, "Hello from default");
    }

    #[test]
    fn falls_back_to_default_bundle_for_attribute_when_message_exists_in_requested_locale() {
        let mut cx = Context::default();
        cx.data::<Environment>().locale.set("fr".parse().unwrap());

        // Requested locale has the message but not the attribute.
        cx.add_translation("fr".parse().unwrap(), "dialog = Dialogue".to_string()).unwrap();

        // Default locale provides the missing attribute.
        cx.add_translation(
            LanguageIdentifier::default(),
            "dialog = Dialog\n    .title = Default Title".to_string(),
        )
        .unwrap();

        let text = Localized::new("dialog").attribute("title").to_string_local(&cx);

        assert_eq!(text, "Default Title");
    }
}
