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
//! The [`Localized`] type provides two methods for referencing a variable. The `arg_const(...)` method allows a keyed value to be inserted into the translation.
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! Label::new(cx, Localized::new("welcome").arg_const("user", "Jane"));
//! ```
//! While the `arg(...)` method allows any keyed signal (value or signal) to be used,
//! binding the fluent variable to application data and updating when that data changes.
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
use crate::context::LocalizationContext;
use crate::prelude::*;
use fluent_bundle::FluentArgs;
use fluent_bundle::FluentValue;
use hashbrown::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

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

#[derive(Clone)]
pub(crate) struct ValState<T> {
    val: T,
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

impl<T> FluentStore for ValState<T>
where
    T: 'static + Clone + Into<FluentValue<'static>>,
{
    fn get_val(&self, _cx: &LocalizationContext) -> FluentValue<'static> {
        self.val.clone().into()
    }

    fn make_clone(&self) -> Box<dyn FluentStore> {
        Box::new(self.clone())
    }

    fn bind(&self, cx: &mut Context, closure: Box<dyn Fn(&mut Context)>) {
        closure(cx);
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
    /// Takes a key name and a signal for the argument value (value or signal).
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

    /// Add a constant argument to the Localized type.
    ///
    /// Takes a key name and a value for the argument.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///
    ///     Label::new(cx, Localized::new("key").arg_const("value", 32));
    /// })
    /// .run();
    pub fn arg_const<T: Into<FluentValue<'static>> + Clone + 'static>(
        mut self,
        key: &str,
        val: T,
    ) -> Self {
        self.args.insert(key.to_owned(), Box::new(ValState { val }));
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
