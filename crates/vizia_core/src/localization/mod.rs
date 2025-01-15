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
//! While the `arg(...)` method allows a keyed lens to be used, binding the fluent variable to a piece of application data, and updating when that data changes.
//! ```ignore
//! # use vizia_core::prelude::*;
//! # let mut cx = &mut Context::default();
//! # #[derive(Lens)]
//! # pub struct AppData {
//! #   user: String,
//! # }
//! Label::new(cx, Localized::new("welcome").arg("user", AppData::user));
//! ```
use crate::context::LocalizationContext;
use crate::prelude::*;
use fluent_bundle::FluentArgs;
use fluent_bundle::FluentValue;
use hashbrown::HashMap;
use std::rc::Rc;
use std::sync::Arc;

pub(crate) trait FluentStore {
    fn get_val(&self, cx: &LocalizationContext) -> FluentValue<'static>;
    fn make_clone(&self) -> Box<dyn FluentStore>;
    fn bind(&self, cx: &mut Context, closure: Box<dyn Fn(&mut Context)>);
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct LensState<L> {
    lens: L,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct ValState<T> {
    val: T,
}

impl<L> FluentStore for LensState<L>
where
    L: Lens<Target: Into<FluentValue<'static>> + Data>,
{
    fn get_val(&self, cx: &LocalizationContext) -> FluentValue<'static> {
        self.lens
            .view(
                cx.data()
                    .expect("Failed to get data from context. Has it been built into the tree?"),
            )
            .unwrap()
            .into_owned()
            .into()
    }

    fn make_clone(&self) -> Box<dyn FluentStore> {
        Box::new(*self)
    }

    fn bind(&self, cx: &mut Context, closure: Box<dyn Fn(&mut Context)>) {
        Binding::new(cx, self.lens, move |cx, _| closure(cx));
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
    args: HashMap<String, Box<dyn FluentStore>>,
    map: Rc<dyn Fn(&str) -> String + 'static>,
}

impl PartialEq for Localized {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Clone for Localized {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            args: self.args.iter().map(|(k, v)| (k.clone(), v.make_clone())).collect(),
            map: self.map.clone(),
        }
    }
}

impl Localized {
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
    /// # use vizia_derive::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     Label::new(cx, Localized::new("key"));
    /// })
    /// .run();
    pub fn new(key: &str) -> Self {
        Self { key: key.to_owned(), args: HashMap::new(), map: Rc::new(|s| s.to_string()) }
    }

    /// Sets a mapping function to apply to the translated text.
    pub fn map(mut self, mapping: impl Fn(&str) -> String + 'static) -> Self {
        self.map = Rc::new(mapping);

        self
    }

    /// Add a variable argument binding to the Localized type.
    ///
    /// Takes a key name and a lens to the value for the argument.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_derive::*;
    /// # use vizia_winit::application::Application;
    /// # #[derive(Lens)]
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
    pub fn arg<L>(mut self, key: &str, lens: L) -> Self
    where
        L: Lens<Target: Into<FluentValue<'static>> + Data>,
    {
        self.args.insert(key.to_owned(), Box::new(LensState { lens }));
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
    pub fn arg_const<T: Into<FluentValue<'static>> + Data>(mut self, key: &str, val: T) -> Self {
        self.args.insert(key.to_owned(), Box::new(ValState { val }));
        self
    }
}

impl ResGet<String> for Localized {
    fn get_ref<'a>(&'a self, cx: &'a impl DataContext) -> Option<LensValue<'a, String>> {
        Some(LensValue::Owned(self.get(cx)))
    }

    fn get(&self, cx: &impl DataContext) -> String {
        let cx = cx.localization_context().expect("Failed to get context");
        let locale = &cx.environment().locale;
        let bundle = cx.resource_manager.current_translation(locale);
        let message = if let Some(msg) = bundle.get_message(&self.key) {
            msg
        } else {
            return (self.map)(&self.key);
        };

        let value = if let Some(value) = message.value() {
            value
        } else {
            return (self.map)(&self.key);
        };

        let mut err = vec![];
        let args = self.get_args(&cx);
        let res = bundle.format_pattern(value, Some(&args), &mut err);

        if err.is_empty() {
            (self.map)(&res)
        } else {
            format!("{} {{ERROR: {:?}}}", res, err)
        }
    }
}

impl Res<String> for Localized {
    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Localized),
    {
        let self2 = self.clone();
        let closure = Arc::new(closure);
        Binding::new(cx, Environment::locale, move |cx, _| {
            cx.with_current(entity, |cx| {
                let lenses = self2.args.values().map(|x| x.make_clone()).collect::<Vec<_>>();
                let self3 = self2.clone();
                let closure = closure.clone();
                bind_recursive(cx, &lenses, move |cx| {
                    closure(cx, self3.clone());
                });
            });
        });
    }
}

fn bind_recursive<F>(cx: &mut Context, lenses: &[Box<dyn FluentStore>], closure: F)
where
    F: 'static + Clone + Fn(&mut Context),
{
    if let Some((lens, rest)) = lenses.split_last() {
        let rest = rest.iter().map(|x| x.make_clone()).collect::<Vec<_>>();
        lens.bind(
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

        let locale = &cx.environment().locale;
        let bundle = cx.resource_manager.current_translation(locale);
        let message = if let Some(msg) = bundle.get_message(&self.key) {
            msg
        } else {
            // Warn here of missing key
            return (self.map)(&self.key);
        };

        let value = if let Some(value) = message.value() {
            value
        } else {
            // Warn here of missing value
            return (self.map)(&self.key);
        };

        let mut err = vec![];
        let args = self.get_args(&cx);
        let res = bundle.format_pattern(value, Some(&args), &mut err);

        if err.is_empty() {
            (self.map)(&res)
        } else {
            format!("{} {{ERROR: {:?}}}", res, err)
        }
    }
}
